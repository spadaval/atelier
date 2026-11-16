// The only client code coupled to T3 internals. Every hook has a named probe:
// a missing hook is reported by name and turns off only the surfaces that need it.
const hostSelectors = {
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
  // Sidebar views: the search field (the Inbox row and Missions go below its group), T3's main
  // area (the layer covers it) and its thread rows (a click on one closes the layer).
  sidebarSearch: '[data-workspace-area="sidebar"] [data-slot="sidebar-group"] input[type="search"]',
  mainInset: 'main[data-slot="sidebar-inset"]',
  sidebarRow: '[data-sidebar-row]',
  // T3's own thread list items, hidden by CSS when a mission holds the thread.
  threadItem: '[data-thread-item]',
  composerEditor: '[data-testid="composer-editor"]',
  // The thread breadcrumb's project button, which opens the project home instead.
  breadcrumbProject: 'nav[aria-label="Thread breadcrumb"] li:first-child button',
};
// ok: true (found), false (missing), null (DOM hook not on screen yet).
const hostProbes = Object.fromEntries(Object.keys(hostSelectors).map(name => [`dom:${name}`, { ok: null, detail: `${hostSelectors[name]} not seen yet` }]));
function setHostProbe(name, ok, detail) {
  if (hostProbes[name]?.ok === ok && hostProbes[name]?.detail === detail) return;
  hostProbes[name] = { ok, detail };
  api.reportProbes?.(hostProbes);
}
// DOM hooks only exist while their part of T3 is shown, so they are confirmed when seen.
function observeHostDom() {
  for (const [name, selector] of Object.entries(hostSelectors)) {
    if (!hostProbes[`dom:${name}`].ok && document.querySelector(selector)) setHostProbe(`dom:${name}`, true, selector);
  }
}

// A DOM hook a view needs where T3 should render it: found, it is confirmed; still not found 3
// seconds after the view first looked for it, it is reported missing and that view turns off.
// hostDomExpected holds the hooks being waited for, so the caller can look again.
const hostDomExpected = {};
function expectHostDom(name, expected, where) {
  if (!expected) { delete hostDomExpected[name]; return; }
  const selector = hostSelectors[name];
  if (document.querySelector(selector)) { delete hostDomExpected[name]; setHostProbe(`dom:${name}`, true, selector); return; }
  hostDomExpected[name] ??= Date.now();
  if (Date.now() - hostDomExpected[name] >= 3000) setHostProbe(`dom:${name}`, false, `${selector} not found 3 seconds after ${where}`);
}

// Every module discovery imported, for diagnostics and the contract test.
const hostImportedModules = [];

async function createHostAdapter(api) {
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
  // Every thread of the connected environments, with who started it: lineage.parentThreadId for a
  // local subagent (delegate_task), delegatedFrom.threadId for a thread a linked environment's
  // agent delegated or launched here (the creator's thread ID in that other environment).
  function threadList() {
    const list = [];
    for (const environmentId of registry.get(environments.connectedEnvironmentIdsAtom) ?? []) {
      for (const thread of registry.get(threads.snapshotAtom(environmentId))?.threads ?? []) {
        list.push({ environmentId, id: thread.id, title: thread.title, status: thread.status, parent: thread.lineage?.parentThreadId ?? null, delegatedFrom: thread.delegatedFrom?.threadId ?? null, hasLineage: 'lineage' in thread });
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

// T3's composer is a ProseMirror (TipTap) editor: its content is selected and replaced with
// execCommand('insertText'), which fires the beforeinput/input events ProseMirror handles, so T3's
// draft keeps the text (test-contract.mjs checks the draft store). Empty text clears it.
function fillHostComposer(text) {
  const editor = document.querySelector(hostSelectors.composerEditor);
  if (!editor) throw new Error(`T3's composer editor (${hostSelectors.composerEditor}) is not on screen`);
  editor.focus();
  getSelection().selectAllChildren(editor);
  if (!document.execCommand(text ? 'insertText' : 'delete', false, text)) throw new Error(`T3's composer refused execCommand ${text ? 'insertText' : 'delete'}`);
}

// T3's theme, from its own CSS custom properties, in the form of the Desk's host:theme message.
// Each colour is resolved to a computed value, so the Desk gets no var() or color-mix() of T3's.
const hostThemeColors = { background: '--background', foreground: '--foreground', surface: '--card', border: '--border', muted: '--muted-foreground', accent: '--primary' };
function readHostTheme(root) {
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
function observeHostTheme(callback) {
  const observer = new MutationObserver(callback);
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class', 'style', 'data-theme'] });
  return () => observer.disconnect();
}
