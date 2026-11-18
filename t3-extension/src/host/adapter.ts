// The only client code coupled to T3 internals. Every hook has a named probe:
// a missing hook is reported by name and turns off only the surfaces that need it.
import type { ExtensionApi, Probe } from "./api";

export const hostSelectors = {
  panel: '[data-workspace-area="panel"]',
  panelContent: "[data-right-panel-surface-content]",
  panelTabbar: "[data-right-panel-tabbar]",
  panelTabList: "[data-right-panel-tab-list]",
  panelAddButton: 'button[aria-label="Add panel surface"]',
  panelTab: "[data-active-tab]",
  layoutControls: "[data-panel-layout-controls]",
  conversation: '[data-workspace-area="conversation"]',
  settingsPage: "[data-settings-page-scroll]",
  // Project strip on the new-thread screen: the composer overlay T3 centres, its composer stack
  // (the strip goes under it) and its editor (ProseMirror), which a Next up card fills.
  composerOverlay: '[data-workspace-area="composer"][data-chat-composer-overlay]',
  composerStack: "[data-chat-composer-stack]",
  // Sidebar views: the search field (the Inbox row goes below its group), T3's main area (the
  // layer covers it) and its thread rows (a click on one closes the layer).
  sidebarSearch: '[data-workspace-area="sidebar"] [data-slot="sidebar-group"] input[type="search"]',
  mainInset: 'main[data-slot="sidebar-inset"]',
  sidebarRow: "[data-sidebar-row]",
  // T3's own thread list items (li[data-thread-item="<environment>:<thread>"]). In the boring
  // sidebar they follow their project's folder header in one flat list, which missions reorder
  // with CSS order; the folder's button names it and says whether it is open.
  threadItem: "[data-thread-item]",
  projectFolder: '[data-workspace-area="sidebar"] li[data-testid="sidebar-project-folder"]',
  folderButton: 'button[data-sidebar-row^="folder:"][aria-expanded]',
  // A thread row's Settle button: pressed and dragged it settles every row between, in T3's order.
  settleButton: 'li[data-thread-item] button[aria-label="Settle thread"]',
  composerEditor: '[data-testid="composer-editor"]',
  // The thread breadcrumb's project button, which opens the project home instead.
  breadcrumbProject: 'nav[aria-label="Thread breadcrumb"] li:first-child button',
  // Which thread the rendered conversation is: T3 wraps each assistant message in an element naming
  // its environment and thread (AssistantCitationSource); every row has a timeline row ID.
  conversationThread: "[data-assistant-citation-thread]",
  timelineRow: "[data-timeline-row-id]",
} as const;
export type HostSelector = keyof typeof hostSelectors;

/**
 * Every probe by name: `dom:<selector>` for a DOM hook, `module:<shape>` for a module of T3's,
 * `route:thread` and `route:draft` for what a route names, `state:thread-origin` for the lineage
 * fields of T3's threads, `css:theme` for its theme variables, and `layout:threadList` for the
 * shape of its sidebar list.
 */
export type ProbeName = `dom:${HostSelector}` | `module:${ModuleShape}` | "route:thread" | "route:draft" | "state:thread-origin" | "css:theme" | "layout:threadList";

// The injected script's api, which setHostProbe reports through. start() sets it before anything
// here runs.
let hostApi: ExtensionApi | null = null;
export function useHostApi(api: ExtensionApi): void {
  hostApi = api;
}
export const hostProbes: Partial<Record<ProbeName, Probe>> = Object.fromEntries(
  (Object.keys(hostSelectors) as HostSelector[]).map((name) => [`dom:${name}`, { ok: null, detail: `${hostSelectors[name]} not seen yet` }]),
);
export function setHostProbe(name: ProbeName, ok: boolean | null, detail: string): void {
  if (hostProbes[name]?.ok === ok && hostProbes[name]?.detail === detail) return;
  hostProbes[name] = { ok, detail };
  hostApi?.reportProbes(hostProbes as Record<string, Probe>);
}
/** Whether a probe has been found missing; a probe not yet seen is not. */
export const probeMissing = (name: ProbeName): boolean => hostProbes[name]?.ok === false;

