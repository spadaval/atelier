// The Desk in T3's workspace panel: a frame of the Desk's embedded mode laid over T3's panel
// content, with the Desk tab (tab.ts). Desk remains the backend; no T3 credentials enter its iframe.
// The Desk talks to this page over its host contract (atelier: docs/architecture/desk.md, "The
// embedded mode" and "The host contract"; the Desk's side is desk/src/lib/host.ts).
import { element } from "../core/dom";
import type { Extension } from "../core/extension";
import { hostSelectors, observeHostDom, observeHostTheme, readHostTheme, type ThreadContext } from "../host/adapter";
import { recordPath, repositoryKey, repositoryPath, type RecordRef } from "../repositories";
import { deskAvailable } from "../views";
import { createDeskTab } from "./tab";

/** A message the Desk sends its host (DeskMessage in desk/src/lib/host.ts). */
type DeskMessage =
  | { type: "desk:ready"; version: 1 }
  | { type: "desk:navigated"; path: string; title: string }
  | { type: "desk:open-thread"; thread: string }
  | { type: "desk:resized"; height: number }
  | { type: "desk:key"; key: "Escape" };
/** host:thread, which the Desk reads as its HostContext (desk/src/lib/host.ts). */
type HostThread = { type: "host:thread"; thread: string; repository?: string; children: string[]; mentioned: string[] };

/** A record a link opened, on the Desk that answered for it. */
export type DeskRecord = RecordRef & { desk: string };

export type DeskPanel = {
  /** Opens the panel on the thread T3 shows, at `record` when one is given. */
  show(record?: DeskRecord): Promise<void>;
  /** Goes to a thread the Desk or a page names, or says why it cannot. */
  openThread(threadId: string): void;
  /** Tells the Desk the IDs the conversation of thread `key` (`<environment>:<thread>`) names. */
  noteMentions(key: string, ids: string[]): void;
  /** Tells the Desk the thread T3 shows, when what it would say changed. */
  tellThread(): void;
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

export function createDeskPanel(ext: Extension, openSettings: () => void): DeskPanel {
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
  :host{display:block;height:100%}.panel{height:100%;min-height:0;display:flex;flex-direction:column;background:var(--background,#202124)}iframe{flex:1;min-height:0;width:100%;border:0;background:var(--background,#202124)}.empty{padding:20px;max-width:55ch}.notice,.panel>.error{padding:6px 10px;font-size:12px;border-bottom:1px solid var(--border,#60636a)}`;
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
  root.append(element("div", { class: "panel" }, notice, panelError, empty, iframe));
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
  // The Desk shown in the iframe, its route without /embed (from desk:navigated once it is ready),
  // and its page title.
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
  // The panel loads the embedded mode; Open externally keeps the normal address. Opened plainly
  // it loads /embed/ and the Desk chooses the page for the thread T3 shows (host:thread). A page
  // asked for, a reference or a Refresh, is kept: the Desk is not told the current thread again.
  function loadFrame(desk: string, path: string, follow: boolean) {
    frameDesk = desk;
    framePath = path;
    deskReady = false;
    deskTitle = "";
    sentTheme = "";
    keepThread = follow ? null : (ext.current?.threadId ?? null);
    iframe.src = `${desk}/embed${path}`;
    ext.scheduleDOM();
  }
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
      loadFrame(record.desk, recordPath(record), false);
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
  // host:thread tells the Desk which thread T3 shows, by T3's thread ID, whole, with the Desk
  // repository its project maps to, the threads it started (T3's lineage) and the item and mission
  // IDs its conversation names (the reference scanner). It is sent when the Desk becomes ready, when
  // T3 shows another thread, and when the children or mentioned IDs change; never twice in a row
  // with the same content.
  // Each desk:ready is a fresh Desk that knows no thread, also when T3 re-rendered the panel slot
  // and moving the iframe reloaded it. keepThread is the thread a load whose page was asked for
  // (a reference or Refresh) shows: that thread is not sent, but a switch before the Desk is ready is.
  let threadSent: string | null = null;
  let keepThread: string | null = null;
  // The IDs each thread's conversation names, as last scanned for that thread. A thread is sent at
  // once with what is known for it (nothing if it was never scanned); its scan sends the rest.
  const mentionsByThread = new Map<string, string[]>();
  function threadMessage(current: ThreadContext): HostThread {
    const message: HostThread = { type: "host:thread", thread: current.threadId, children: [], mentioned: [] };
    // The repository lets the Desk show this project, not the system inbox, for a thread that holds nothing.
    const repository = repositories.selected(current)?.repo;
    if (repository) message.repository = repository;
    message.children = ext.host?.children?.(current.threadId) ?? [];
    // With reference links off nothing is scanned, so no thread is said to mention anything.
    message.mentioned = settings.references ? (mentionsByThread.get(threadKey(current)) ?? []) : [];
    return message;
  }
  function tellThread() {
    if (!deskReady || !iframe.contentWindow || !ext.current) return;
    const message = threadMessage(ext.current);
    const text = JSON.stringify(message);
    if (text === threadSent) return;
    threadSent = text;
    iframe.contentWindow.postMessage(message, deskOrigin());
  }
  // Children appear when a thread starts one; the snapshot is read again every 3 seconds.
  const childrenTimer = setInterval(tellThread, 3000);
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
      threadSent = keepThread && keepThread === ext.current?.threadId ? JSON.stringify(threadMessage(ext.current)) : null;
      keepThread = null;
      sendTheme(true);
      tellThread();
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
      if (frameDesk && framePath) loadFrame(frameDesk, framePath, false);
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
    if (repo && frameDesk !== repo.desk) loadFrame(repo.desk, "/", true);
    tab.sync(tabbar, {
      allowed,
      visible,
      label: repo && deskTitle ? deskTitle : "Desk",
      title: repo ? `Atelier Desk: ${deskTitle || repo.repo}` : "Atelier Desk",
      external: frameDesk && framePath !== null ? frameDesk + framePath : null,
      repository: Boolean(repo),
    });
    if (!visible || !content || !area) {
      mount.hidden = true;
      restoreNative();
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
    empty.hidden = Boolean(repo);
    iframe.hidden = !repo;
    if (!areaLabels.has(area)) areaLabels.set(area, area.getAttribute("aria-label"));
    area.setAttribute("aria-label", `Workspace panel: Atelier Desk${deskTitle ? `, ${deskTitle}` : ""}`);
  }
  ext.onDOM(sync);

  ext.onContext((context) => {
    if (!context) {
      desiredOpen = false;
      return;
    }
    desiredOpen = Boolean(ext.view.openThreads[threadKey(context)]);
    nativeActiveId = ext.host?.panel?.state(context)?.activeSurfaceId ?? null;
    // The thread is sent at once; its conversation's scan sends its mentions as an update.
    tellThread();
  });
  ext.diagnostics.panel = () => ({ desiredOpen, deskAvailable: deskAvailable(ext.host), frame: { desk: frameDesk, path: framePath, title: deskTitle, ready: deskReady }, hostThread: threadSent && JSON.parse(threadSent) });
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
      tellThread();
    },
    tellThread,
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
