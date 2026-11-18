// Record links in T3's conversation: an item or mission ID becomes a link with a hover card, with
// no change to the text T3 rendered. Ranges (a CSS highlight) mark the IDs, and transparent
// keyboard links in the extension's own root sit over them, so React's message text is untouched.
// The links sit over DOM T3 owns, so they are DOM code; the card is React (HoverCard.tsx).
import type { ItemPage } from "../../../desk/src/api/ItemPage";
import type { MissionPage } from "../../../desk/src/api/MissionPage";
import type { RecordName } from "../../../desk/src/api/RecordName";
import { element } from "../core/dom";
import type { Extension } from "../core/extension";
import { deskGet, type RequestError } from "../host/api";
import { hostSelectors } from "../host/adapter";
import type { DeskPanel, DeskRecord } from "../panel/panel";
import { mountReact } from "../react/mount";
import { recordPath, repositoryKey } from "../repositories";
import { deskAvailable } from "../views";
import { HoverCard } from "./HoverCard";

/** A record ID found in the conversation, and the link laid over it. */
export type Reference = {
  /** The repository its slug names, until the Desk that answered says which it is in. */
  repo: string;
  kind: "item" | "mission";
  id: string;
  /** The Desk that answered for it, once resolved. */
  desk?: string;
  /** The text node it is in, and where. */
  node: Text;
  offset: number;
  range: Range;
  rects: DOMRect[];
  anchor: HTMLAnchorElement;
};
/** A record as `GET /api/records/{id}` names it, with the Desk that answered. */
export type ResolvedRecord = RecordName & { desk: string };
/** A record's own page, as the card reads its further lines from it. */
export type RecordDetails = { kind: "item"; page: ItemPage } | { kind: "mission"; page: MissionPage };

export type References = {
  /** Scans the conversation 250 ms after it stops changing, and at least once a second while it changes. */
  queueScan(): void;
  /** Builds the ID pattern from the repositories' slugs. */
  buildPattern(): void;
};