// DOM hooks only exist while their part of T3 is shown, so they are confirmed when seen.
export function observeHostDom(): void {
  for (const [name, selector] of Object.entries(hostSelectors) as [HostSelector, string][]) {
    if (!hostProbes[`dom:${name}`]?.ok && document.querySelector(selector)) setHostProbe(`dom:${name}`, true, selector);
  }
}

// A DOM hook a view needs where T3 should render it: found, it is confirmed; still not found 3
// seconds after the view first looked for it, it is reported missing and that view turns off.
// hostDomExpected holds the hooks being waited for, so the caller can look again.
export const hostDomExpected: Partial<Record<HostSelector, number>> = {};
export function expectHostDom(name: HostSelector, expected: boolean, where: string): void {
  if (!expected) {
    delete hostDomExpected[name];
    return;
  }
  const selector = hostSelectors[name];
  if (document.querySelector(selector)) {
    delete hostDomExpected[name];
    setHostProbe(`dom:${name}`, true, selector);
    return;
  }
  hostDomExpected[name] ??= Date.now();
  if (Date.now() - hostDomExpected[name] >= 3000) setHostProbe(`dom:${name}`, false, `${selector} not found 3 seconds after ${where}`);
}

// Every module discovery imported, for diagnostics and the contract test.
export const hostImportedModules: string[] = [];

// T3's private state, as far as the extension reads it. Each is a guess at a shape T3 does not
// publish; discovery checks the parts named in `shapes` before any is used.

/** A project in T3's snapshot of an environment. */
export type T3Project = { id: string; title: string; repositoryIdentity?: { canonicalKey?: string | null } | null };
/** A thread in T3's snapshot of an environment. */
export type T3Thread = {
  id: string;
  title: string;
  status: string;
  projectId: string;
  /** Absent on an older T3; parentThreadId names the thread that started a local subagent. */
  lineage?: { parentThreadId: string | null };
  /** The creator's thread, in another environment, of a thread a linked environment started here. */
  delegatedFrom?: { environmentId: string; threadId: string; title?: string } | null;
};
export type T3Snapshot = { projects: T3Project[]; threads: T3Thread[] };
/** An effect-atom; the registry reads it. */
type Atom<T> = { readonly __atom?: T };
type AtomRegistry = { get<T>(atom: Atom<T>): T };
type ThreadSnapshotAtoms = { snapshotAtom(environmentId: string): Atom<T3Snapshot | null | undefined> };
type EnvironmentAtoms = { connectedEnvironmentIdsAtom: Atom<string[] | null | undefined> };
/** T3's workspace panel for a thread, in its panel store. */
export type T3PanelState = { isOpen: boolean; activeSurfaceId: string | null };
type PanelStore = {
  getState(): { byThreadKey: Record<string, T3PanelState | undefined>; show(target: { environmentId: string; threadId: string }): void };
  subscribe(listener: () => void): () => void;
};
type T3Draft = { environmentId: string; projectId: string | null; logicalProjectKey?: string | null };
type DraftStore = {
  getState(): { draftThreadsByThreadKey: Record<string, T3Draft | undefined>; draftsByThreadKey: Record<string, { prompt?: string } | undefined> };
};

// Modules are matched by the shape of their exports, not by chunk or export names,
// which change with every T3 build.
type Shape = (value: any) => boolean;
const shapes = {
  "workspace-panel-store": (value) => typeof value.subscribe === "function" && ["byThreadKey", "show", "openBrowser"].every((key) => key in value.getState()),
  "atom-registry": (value) => "~effect/reactivity/AtomRegistry" in value && typeof value.get === "function",
  "thread-snapshot-atom": (value) => ["snapshotAtom", "familyAtom", "mountRows"].every((key) => typeof value[key] === "function"),
  "environment-atoms": (value) => ["environmentIdsAtom", "connectedEnvironmentIdsAtom"].every((key) => key in value),
  // T3's composer draft store: a new-thread screen (#/draft/<id>) names its project there.
  "composer-draft-store": (value) => typeof value.getState === "function" && "draftThreadsByThreadKey" in value.getState(),
} satisfies Record<string, Shape>;
export type ModuleShape = keyof typeof shapes;
type Modules = {
  "workspace-panel-store": PanelStore;
  "atom-registry": AtomRegistry;
  "thread-snapshot-atom": ThreadSnapshotAtoms;
  "environment-atoms": EnvironmentAtoms;
  "composer-draft-store": DraftStore;
};

