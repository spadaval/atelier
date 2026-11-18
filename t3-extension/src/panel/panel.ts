// Atelier in T3's workspace panel, laid over T3's panel content, with the Desk tab (tab.ts). It
// shows This thread for the thread T3 shows, drawn by the extension in React (src/thread), or a
// Desk page the human opened, such as an item, a mission's record page or the inbox, in a frame of
// the Desk's embedded mode with a Back button to This thread. A framed page stays where it is when
// T3 shows another thread. Desk remains the backend; no T3 credentials enter its iframe, which
// talks to this page over the Desk's host contract (atelier: docs/architecture/desk.md, "The
// embedded mode" and "The host contract"; the Desk's side is desk/src/lib/host.ts).
import { element, icons } from "../core/dom";
import type { Extension } from "../core/extension";
import { hostSelectors, observeHostDom, observeHostTheme, readHostTheme, type ThreadContext } from "../host/adapter";
import { recordPath, repositoryKey, repositoryPath, type RecordRef } from "../repositories";
import { mountThisThread } from "../thread/mount";
import type { ThisThreadProps } from "../thread/ThisThread";
import { deskAvailable } from "../views";
import { createDeskTab } from "./tab";

/** A message the Desk sends its host (DeskMessage in desk/src/lib/host.ts). */
type DeskMessage =
  | { type: "desk:ready"; version: 1 }
  | { type: "desk:navigated"; path: string; title: string }
  | { type: "desk:open-thread"; thread: string }
  | { type: "desk:resized"; height: number }
  | { type: "desk:key"; key: "Escape" };

/** A record a link opened, on the Desk that answered for it. */
export type DeskRecord = RecordRef & { desk: string };

export type DeskPanel = {
  /** Opens the panel on the thread T3 shows, at `record`, framed, when one is given. */
  show(record?: DeskRecord): Promise<void>;
  /** Goes to a thread the Desk or a page names, or says why it cannot. */
  openThread(threadId: string): void;
  /** Notes the IDs the conversation of thread `key` (`<environment>:<thread>`) names, for This thread. */
  noteMentions(key: string, ids: string[]): void;
  /** Draws This thread again with what is known of the thread T3 shows, when that changed. */
  drawThread(): void;
  /** Once T3's hooks are found: a change of T3's own panel surface closes the Desk. */
  watchPanel(): void;
};

/** `data` as one of the Desk's messages, as far as a host acts on it, or null. */
function deskMessage(data: unknown): DeskMessage | null {
  if (!data || typeof data !== "object") return null;
  const message = data as Record<string, unknown>;
  if (message.type === "desk:ready") return { type: "desk:ready", version: 1 };
  if (message.type === "desk:navigated" && typeof message.path === "string" && message.path.startsWith("/")) return { type: "desk:navigated", path: message.path, title: String(message.title ?? "") };
  if (message.type === "desk:open-thread" && typeof message.thread === "string") return { type: "desk:open-thread", thread: message.thread };
  if (message.type === "desk:key" && message.key === "Escape") return { type: "desk:key", key: "Escape" };
  return null;
}

const threadKey = (context: ThreadContext | null) => (context ? `${context.environmentId}:${context.threadId}` : "");

/** What the panel needs from the views it opens outside itself. */
export type PanelLinks = { openSettings(): void; openMission(desk: string, repo: string, id: string): void };