export function createReferences(ext: Extension, panel: DeskPanel): References {
  const { api, repositories, settings } = ext;
  const style = element("style", { text: "::highlight(t3-atelier-refs) { color: var(--primary, #8ab4ff); text-decoration: underline; }" });
  document.head.append(style);
  const layer = element("div", { id: "reference-links" });
  api.root.append(layer);

  let references: Reference[] = [];
  let pattern: RegExp | null = null;
  // A reference is a repository's slug, a dash and a lowercase-letter/digit suffix. The slug is the
  // Desk's repository name unless config.json's desk.repositories gives another.
  function buildPattern() {
    const escape = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const slugs = [...new Set(repositories.list.map((repo) => repo.slug))].sort((a, b) => b.length - a.length);
    pattern = slugs.length ? new RegExp(`(?<![\\w-])(?:${slugs.map(escape).join("|")})-[a-z0-9]+(?![\\w-])`, "g") : null;
  }
  // A reference's address before it is resolved names the repository its slug belongs to.
  const referenceUrl = (ref: Reference) => (ref.desk ?? repositories.find(ref.repo)?.desk ?? ext.deskUrls[0]) + recordPath(ref);

  // One read resolves an ID: GET /api/records/{id} names its repository and kind. Each Desk is asked
  // in the configured order until one has it. Cached for 30 seconds.
  const cache = new Map<string, { at: number; promise: Promise<unknown> }>();
  function cached<T>(key: string, read: () => Promise<T>): Promise<T> {
    let entry = cache.get(key);
    if (!entry || Date.now() - entry.at > 30000) {
      entry = { at: Date.now(), promise: read() };
      cache.set(key, entry);
    }
    return entry.promise as Promise<T>;
  }
  const readRecord = (id: string) =>
    cached(`record:${id}`, async (): Promise<ResolvedRecord> => {
      for (const desk of ext.deskUrls) {
        try {
          return { ...(await deskGet<RecordName>(api, desk, `/api/records/${id}`)), desk };
        } catch (error) {
          if ((error as RequestError).status !== 404) throw error;
        }
      }
      throw Object.assign(new Error(`No item or mission ${id} was found on ${ext.deskUrls.join(", ")}.`), { status: 404 });
    });
  // The card's further lines (priority, holder, checks, blockers, goal) come from the record's own route.
  const readDetails = (record: ResolvedRecord) =>
    cached(`details:${record.desk}${recordPath(record)}`, async (): Promise<RecordDetails> => {
      const path = `/api/repos/${encodeURIComponent(record.repo)}/${record.kind === "mission" ? "missions" : "items"}/${record.id}`;
      return record.kind === "mission" ? { kind: "mission", page: await deskGet<MissionPage>(api, record.desk, path) } : { kind: "item", page: await deskGet<ItemPage>(api, record.desk, path) };
    });
  async function resolve(ref: Reference): Promise<ResolvedRecord> {
    const record = await readRecord(ref.id);
    Object.assign(ref, { repo: record.repo, kind: record.kind, desk: record.desk });
    ref.anchor.href = referenceUrl(ref);
    return record;
  }
  function forget(ref: Reference) {
    for (const key of cache.keys()) if (key === `record:${ref.id}` || key.endsWith(`/${ref.id}`)) cache.delete(key);
  }

  // The card: which reference it shows, drawn in the extension's root by React.
  const card = mountReact(api.root, api);
  let active: Reference | null = null;
  // Each showing of the card, so a Refresh or Retry reads again.
  let showing = 0;
  let cardElement: HTMLElement | null = null;
  let pointerOverCard = false;
  let hoverTimer: ReturnType<typeof setTimeout> | undefined;
  let dismissTimer: ReturnType<typeof setTimeout> | undefined;
  function drawCard() {
    card.render(
      active ? (
        <HoverCard
          key={`${active.id}:${showing}`}
          reference={active}
          resolve={resolve}
          readDetails={readDetails}
          externalUrl={referenceUrl}
          onOpen={(ref) => void activate(ref)}
          onReload={(ref) => {
            forget(ref);
            showCard(ref);
          }}
          onClose={hideCard}
          onPointer={(over) => {
            pointerOverCard = over;
            if (over) clearTimeout(dismissTimer);
            else dismissSoon();
          }}
          popupRef={(node) => {
            cardElement = node;
          }}
        />
      ) : null,
    );
  }
  function showCard(ref: Reference) {
    clearTimeout(dismissTimer);
    active = ref;
    showing++;
    drawCard();
  }
  function hideCard() {
    clearTimeout(hoverTimer);
    clearTimeout(dismissTimer);
    pointerOverCard = false;
    if (!active) return;
    active = null;
    drawCard();
  }
  function dismissSoon() {
    clearTimeout(dismissTimer);
    dismissTimer = setTimeout(() => {
      if (!pointerOverCard && !(cardElement && api.root.activeElement && cardElement.contains(api.root.activeElement))) hideCard();
    }, 200);
  }
  ext.listen(document, "keydown", (event) => {
    if (event.key === "Escape") hideCard();
  });

  // Opens a record in the Desk panel, or in the browser when the panel cannot show it or the human
  // asked for that (Ctrl or Cmd); a record that cannot be resolved shows its card with the reason.
  async function activate(ref: Reference, external = false) {
    try {
      await resolve(ref);
      if (api.signal.aborted || !ref.desk) return;
      hideCard();
      const record: DeskRecord = { repo: ref.repo, kind: ref.kind, id: ref.id, desk: ref.desk };
      if (external || !deskAvailable(ext.host) || !repositories.find(repositoryKey(record))) window.open(referenceUrl(ref), "_blank", "noopener");
      else await panel.show(record);
    } catch {
      if (!api.signal.aborted) showCard(ref);
    }
  }

  function updateRects() {
    for (const ref of references) {
      ref.rects = [...ref.range.getClientRects()].filter((rect) => rect.width && rect.height && rect.bottom >= 0 && rect.top < innerHeight);
      const rect = ref.rects[0];
      ref.anchor.hidden = !rect;
      if (rect) ref.anchor.style.cssText = `position:fixed;left:${rect.left}px;top:${rect.top}px;width:${rect.width}px;height:${rect.height}px;pointer-events:none;z-index:30;border-radius:2px`;
    }
  }
  let rectFrame = 0;
  function queueRects() {
    if (!rectFrame)
      rectFrame = requestAnimationFrame(() => {
        rectFrame = 0;
        updateRects();
      });
  }

  // The thread whose conversation T3 has rendered. T3 marks each assistant message with its
  // environment and thread; a conversation with none is still the thread's last scanned while its
  // root and first row nodes are the ones that scan read, and the current thread's once either changed.
  let scanned: { thread: string; root: Element; firstRow: Element | null } | null = null;
  const currentKey = () => (ext.current ? `${ext.current.environmentId}:${ext.current.threadId}` : "");
  function conversationOwner(conversation: Element): string {
    const marked = ext.host?.conversationThread(conversation);
    if (marked) return marked;
    const firstRow = conversation.querySelector(hostSelectors.timelineRow);
    if (scanned && scanned.root === conversation && scanned.firstRow === firstRow) return scanned.thread;
    return currentKey();
  }
  function scan() {
    if (api.signal.aborted) return;
    const previous = references;
    references = [];
    CSS.highlights?.delete("t3-atelier-refs");
    const conversation = document.querySelector(hostSelectors.conversation);
    if (!settings.references || !pattern || !conversation) {
      hideCard();
      layer.replaceChildren();
      panel.drawThread();
      return;
    }
    const existing = new Map<Node, Map<string, Reference>>();
    for (const ref of previous) {
      if (!existing.has(ref.node)) existing.set(ref.node, new Map());
      existing.get(ref.node)?.set(`${ref.offset}:${ref.id}`, ref);
    }
    const walker = document.createTreeWalker(conversation, NodeFilter.SHOW_TEXT, {
      acceptNode(node) {
        return node.parentElement?.closest('pre,a,button,input,textarea,script,style,svg,[contenteditable="true"]') ? NodeFilter.FILTER_REJECT : NodeFilter.FILTER_ACCEPT;
      },
    });
    const ranges: Range[] = [];
    while (walker.nextNode()) {
      const node = walker.currentNode as Text;
      pattern.lastIndex = 0;
      for (const match of (node.textContent ?? "").matchAll(pattern)) {
        const slug = match[0].slice(0, match[0].lastIndexOf("-"));
        const repo = repositories.list.find((repo) => repo.slug === slug);
        if (!repo) continue;
        const range = document.createRange();
        range.setStart(node, match.index);
        range.setEnd(node, match.index + match[0].length);
        ranges.push(range);
        const retained = existing.get(node)?.get(`${match.index}:${match[0]}`);
        if (retained) {
          retained.range = range;
          references.push(retained);
          continue;
        }
        const anchor = element("a", { "aria-haspopup": "dialog", "data-atelier-id": match[0] });
        const ref: Reference = { repo: repo.repo, id: match[0], kind: "item", node, offset: match.index, range, rects: [], anchor };
        anchor.href = referenceUrl(ref);
        anchor.setAttribute("aria-label", `Preview Atelier reference ${ref.id}`);
        references.push(ref);
        layer.append(anchor);
        anchor.addEventListener("focus", () => showCard(ref));
        anchor.addEventListener("blur", dismissSoon);
        anchor.addEventListener("click", (event) => {
          event.preventDefault();
          void activate(ref, event.metaKey || event.ctrlKey);
        });
      }
    }
    const kept = new Set(references);
    for (const ref of previous) {
      if (kept.has(ref)) continue;
      if (active === ref) hideCard();
      ref.anchor.remove();
    }
    if (CSS.highlights) CSS.highlights.set("t3-atelier-refs", new Highlight(...ranges));
    updateRects();
    // The IDs are credited only to the thread whose conversation was read: after a switch T3 may
    // still show the previous thread's conversation for a while.
    const owner = conversationOwner(conversation);
    if (ext.current && owner === currentKey()) {
      scanned = { thread: owner, root: conversation, firstRow: conversation.querySelector(hostSelectors.timelineRow) };
      panel.noteMentions(owner, [...new Set(references.map((ref) => ref.id))].sort());
    }
  }
  // A scan runs 250 ms after the conversation stops changing, and at least once a second while it
  // keeps changing, so a streaming thread is scanned too.
  let scanTimer: ReturnType<typeof setTimeout> | undefined;
  let scanDue = 0;
  function queueScan() {
    clearTimeout(scanTimer);
    const now = Date.now();
    if (!scanDue) scanDue = now + 1000;
    scanTimer = setTimeout(
      () => {
        scanDue = 0;
        scan();
      },
      Math.max(0, Math.min(250, scanDue - now)),
    );
  }

  // The links take no pointer events, so T3's text stays selectable: a pointer over an ID's
  // rectangles opens its card after 220 ms, and a plain click on one (with no text selected)
  // opens the record.
  const hit = (event: MouseEvent) => references.find((ref) => ref.rects.some((rect) => event.clientX >= rect.left && event.clientX <= rect.right && event.clientY >= rect.top && event.clientY <= rect.bottom));
  const inConversation = (event: Event) => event.target instanceof Element && Boolean(event.target.closest(hostSelectors.conversation));
  let pointerReference: Reference | null = null;
  ext.listen(document, "pointermove", (event) => {
    const ref = inConversation(event) ? (hit(event) ?? null) : null;
    if (ref === pointerReference) return;
    pointerReference = ref;
    clearTimeout(hoverTimer);
    if (ref) {
      clearTimeout(dismissTimer);
      hoverTimer = setTimeout(() => showCard(ref), 220);
    } else dismissSoon();
  });
  ext.listen(
    document,
    "click",
    (event) => {
      const ref = inConversation(event) ? hit(event) : null;
      if (!ref || event.button !== 0 || !document.getSelection()?.isCollapsed) return;
      event.preventDefault();
      event.stopPropagation();
      void activate(ref, event.metaKey || event.ctrlKey);
    },
    { capture: true },
  );
  ext.listen(document, "scroll", queueRects, { capture: true });
  ext.listen(window, "resize", queueRects);
  ext.listen(document, "visibilitychange", queueRects);
  ext.onContext(() => queueScan());
  ext.onMutation((records) => {
    if (records.some((record) => (record.target.nodeType === Node.ELEMENT_NODE ? (record.target as Element) : record.target.parentElement)?.closest(hostSelectors.conversation))) queueScan();
  });
  ext.onDispose(() => {
    clearTimeout(scanTimer);
    clearTimeout(hoverTimer);
    clearTimeout(dismissTimer);
    cancelAnimationFrame(rectFrame);
    // The card's React root unmounts with the extension (mountReact), possibly before this runs, so
    // the card is dropped without drawing again.
    active = null;
    layer.remove();
    style.remove();
    CSS.highlights?.delete("t3-atelier-refs");
  });
  return { queueScan, buildPattern };
}