/** The thread T3 shows, with its project and that project's repository identity. */
export type ThreadContext = { environmentId: string; threadId: string; projectId: string; projectName: string; repositoryKey: string | null };
/** The new-thread screen T3 shows and its project. */
export type DraftContext = { draftId: string; environmentId: string; projectId: string; projectName: string | null; repositoryKey: string | null };
/** A T3 project, as a mapping to a repository reads it. */
export type ProjectRef = { environmentId: string; projectId: string; repositoryKey: string | null };
/** A thread of a connected environment, with who started it. */
export type ThreadEntry = {
  environmentId: string;
  id: string;
  title: string;
  status: string;
  /** lineage.parentThreadId: the thread that started this local subagent (delegate_task). */
  parent: string | null;
  /** delegatedFrom.threadId: the creator's thread ID in the linked environment that started it. */
  delegatedFrom: string | null;
  hasLineage: boolean;
  projectId: string;
  repositoryKey: string | null;
};

/** What the extension uses of T3. A null member is off: the probes say which hook it lacks. */
export type HostAdapter = {
  context: (() => Promise<ThreadContext | null>) | null;
  draftContext: (() => DraftContext | null) | null;
  draftIds: (() => string[]) | null;
  draftPrompt: ((draftId: string) => string | null) | null;
  fillComposer: ((draftId: string, text: string) => Promise<string | null>) | null;
  conversationThread(conversation: Element): string | null;
  children: ((threadId: string) => string[]) | null;
  threadList: (() => ThreadEntry[]) | null;
  findThread: ((threadId: string) => string | null) | null;
  showThread(environmentId: string, threadId: string): void;
  panel: HostPanel | null;
};
export type HostPanel = {
  state(context: { environmentId: string; threadId: string }): T3PanelState | undefined;
  /** Opens T3's panel and resolves to its content slot, or to null with the missing hook recorded. */
  show(context: { environmentId: string; threadId: string }): Promise<Element | null>;
  subscribe(listener: () => void): () => void;
};

