// The only client code coupled to T3 internals. Every hook has a named probe:
// a missing hook is reported by name and turns off only the surfaces that need it.
export const hostSelectors = {
  panel: '[data-workspace-area="panel"]',
  panelContent: '[data-right-panel-surface-content]',
  panelTabbar: '[data-right-panel-tabbar]',
  panelTabList: '[data-right-panel-tab-list]',
  panelAddButton: 'button[aria-label="Add panel surface"]',
  panelTab: '[data-active-tab]',
  layoutControls: '[data-panel-layout-controls]',
  conversation: '[data-workspace-area="conversation"]',
  settingsPage: '[data-settings-page-scroll]',
  // Project strip on the new-thread screen: the composer overlay T3 centres, its composer stack
  // (the strip goes under it) and its editor (ProseMirror), which a Next up card fills.
  composerOverlay: '[data-workspace-area="composer"][data-chat-composer-overlay]',
  composerStack: '[data-chat-composer-stack]',
  // Sidebar views: the search field (the Inbox row goes below its group), T3's main area (the
  // layer covers it) and its thread rows (a click on one closes the layer).
  sidebarSearch: '[data-workspace-area="sidebar"] [data-slot="sidebar-group"] input[type="search"]',
  mainInset: 'main[data-slot="sidebar-inset"]',
  sidebarRow: '[data-sidebar-row]',
  // T3's own thread list items (li[data-thread-item="<environment>:<thread>"]). In the boring
  // sidebar they follow their project's folder header in one flat list, which missions reorder
  // with CSS order; the folder's button names it and says whether it is open.
  threadItem: '[data-thread-item]',
  projectFolder: '[data-workspace-area="sidebar"] li[data-testid="sidebar-project-folder"]',
  folderButton: 'button[data-sidebar-row^="folder:"][aria-expanded]',
  // A thread row's Settle button: pressed and dragged it settles every row between, in T3's order.
  settleButton: 'li[data-thread-item] button[aria-label="Settle thread"]',
  composerEditor: '[data-testid="composer-editor"]',
  // The thread breadcrumb's project button, which opens the project home instead.
  breadcrumbProject: 'nav[aria-label="Thread breadcrumb"] li:first-child button',
  // Which thread the rendered conversation is: T3 wraps each assistant message in an element naming
  // its environment and thread (AssistantCitationSource); every row has a timeline row ID.
  conversationThread: '[data-assistant-citation-thread]',
  timelineRow: '[data-timeline-row-id]',
};
// The injected script's api, which setHostProbe reports through. The caller sets it before
// anything here runs.
let hostApi;
export function useHostApi(api) { hostApi = api; }
// ok: true (found), false (missing), null (DOM hook not on screen yet).
export const hostProbes = Object.fromEntries(Object.keys(hostSelectors).map(name => [`dom:${name}`, { ok: null, detail: `${hostSelectors[name]} not seen yet` }]));
export function setHostProbe(name, ok, detail) {
  if (hostProbes[name]?.ok === ok && hostProbes[name]?.detail === detail) return;
  hostProbes[name] = { ok, detail };
  hostApi.reportProbes?.(hostProbes);
}
// DOM hooks only exist while their part of T3 is shown, so they are confirmed when seen.
export function observeHostDom() {
  for (const [name, selector] of Object.entries(hostSelectors)) {
    if (!hostProbes[`dom:${name}`].ok && document.querySelector(selector)) setHostProbe(`dom:${name}`, true, selector);
  }
}

// A DOM hook a view needs where T3 should render it: found, it is confirmed; still not found 3
// seconds after the view first looked for it, it is reported missing and that view turns off.
// hostDomExpected holds the hooks being waited for, so the caller can look again.
export const hostDomExpected = {};
export function expectHostDom(name, expected, where) {
  if (!expected) { delete hostDomExpected[name]; return; }
  const selector = hostSelectors[name];
  if (document.querySelector(selector)) { delete hostDomExpected[name]; setHostProbe(`dom:${name}`, true, selector); return; }
  hostDomExpected[name] ??= Date.now();
  if (Date.now() - hostDomExpected[name] >= 3000) setHostProbe(`dom:${name}`, false, `${selector} not found 3 seconds after ${where}`);
}

// Every module discovery imported, for diagnostics and the contract test.
export const hostImportedModules = [];