export function createDeskPanel(ext: Extension, links: PanelLinks): DeskPanel {
  const { openSettings } = links;
  const { api, repositories, settings } = ext;
  const base = `
  :host { color: var(--foreground, #eee); font: 13px var(--font-sans, system-ui, sans-serif); }
  * { box-sizing: border-box; } [hidden] { display: none !important; }
  button { font: inherit; color: inherit; cursor: pointer; background: var(--secondary, #303238); border: 1px solid var(--border, #60636a); border-radius: 6px; padding: 6px 10px; }
  button:hover { background: var(--accent, #41444b); }
  button:focus-visible { outline: 2px solid var(--ring, #86b5ff); outline-offset: 2px; }
  h3 { margin: 0; font-size: 14px; } p { margin: 0; line-height: 1.5; } .muted { color: var(--muted-foreground, #b1b4bc); }
  .stack { display: flex; flex-direction: column; gap: 12px; }
  .error { color: var(--destructive, #ff9292); white-space: pre-wrap; overflow-wrap: anywhere; }
  :host{display:block;height:100%}.panel{height:100%;min-height:0;display:flex;flex-direction:column;background:var(--background,#202124)}iframe,.native{flex:1;min-height:0;width:100%;border:0;background:var(--background,#202124)}.empty{padding:20px;max-width:55ch}.notice,.panel>.error{padding:6px 10px;font-size:12px;border-bottom:1px solid var(--border,#60636a)}
  .back{display:flex;align-items:center;gap:8px;padding:4px 8px;border-bottom:1px solid var(--border,#60636a);font-size:12px}.back button{display:inline-flex;align-items:center;gap:2px;height:24px;padding:0 8px 0 4px;border:0;background:transparent;color:var(--muted-foreground,#b1b4bc)}.back button:hover{background:var(--accent,#41444b);color:var(--foreground,#eee)}.back svg{width:14px;height:14px}.back span{min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;color:var(--muted-foreground,#b1b4bc)}`;
  // While the Desk is shown, T3's own content in the slot stays in place, hidden and inert.
  const globalStyle = element("style", { text: "[data-t3-desk-selected] > :not([data-t3-desk-mount]) { visibility: hidden !important; }" });
  document.head.append(globalStyle);

  const mount = element("div", { "data-t3-desk-mount": "", hidden: "" });
  mount.style.cssText = "position:absolute;inset:0;z-index:1;min-width:0;min-height:0";
  const root = mount.attachShadow({ mode: "open" });
  root.append(element("style", { text: base }));
  const notice = element("p", { class: "notice muted", role: "status", hidden: "" });
  const panelError = element("p", { class: "error", hidden: "" });
  const iframe = element("iframe", { title: "Atelier Desk" });
  const linkProject = element("button", { text: "Link project in Customizations" });
  const empty = element("div", { class: "empty stack" }, element("h3", { text: "No Atelier repository linked" }), element("p", { class: "muted", text: "No Desk serves a repository with this project's identity. Choose one for this T3 project in Customizations." }), linkProject);
  // A framed page has a bar above it with Back, which returns to This thread, and the page's title.
  const backButton = element("button", { "data-t3-desk-back": "", title: "Back to This thread", html: `${icons.back}This thread` });
  const backTitle = element("span");
  const backBar = element("div", { class: "back", hidden: "" }, backButton, backTitle);
  // This thread, drawn by the extension, in its own shadow root with T3's components.
  const native = element("div", { class: "native", "data-t3-this-thread": "" });
  const thisThread = mountThisThread(native, api);
  root.append(element("div", { class: "panel" }, notice, panelError, empty, backBar, native, iframe));
  let noticeTimer: ReturnType<typeof setTimeout> | undefined;
  function notify(text: string) {
    notice.textContent = text;
    notice.hidden = false;
    clearTimeout(noticeTimer);
    noticeTimer = setTimeout(() => {
      notice.hidden = true;
    }, 8000);
  }

  // What the panel does to T3's nodes while the Desk is shown, so it can put each back.
  const nativeInert = new Map<HTMLElement, boolean>();
  const positions = new Map<HTMLElement, string>();
  const areaLabels = new Map<Element, string | null>();
  const markedTabbars = new Set<Element>();
  let surfaceContent: HTMLElement | null = null;
  // Whether the human has the Desk open on this thread, and the surface T3's panel showed then: a
  // change of T3's surface closes the Desk.
  let desiredOpen = false;
  let nativeActiveId: string | null = null;
  // The Desk page the human opened, framed, while it is shown instead of This thread: its Desk, its
  // route without /embed (from desk:navigated once it is ready), and its page title.
  let framed = false;
  let frameDesk: string | null = null;
  let framePath: string | null = null;
  let deskTitle = "";
  let deskReady = false;
  function restoreNative() {
    for (const [node, inert] of nativeInert) node.inert = inert;
    nativeInert.clear();
    surfaceContent?.removeAttribute("data-t3-desk-selected");
    for (const tabbar of markedTabbars) tabbar.removeAttribute("data-t3-desk-tabbar");
    markedTabbars.clear();
    for (const [area, label] of areaLabels) label === null ? area.removeAttribute("aria-label") : area.setAttribute("aria-label", label);
    areaLabels.clear();
  }
  // A page the human opened loads in the embedded mode; Open externally keeps the normal address.
  function loadFrame(desk: string, path: string) {
    framed = true;
    frameDesk = desk;
    framePath = path;
    deskReady = false;
    deskTitle = "";
    sentTheme = "";
    iframe.src = `${desk}/embed${path}`;
    ext.scheduleDOM();
  }
  function back() {
    framed = false;
    frameDesk = framePath = null;
    deskReady = false;
    deskTitle = "";
    iframe.removeAttribute("src");
    ext.scheduleDOM();
  }
  ext.listen(backButton, "click", back);
  async function show(record?: DeskRecord) {
    if (!settings.enabled) {
      settings.enabled = true;
      void ext.saveSettings();
    }
    const current = ext.current;
    if (!current || !deskAvailable(ext.host) || !ext.host?.panel) {
      openSettings();
      return;
    }
    if (record) {
      // The record opens on the Desk that answered for it.
      repositories.override = { projectKey: `${current.environmentId}:${current.projectId}`, repository: repositoryKey(record) };
      loadFrame(record.desk, recordPath(record));
    }
    desiredOpen = true;
    ext.view.openThreads[threadKey(current)] = true;
    ext.saveView();
    if (location.hash.includes("/settings")) location.hash = `/${current.environmentId}/${encodeURIComponent(current.threadId)}`;
    ext.scheduleDOM();
    // A stop or reinjection during the wait must not open anything afterwards.
    if ((await ext.host.panel.show(current)) || api.signal.aborted) return;
    // T3's panel did not render a slot (the probe now names the missing hook): use the browser.
    hide();
    const repo = repositories.selected(ext.current);
    window.open(record ? record.desk + recordPath(record) : repo ? repo.desk + repositoryPath(repo) : ext.deskUrls[0], "_blank", "noopener");
  }
  function hide() {
    desiredOpen = false;
    if (ext.current) {
      ext.view.openThreads[threadKey(ext.current)] = false;
      ext.saveView();
    }
    ext.scheduleDOM();
  }
  ext.listen(linkProject, "click", openSettings);
  ext.listen(iframe, "error", () => {
    panelError.hidden = false;
    panelError.textContent = `Could not load ${iframe.src}. Check connectivity to ${frameDesk} and use Refresh.`;
  });

  // The host contract. Messages count only from the Desk iframe's window and the Desk's origin,
  // and every message to it names that origin as its target.
  const deskOrigin = () => new URL(frameDesk ?? "").origin;
  let sentTheme = "";
  function sendTheme(always: boolean) {
    if (!deskReady || !iframe.contentWindow) return;
    const theme = readHostTheme(api.root);
    const text = JSON.stringify(theme);
    if (!always && text === sentTheme) return;
    sentTheme = text;
    iframe.contentWindow.postMessage({ type: "host:theme", ...theme }, deskOrigin());
  }
  ext.onDispose(observeHostTheme(() => sendTheme(false)));
  // This thread: the thread T3 shows, the threads it started (T3's lineage) and the item and mission
  // IDs its conversation names (the reference scanner), all known to the extension. It is drawn
  // again when T3 shows another thread, and when the children or the mentioned IDs change.
  // The IDs each thread's conversation names, as last scanned for that thread. A thread is drawn at
  // once with what is known for it (nothing if it was never scanned); its scan adds the rest.
  const mentionsByThread = new Map<string, string[]>();
  let drawn: ThisThreadProps | null = null;
  let drawnKey = "";
  /** The thread T3 shows as This thread follows it, whether or not the panel shows it. */
  function followed(current: ThreadContext) {
    return {
      thread: current.threadId,
      repository: repositories.selected(current)?.repo ?? null,
      children: ext.host?.children?.(current.threadId) ?? [],
      // With reference links off nothing is scanned, so no thread is said to name anything.
      mentioned: settings.references ? (mentionsByThread.get(threadKey(current)) ?? []) : [],
    };
  }
  function threadProps(): ThisThreadProps | null {
    const current = ext.current;
    const repo = repositories.selected(current);
    if (!current || !repo) return null;
    const desk = repo.desk;
    const { thread, children, mentioned } = followed(current);
    return {
      desk,
      repository: repo.repo,
      thread,
      children,
      mentioned,
      openItem: (repo, id) => loadFrame(desk, `/r/${encodeURIComponent(repo)}/item/${encodeURIComponent(id)}`),
      // The mission page draws over T3's main area; the record page of a proposed plan, which it does not serve, is framed here.
      openMission: (repo, id, proposed) => (proposed ? loadFrame(desk, `/r/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}`) : links.openMission(desk, repo, id)),
      openInbox: () => loadFrame(desk, "/inbox"),
      openThread,
      openUrl: (url) => window.open(url, "_blank", "noopener"),
    };
  }
  // Draws This thread while it is shown, only when what it shows changed; otherwise draws nothing, so
  // a hidden page reads nothing.
  function drawThread() {
    const props = !framed && mount.isConnected && !mount.hidden ? threadProps() : null;
    if (!props) {
      drawn = null;
      drawnKey = "";
      thisThread.hide();
      return;
    }
    const key = JSON.stringify([props.desk, props.repository, props.thread, props.children, props.mentioned]);
    if (key === drawnKey) return;
    drawnKey = key;
    drawn = props;
    thisThread.show(props);
  }
  // Children appear when a thread starts one; the snapshot is read again every 3 seconds.
  const childrenTimer = setInterval(drawThread, 3000);
  ext.onDispose(() => clearInterval(childrenTimer));
  function openThread(threadId: string) {
    if (!ext.host?.findThread) {
      notify(`Cannot open thread ${threadId}: T3 hook module:environment-atoms is missing (see Customizations).`);
      return;
    }
    const environmentId = ext.host.findThread(threadId);
    if (!environmentId) {
      notify(`Thread ${threadId} is not in any connected T3 environment.`);
      return;
    }
    ext.host.showThread(environmentId, threadId);
  }
  ext.listen(window, "message", (event) => {
    if (!frameDesk || event.source !== iframe.contentWindow || event.origin !== deskOrigin()) return;
    const message = deskMessage(event.data);
    if (message?.type === "desk:ready") {
      deskReady = true;
      sendTheme(true);
    } else if (message?.type === "desk:navigated") {
      framePath = message.path;
      deskTitle = message.title;
      ext.scheduleDOM();
    } else if (message?.type === "desk:open-thread") openThread(message.thread);
  });

  const tab = createDeskTab(ext, {
    toggle: () => (desiredOpen ? hide() : void show()),
    close: hide,
    refresh: () => {
      if (framed && frameDesk && framePath) loadFrame(frameDesk, framePath);
      else thisThread.refresh();
    },
    settings: openSettings,
    launch: () => void show(),
  });
  // A click on one of T3's own tabs shows that tab, so the Desk goes.
  ext.listen(
    document,
    "click",
    (event) => {
      const target = event.target instanceof Element ? event.target : null;
      if (target?.closest(`${hostSelectors.panelTabbar} ${hostSelectors.panelTab}`)) hide();
    },
    { capture: true },
  );

  function sync() {
    observeHostDom();
    const area = document.querySelector(hostSelectors.panel);
    const content = area?.querySelector<HTMLElement>(hostSelectors.panelContent);
    const tabbar = area?.querySelector(hostSelectors.panelTabbar);
    const current = ext.current;
    const allowed = settings.enabled && deskAvailable(ext.host) && Boolean(current) && !location.hash.includes("/settings");
    const state = allowed && current ? ext.host?.panel?.state(current) : undefined;
    const visible = Boolean(allowed && desiredOpen && state?.isOpen && content && area && area.getBoundingClientRect().width > 0);
    const repo = visible ? repositories.selected(current) : null;
    const title = framed ? deskTitle : "This thread";
    tab.sync(tabbar, {
      allowed,
      visible,
      label: repo && title ? title : "Desk",
      title: repo ? `Atelier Desk: ${title || repo.repo}` : "Atelier Desk",
      external: framed ? (frameDesk && framePath !== null ? frameDesk + framePath : null) : repo ? repo.desk + repositoryPath(repo) : null,
      repository: Boolean(repo),
    });
    if (!visible || !content || !area) {
      mount.hidden = true;
      restoreNative();
      drawThread();
      return;
    }
    if (surfaceContent !== content) {
      restoreNative();
      surfaceContent = content;
    }
    if (!positions.has(content)) {
      positions.set(content, content.style.position);
      if (getComputedStyle(content).position === "static") content.style.position = "relative";
    }
    if (mount.parentElement !== content) content.append(mount);
    content.setAttribute("data-t3-desk-selected", "");
    if (tabbar) {
      tabbar.setAttribute("data-t3-desk-tabbar", "");
      markedTabbars.add(tabbar);
    }
    for (const child of content.children) {
      if (child instanceof HTMLElement && child !== mount && !nativeInert.has(child)) {
        nativeInert.set(child, child.inert);
        child.inert = true;
      }
    }
    mount.hidden = false;
    // A project with no repository shows how to link one, also over a page framed for another;
    // a record link from it opens its repository for the project (repositories.override).
    empty.hidden = Boolean(repo);
    iframe.hidden = backBar.hidden = !framed || !repo;
    backTitle.textContent = deskTitle;
    // The panel holds This thread or the frame, never both, so This thread has no iframe beside it.
    native.hidden = framed || !repo;
    if (framed && !iframe.isConnected) native.after(iframe);
    if (!framed && iframe.isConnected) iframe.remove();
    drawThread();
    if (!areaLabels.has(area)) areaLabels.set(area, area.getAttribute("aria-label"));
    area.setAttribute("aria-label", `Workspace panel: Atelier Desk, ${title || "This thread"}`);
  }
  ext.onDOM(sync);

  ext.onContext((context) => {
    if (!context) {
      desiredOpen = false;
      return;
    }
    desiredOpen = Boolean(ext.view.openThreads[threadKey(context)]);
    nativeActiveId = ext.host?.panel?.state(context)?.activeSurfaceId ?? null;
    // The thread is drawn at once; its conversation's scan adds its mentions.
    drawThread();
  });
  ext.diagnostics.panel = () => ({
    desiredOpen,
    deskAvailable: deskAvailable(ext.host),
    panelView: framed ? "frame" : "this-thread",
    frame: { desk: frameDesk, path: framePath, title: deskTitle, ready: deskReady },
    // What This thread follows for the thread T3 shows, and what it is drawn with now (null while it is not shown).
    thread: ext.current && followed(ext.current),
    thisThread: drawn && { desk: drawn.desk, repository: drawn.repository, thread: drawn.thread, children: drawn.children, mentioned: drawn.mentioned },
  });
  ext.onDispose(() => {
    clearTimeout(noticeTimer);
    restoreNative();
    mount.remove();
    globalStyle.remove();
    for (const [node, position] of positions) node.style.position = position;
  });

  return {
    show,
    openThread,
    noteMentions(key, ids) {
      mentionsByThread.set(key, ids);
      drawThread();
    },
    drawThread,
    watchPanel() {
      const panel = ext.host?.panel;
      if (!panel) return;
      ext.onDispose(
        panel.subscribe(() => {
          const active = ext.current ? (panel.state(ext.current)?.activeSurfaceId ?? null) : null;
          if (active !== nativeActiveId) {
            nativeActiveId = active;
            hide();
          }
          ext.scheduleDOM();
        }),
      );
    },
  };
}