const aborted = () => new DOMException("Extension was reloaded", "AbortError");
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export async function createHostAdapter(api: ExtensionApi): Promise<HostAdapter> {
  const names = Object.keys(shapes) as ModuleShape[];
  // Last seen chunk names only decide the search order, so a renamed chunk costs time, not a hook.
  const likelyChunks = /\/(terminalFocus|session|utils|toast)-[^/]*\.js$/;
  const found: Partial<Modules> = {};
  const searched = new Set<string>();
  const complete = () => names.every((name) => found[name]);
  const deadline = Date.now() + 15000;
  while (!api.signal.aborted && !complete() && Date.now() < deadline) {
    // Only modules the renderer itself loads as modules: its entry script and the modulepreload
    // links Vite adds for static and lazy chunks. Never other script resources such as workers,
    // whose top-level code would run in the window and outlive the extension.
    const urls = [...document.querySelectorAll<HTMLScriptElement | HTMLLinkElement>('script[type="module"][src], link[rel="modulepreload"][href]')]
      .map((node) => ("src" in node ? node.src : node.href))
      .filter((url) => url.startsWith(location.origin + "/") && !searched.has(url))
      .sort((a, b) => Number(likelyChunks.test(b)) - Number(likelyChunks.test(a)));
    for (const url of new Set(urls)) {
      if (complete()) break;
      searched.add(url);
      let module: Record<string, unknown>;
      try {
        module = await import(/* @vite-ignore */ url);
      } catch {
        continue;
      }
      hostImportedModules.push(url);
      for (const value of Object.values(module)) {
        if (!value || (typeof value !== "object" && typeof value !== "function")) continue;
        for (const name of names) {
          try {
            if (!found[name] && (shapes[name] as Shape)(value)) (found as Record<string, unknown>)[name] = value;
          } catch {}
        }
      }
    }
    if (!complete()) await sleep(200);
  }
  if (api.signal.aborted) throw aborted();
  for (const name of names) {
    setHostProbe(`module:${name}`, Boolean(found[name]), found[name] ? "found" : `no export with this shape in ${searched.size} T3 modules after 15 seconds`);
  }
  const panelStore = found["workspace-panel-store"];
  const registry = found["atom-registry"];
  const threads = found["thread-snapshot-atom"];
  const environments = found["environment-atoms"];
  const drafts = found["composer-draft-store"];
  const repositoryOf = (project: T3Project | undefined) => project?.repositoryIdentity?.canonicalKey ?? null;

  // Reads T3's own list of the environment's projects and threads, with the rows T3 loads
  // separately for an older or child thread merged in. It needs no request, so it works
  // whichever way T3 authenticates to that environment.
  async function context(registry: AtomRegistry, threads: ThreadSnapshotAtoms): Promise<ThreadContext | null> {
    const hash = location.hash;
    const route = new URL(hash.slice(1) || "/", "https://t3.invalid");
    if (route.pathname.startsWith("/settings") || route.pathname.startsWith("/draft/")) return null;
    const [, environmentId, encodedThreadId] = route.pathname.split("/");
    if (!environmentId || !encodedThreadId) return null;
    const threadId = decodeURIComponent(encodedThreadId);
    // T3 shows a cached list first and adds this thread's row when it has synchronised or
    // loaded it, so keep looking while the route is still this thread.
    const deadline = Date.now() + 15000;
    let snapshot: T3Snapshot | null | undefined = null;
    while (!api.signal.aborted && location.hash === hash) {
      snapshot = registry.get(threads.snapshotAtom(environmentId));
      const thread = snapshot?.threads.find((thread) => thread.id === threadId);
      const project = snapshot?.projects.find((project) => project.id === thread?.projectId);
      if (project) {
        setHostProbe("route:thread", true, `#/${environmentId}/<thread>`);
        return { environmentId, threadId, projectId: project.id, projectName: project.title, repositoryKey: repositoryOf(project) };
      }
      if (Date.now() >= deadline) break;
      await sleep(100);
    }
    if (api.signal.aborted) throw aborted();
    if (location.hash !== hash) return null;
    if (!snapshot) throw new Error(`T3 adapter: environment ${environmentId} had no project list within 15 seconds. Check its connection in T3, then reload scripts.`);
    setHostProbe("route:thread", false, `#/${environmentId}/${threadId} named no thread with a project in that environment within 15 seconds`);
    return null;
  }
  const connected = (registry: AtomRegistry, environments: EnvironmentAtoms) => registry.get(environments.connectedEnvironmentIdsAtom) ?? [];
  // The connected environment whose loaded threads include threadId, or null.
  function findThread(registry: AtomRegistry, threads: ThreadSnapshotAtoms, environments: EnvironmentAtoms, threadId: string): string | null {
    for (const environmentId of connected(registry, environments)) {
      if (registry.get(threads.snapshotAtom(environmentId))?.threads.some((thread) => thread.id === threadId)) return environmentId;
    }
    return null;
  }
  // The project of the new-thread screen T3 shows (#/draft/<id>), from its draft store, or null.
  function draftContext(registry: AtomRegistry, threads: ThreadSnapshotAtoms, drafts: DraftStore): DraftContext | null {
    const match = location.hash.match(/^#\/draft\/([^/?#]+)/);
    if (!match) return null;
    const draftId = decodeURIComponent(match[1]);
    const draft = drafts.getState().draftThreadsByThreadKey[draftId];
    if (!draft) {
      setHostProbe("route:draft", false, `#/draft/${draftId} has no entry in T3's draftThreadsByThreadKey`);
      return null;
    }
    setHostProbe("route:draft", true, "#/draft/<draft> → draftThreadsByThreadKey");
    if (!draft.projectId) return null;
    const project = registry.get(threads.snapshotAtom(draft.environmentId))?.projects.find((project) => project.id === draft.projectId);
    return { draftId, environmentId: draft.environmentId, projectId: draft.projectId, projectName: project?.title ?? null, repositoryKey: repositoryOf(project) ?? draft.logicalProjectKey ?? null };
  }
  // Puts text in the composer of new-thread draft draftId and resolves to null, or to the reason it
  // did not. T3 must show that draft, and the write must change that draft's prompt in T3's draft
  // store to the text and no other draft's: the composer on the page can still be another
  // thread's while T3 renders. A write that reached another draft is undone in that draft.
  async function fillComposer(draftContext: () => DraftContext | null, drafts: DraftStore, draftId: string, text: string): Promise<string | null> {
    if (draftContext()?.draftId !== draftId) return `T3 does not show draft ${draftId} (route ${location.hash})`;
    const editor = document.querySelector<HTMLElement>(hostSelectors.composerEditor);
    if (!editor) return `T3's composer editor (${hostSelectors.composerEditor}) is not on screen`;
    const prompts = () => Object.fromEntries(Object.entries(drafts.getState().draftsByThreadKey).map(([key, draft]) => [key, draft?.prompt ?? ""]));
    const before = prompts();
    if (!writeHostComposer(editor, text)) return `T3's composer refused execCommand ${text ? "insertText" : "delete"}`;
    // T3 records the editor's text in its draft store a few microtasks later.
    let after = prompts();
    const changed = () => Object.keys({ ...before, ...after }).filter((key) => (after[key] ?? "") !== (before[key] ?? ""));
    for (let turn = 0; turn < 20 && changed().length === 0 && (before[draftId] ?? "") !== text; turn++) {
      await Promise.resolve();
      after = prompts();
    }
    const others = changed().filter((key) => key !== draftId);
    if (others.length) {
      writeHostComposer(editor, before[others[0]] ?? "");
      return `the composer on the page was draft ${others.join(", ")}'s, not ${draftId}'s; its prompt was put back`;
    }
    if ((after[draftId] ?? "") !== text) return `T3's draft store did not record the text for draft ${draftId} (it holds ${JSON.stringify(after[draftId] ?? null)})`;
    return null;
  }
  // Every thread of the connected environments, with its project's repository identity and who
  // started it: lineage.parentThreadId for a local subagent (delegate_task), delegatedFrom.threadId
  // for a thread a linked environment's agent delegated or launched here (the creator's thread ID
  // in that other environment).
  function threadList(registry: AtomRegistry, threads: ThreadSnapshotAtoms, environments: EnvironmentAtoms): ThreadEntry[] {
    const list: ThreadEntry[] = [];
    for (const environmentId of connected(registry, environments)) {
      const snapshot = registry.get(threads.snapshotAtom(environmentId));
      const projects = new Map((snapshot?.projects ?? []).map((project) => [project.id, project]));
      for (const thread of snapshot?.threads ?? []) {
        list.push({
          environmentId,
          id: thread.id,
          title: thread.title,
          status: thread.status,
          parent: thread.lineage?.parentThreadId ?? null,
          delegatedFrom: thread.delegatedFrom?.threadId ?? null,
          hasLineage: "lineage" in thread,
          projectId: thread.projectId,
          repositoryKey: repositoryOf(projects.get(thread.projectId)),
        });
      }
    }
    // Only the lineage field is checked: delegatedFrom is absent on threads no linked environment started.
    if (list.length) {
      const shells = list.length - list.filter((thread) => thread.hasLineage).length;
      setHostProbe("state:thread-origin", shells === 0, shells === 0 ? "lineage on every thread shell; delegatedFrom where a linked environment started the thread" : `${shells} of ${list.length} thread shells have no lineage field`);
    }
    return list;
  }

  const drafted = registry && threads && drafts ? () => draftContext(registry, threads, drafts) : null;
  const listed = registry && threads && environments ? () => threadList(registry, threads, environments) : null;
  return {
    draftContext: drafted,
    // The new-thread drafts that name a project, for the contract test to open one.
    draftIds: drafts
      ? () =>
          Object.entries(drafts.getState().draftThreadsByThreadKey)
            .filter(([, draft]) => draft?.projectId)
            .map(([id]) => id)
      : null,
    // The prompt T3's draft store holds for a new-thread draft, for the contract test.
    draftPrompt: drafts ? (draftId) => drafts.getState().draftsByThreadKey[draftId]?.prompt ?? null : null,
    fillComposer: drafted && drafts ? (draftId, text) => fillComposer(drafted, drafts, draftId, text) : null,
    // The environment:thread of the conversation T3 has rendered, from its assistant messages, or
    // null when it has none.
    conversationThread: (conversation) => {
      const marked = conversation.querySelector(hostSelectors.conversationThread);
      return marked ? `${marked.getAttribute("data-assistant-citation-environment")}:${marked.getAttribute("data-assistant-citation-thread")}` : null;
    },
    // Threads this thread started, by either link.
    children: listed
      ? (threadId) =>
          listed()
            .filter((thread) => thread.parent === threadId || thread.delegatedFrom === threadId)
            .map((thread) => thread.id)
            .sort()
      : null,
    threadList: listed,
    // null means the surfaces that need this hook are off; hostProbes says why.
    context: registry && threads ? () => context(registry, threads) : null,
    findThread: registry && threads && environments ? (threadId) => findThread(registry, threads, environments, threadId) : null,
    showThread: (environmentId, threadId) => {
      location.hash = `/${environmentId}/${encodeURIComponent(threadId)}`;
    },
    panel: panelStore
      ? {
          state: (context) => panelStore.getState().byThreadKey[`${context.environmentId}:${context.threadId}`],
          async show(context) {
            panelStore.getState().show({ environmentId: context.environmentId, threadId: context.threadId });
            const slot = () => document.querySelector(hostSelectors.panel)?.querySelector(hostSelectors.panelContent) ?? null;
            for (let attempt = 0; attempt < 30 && !slot() && !api.signal.aborted; attempt++) await sleep(100);
            if (api.signal.aborted || slot()) return slot();
            for (const name of ["panel", "panelContent"] as const) {
              if (!document.querySelector(hostSelectors[name])) setHostProbe(`dom:${name}`, false, `${hostSelectors[name]} not found 3 seconds after opening T3's workspace panel`);
            }
            return null;
          },
          subscribe: (listener) => panelStore.subscribe(listener),
        }
      : null,
  };
}

// Why T3's boring sidebar list, the parent of its project folders, is not what missions need, or
// null: one flex column holding every folder, each with its button.
export function hostListProblem(list: Element): string | null {
  const style = getComputedStyle(list);
  if (style.display !== "flex" || style.flexDirection !== "column") return `the folders' list is "${style.display} ${style.flexDirection}", not a flex column`;
  for (const folder of document.querySelectorAll(hostSelectors.projectFolder)) {
    if (folder.parentElement !== list) return "the project folders are not all in one list";
    if (!folder.querySelector(hostSelectors.folderButton)) return `a project folder has no ${hostSelectors.folderButton}`;
  }
  return null;
}

// T3's composer is a ProseMirror (TipTap) editor: its content is selected and replaced with
// execCommand('insertText'), which fires the beforeinput/input events ProseMirror handles. Empty
// text clears it.
export function writeHostComposer(editor: HTMLElement, text: string): boolean {
  editor.focus();
  getSelection()?.selectAllChildren(editor);
  return document.execCommand(text ? "insertText" : "delete", false, text);
}

// T3's theme, from its own CSS custom properties, in the form of the Desk's host:theme message.
// Each colour is resolved to a computed value, so the Desk gets no var() or color-mix() of T3's.
export const hostThemeColors = { background: "--background", foreground: "--foreground", surface: "--card", border: "--border", muted: "--muted-foreground", accent: "--primary" } as const;
export type HostTheme = { colors: Partial<Record<keyof typeof hostThemeColors, string>>; font: string; scheme: "dark" | "light" };
export function readHostTheme(root: ShadowRoot | Element): HostTheme {
  const probe = document.createElement("span");
  root.append(probe);
  const colors: HostTheme["colors"] = {};
  const missing: string[] = [];
  for (const [name, variable] of Object.entries(hostThemeColors) as [keyof typeof hostThemeColors, string][]) {
    if (!getComputedStyle(document.documentElement).getPropertyValue(variable).trim()) {
      missing.push(variable);
      continue;
    }
    probe.style.color = `var(${variable})`;
    colors[name] = getComputedStyle(probe).color;
  }
  probe.remove();
  setHostProbe("css:theme", missing.length === 0, missing.length ? `T3 defines no ${missing.join(", ")} on its root element` : Object.values(hostThemeColors).join(", "));
  const scheme = document.documentElement.classList.contains("dark") ? "dark" : "light";
  return { colors, font: getComputedStyle(document.body).fontFamily, scheme };
}
// T3 switches theme on its root element's class and style attributes.
export function observeHostTheme(callback: () => void): () => void {
  const observer = new MutationObserver(callback);
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class", "style", "data-theme"] });
  return () => observer.disconnect();
}