export async function createHostAdapter(api) {
  // Modules are matched by the shape of their exports, not by chunk or export names,
  // which change with every T3 build.
  const shapes = {
    'workspace-panel-store': value => typeof value.subscribe === 'function' && ['byThreadKey', 'show', 'openBrowser'].every(key => key in value.getState()),
    'atom-registry': value => '~effect/reactivity/AtomRegistry' in value && typeof value.get === 'function',
    'thread-snapshot-atom': value => ['snapshotAtom', 'familyAtom', 'mountRows'].every(key => typeof value[key] === 'function'),
    'environment-atoms': value => ['environmentIdsAtom', 'connectedEnvironmentIdsAtom'].every(key => key in value),
    // T3's composer draft store: a new-thread screen (#/draft/<id>) names its project there.
    'composer-draft-store': value => typeof value.getState === 'function' && 'draftThreadsByThreadKey' in value.getState(),
  };
  // Last seen chunk names only decide the search order, so a renamed chunk costs time, not a hook.
  const likelyChunks = /\/(terminalFocus|session|utils|toast)-[^/]*\.js$/;
  const found = {};
  const searched = new Set();
  const complete = () => Object.keys(found).length === Object.keys(shapes).length;
  const deadline = Date.now() + 15000;
  while (!api.signal.aborted && !complete() && Date.now() < deadline) {
    // Only modules the renderer itself loads as modules: its entry script and the modulepreload
    // links Vite adds for static and lazy chunks. Never other script resources such as workers,
    // whose top-level code would run in the window and outlive the extension.
    const urls = [...document.querySelectorAll('script[type="module"][src], link[rel="modulepreload"][href]')]
      .map(node => node.src || node.href)
      .filter(url => url.startsWith(location.origin + '/') && !searched.has(url))
      .sort((a, b) => likelyChunks.test(b) - likelyChunks.test(a));
    for (const url of new Set(urls)) {
      if (complete()) break;
      searched.add(url);
      let module;
      try { module = await import(url); } catch { continue; }
      hostImportedModules.push(url);
      for (const value of Object.values(module)) {
        if (!value || (typeof value !== 'object' && typeof value !== 'function')) continue;
        for (const [name, test] of Object.entries(shapes)) {
          try { if (!found[name] && test(value)) found[name] = value; } catch {}
        }
      }
    }
    if (!complete()) await new Promise(resolve => setTimeout(resolve, 200));
  }
  if (api.signal.aborted) throw new DOMException('Extension was reloaded', 'AbortError');
  for (const name of Object.keys(shapes)) {
    setHostProbe(`module:${name}`, Boolean(found[name]), found[name] ? 'found' : `no export with this shape in ${searched.size} T3 modules after 15 seconds`);
  }
  const panelStore = found['workspace-panel-store'];
  const registry = found['atom-registry'];
  const threads = found['thread-snapshot-atom'];
  const environments = found['environment-atoms'];
  const drafts = found['composer-draft-store'];

  // Reads T3's own list of the environment's projects and threads, with the rows T3 loads
  // separately for an older or child thread merged in. It needs no request, so it works
  // whichever way T3 authenticates to that environment.
  async function context() {
    const hash = location.hash;
    const route = new URL(hash.slice(1) || '/', 'https://t3.invalid');
    if (route.pathname.startsWith('/settings') || route.pathname.startsWith('/draft/')) return null;
    const [, environmentId, encodedThreadId] = route.pathname.split('/');
    if (!environmentId || !encodedThreadId) return null;
    const threadId = decodeURIComponent(encodedThreadId);
    // T3 shows a cached list first and adds this thread's row when it has synchronised or
    // loaded it, so keep looking while the route is still this thread.
    const deadline = Date.now() + 15000;
    let snapshot = null;
    while (!api.signal.aborted && location.hash === hash) {
      snapshot = registry.get(threads.snapshotAtom(environmentId));
      const thread = snapshot?.threads.find(thread => thread.id === threadId);
      const project = snapshot?.projects.find(project => project.id === thread?.projectId);
      if (project) {
        setHostProbe('route:thread', true, `#/${environmentId}/<thread>`);
        return { environmentId, threadId, projectId: project.id, projectName: project.title, repositoryKey: project.repositoryIdentity?.canonicalKey ?? null };
      }
      if (Date.now() >= deadline) break;
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    if (api.signal.aborted) throw new DOMException('Extension was reloaded', 'AbortError');
    if (location.hash !== hash) return null;
    if (!snapshot) throw new Error(`T3 adapter: environment ${environmentId} had no project list within 15 seconds. Check its connection in T3, then reload scripts.`);
    setHostProbe('route:thread', false, `#/${environmentId}/${threadId} named no thread with a project in that environment within 15 seconds`);
    return null;
  }
  // The connected environment whose loaded threads include threadId, or null.
  function findThread(threadId) {
    for (const environmentId of registry.get(environments.connectedEnvironmentIdsAtom) ?? []) {
      if (registry.get(threads.snapshotAtom(environmentId))?.threads.some(thread => thread.id === threadId)) return environmentId;
    }
    return null;
  }
  // The project of the new-thread screen T3 shows (#/draft/<id>), from its draft store, or null.
  function draftContext() {
    const match = location.hash.match(/^#\/draft\/([^/?#]+)/);
    if (!match) return null;
    const draftId = decodeURIComponent(match[1]);
    const draft = drafts.getState().draftThreadsByThreadKey[draftId];
    if (!draft) { setHostProbe('route:draft', false, `#/draft/${draftId} has no entry in T3's draftThreadsByThreadKey`); return null; }
    setHostProbe('route:draft', true, '#/draft/<draft> → draftThreadsByThreadKey');
    if (!draft.projectId) return null;
    const project = registry.get(threads.snapshotAtom(draft.environmentId))?.projects.find(project => project.id === draft.projectId);
    return { draftId, environmentId: draft.environmentId, projectId: draft.projectId, projectName: project?.title ?? null, repositoryKey: project?.repositoryIdentity?.canonicalKey ?? draft.logicalProjectKey ?? null };
  }
  // Puts text in the composer of new-thread draft draftId and resolves to null, or to the reason it
  // did not. T3 must show that draft, and the write must change that draft's prompt in T3's draft
  // store to the text and no other draft's: the composer on the page can still be another
  // thread's while T3 renders. A write that reached another draft is undone in that draft.
  async function fillComposer(draftId, text) {
    if (draftContext()?.draftId !== draftId) return `T3 does not show draft ${draftId} (route ${location.hash})`;
    const editor = document.querySelector(hostSelectors.composerEditor);
    if (!editor) return `T3's composer editor (${hostSelectors.composerEditor}) is not on screen`;
    const prompts = () => Object.fromEntries(Object.entries(drafts.getState().draftsByThreadKey).map(([key, draft]) => [key, draft?.prompt ?? '']));
    const before = prompts();
    if (!writeHostComposer(editor, text)) return `T3's composer refused execCommand ${text ? 'insertText' : 'delete'}`;
    // T3 records the editor's text in its draft store a few microtasks later.
    let after = prompts();
    const changed = () => Object.keys({ ...before, ...after }).filter(key => (after[key] ?? '') !== (before[key] ?? ''));
    for (let turn = 0; turn < 20 && changed().length === 0 && (before[draftId] ?? '') !== text; turn++) { await Promise.resolve(); after = prompts(); }
    const others = changed().filter(key => key !== draftId);
    if (others.length) {
      writeHostComposer(editor, before[others[0]] ?? '');
      return `the composer on the page was draft ${others.join(', ')}'s, not ${draftId}'s; its prompt was put back`;
    }
    if ((after[draftId] ?? '') !== text) return `T3's draft store did not record the text for draft ${draftId} (it holds ${JSON.stringify(after[draftId] ?? null)})`;
    return null;
  }
  // Every thread of the connected environments, with its project's repository identity and who
  // started it: lineage.parentThreadId for a local subagent (delegate_task), delegatedFrom.threadId
  // for a thread a linked environment's agent delegated or launched here (the creator's thread ID
  // in that other environment).
  function threadList() {
    const list = [];
    for (const environmentId of registry.get(environments.connectedEnvironmentIdsAtom) ?? []) {
      const snapshot = registry.get(threads.snapshotAtom(environmentId));
      const projects = new Map((snapshot?.projects ?? []).map(project => [project.id, project]));
      for (const thread of snapshot?.threads ?? []) {
        list.push({ environmentId, id: thread.id, title: thread.title, status: thread.status, parent: thread.lineage?.parentThreadId ?? null, delegatedFrom: thread.delegatedFrom?.threadId ?? null, hasLineage: 'lineage' in thread, projectId: thread.projectId, repositoryKey: projects.get(thread.projectId)?.repositoryIdentity?.canonicalKey ?? null });
      }
    }
    // Only the lineage field is checked: delegatedFrom is absent on threads no linked environment started.
    if (list.length) {
      const shells = list.length - list.filter(thread => thread.hasLineage).length;
      setHostProbe('state:thread-origin', shells === 0, shells === 0 ? 'lineage on every thread shell; delegatedFrom where a linked environment started the thread' : `${shells} of ${list.length} thread shells have no lineage field`);
    }
    return list;
  }
  // Threads this thread started, by either link.
  const children = threadId => threadList().filter(thread => thread.parent === threadId || thread.delegatedFrom === threadId).map(thread => thread.id).sort();
  return {
    draftContext: registry && threads && drafts ? draftContext : null,
    // The new-thread drafts that name a project, for the contract test to open one.
    draftIds: drafts ? () => Object.entries(drafts.getState().draftThreadsByThreadKey).filter(([, draft]) => draft.projectId).map(([id]) => id) : null,
    // The prompt T3's draft store holds for a new-thread draft, for the contract test.
    draftPrompt: drafts ? draftId => drafts.getState().draftsByThreadKey[draftId]?.prompt ?? null : null,
    fillComposer: drafts ? fillComposer : null,
    // The environment:thread of the conversation T3 has rendered, from its assistant messages, or
    // null when it has none.
    conversationThread: conversation => {
      const marked = conversation.querySelector(hostSelectors.conversationThread);
      return marked ? `${marked.getAttribute('data-assistant-citation-environment')}:${marked.getAttribute('data-assistant-citation-thread')}` : null;
    },
    children: registry && threads && environments ? children : null,
    threadList: registry && threads && environments ? threadList : null,
    // null means the surfaces that need this hook are off; hostProbes says why.
    context: registry && threads ? context : null,
    findThread: registry && threads && environments ? findThread : null,
    showThread: (environmentId, threadId) => { location.hash = `/${environmentId}/${encodeURIComponent(threadId)}`; },
    panel: panelStore ? {
      state: context => panelStore.getState().byThreadKey[`${context.environmentId}:${context.threadId}`],
      // Opens T3's panel and resolves to its content slot, or to null with the missing hook recorded.
      async show(context) {
        panelStore.getState().show({ environmentId: context.environmentId, threadId: context.threadId });
        const slot = () => document.querySelector(hostSelectors.panel)?.querySelector(hostSelectors.panelContent);
        for (let attempt = 0; attempt < 30 && !slot() && !api.signal.aborted; attempt++) await new Promise(resolve => setTimeout(resolve, 100));
        if (api.signal.aborted || slot()) return slot();
        for (const name of ['panel', 'panelContent']) {
          if (!document.querySelector(hostSelectors[name])) setHostProbe(`dom:${name}`, false, `${hostSelectors[name]} not found 3 seconds after opening T3's workspace panel`);
        }
        return null;
      },
      subscribe: listener => panelStore.subscribe(listener),
    } : null,
  };
}

// Why T3's boring sidebar list, the parent of its project folders, is not what missions need, or
// null: one flex column holding every folder, each with its button.
export function hostListProblem(list) {
  const style = getComputedStyle(list);
  if (style.display !== 'flex' || style.flexDirection !== 'column') return `the folders' list is "${style.display} ${style.flexDirection}", not a flex column`;
  for (const folder of document.querySelectorAll(hostSelectors.projectFolder)) {
    if (folder.parentElement !== list) return 'the project folders are not all in one list';
    if (!folder.querySelector(hostSelectors.folderButton)) return `a project folder has no ${hostSelectors.folderButton}`;
  }
  return null;
}

// T3's composer is a ProseMirror (TipTap) editor: its content is selected and replaced with
// execCommand('insertText'), which fires the beforeinput/input events ProseMirror handles. Empty
// text clears it.
export function writeHostComposer(editor, text) {
  editor.focus();
  getSelection().selectAllChildren(editor);
  return document.execCommand(text ? 'insertText' : 'delete', false, text);
}

// T3's theme, from its own CSS custom properties, in the form of the Desk's host:theme message.
// Each colour is resolved to a computed value, so the Desk gets no var() or color-mix() of T3's.
export const hostThemeColors = { background: '--background', foreground: '--foreground', surface: '--card', border: '--border', muted: '--muted-foreground', accent: '--primary' };
export function readHostTheme(root) {
  const probe = document.createElement('span');
  root.append(probe);
  const colors = {};
  const missing = [];
  for (const [name, variable] of Object.entries(hostThemeColors)) {
    if (!getComputedStyle(document.documentElement).getPropertyValue(variable).trim()) { missing.push(variable); continue; }
    probe.style.color = `var(${variable})`;
    colors[name] = getComputedStyle(probe).color;
  }
  probe.remove();
  setHostProbe('css:theme', missing.length === 0, missing.length ? `T3 defines no ${missing.join(', ')} on its root element` : Object.values(hostThemeColors).join(', '));
  const scheme = document.documentElement.classList.contains('dark') ? 'dark' : 'light';
  return { colors, font: getComputedStyle(document.body).fontFamily, scheme };
}
// T3 switches theme on its root element's class and style attributes.
export function observeHostTheme(callback) {
  const observer = new MutationObserver(callback);
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class', 'style', 'data-theme'] });
  return () => observer.disconnect();
}
