// The only client code coupled to T3 internals. Every hook has a named probe:
// a missing hook is reported by name and turns off only the surfaces that need it.
const hostSelectors = {
  panel: '[data-workspace-area="panel"]',
  panelContent: '[data-right-panel-surface-content]',
  panelTabbar: '[data-right-panel-tabbar]',
  panelAddButton: 'button[aria-label="Add panel surface"]',
  panelTab: '[data-active-tab]',
  layoutControls: '[data-panel-layout-controls]',
  conversation: '[data-workspace-area="conversation"]',
  settingsPage: '[data-settings-page-scroll]',
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

// Every module discovery imported, for diagnostics and the contract test.
const hostImportedModules = [];

async function createHostAdapter(api) {
  // Modules are matched by the shape of their exports, not by chunk or export names,
  // which change with every T3 build.
  const shapes = {
    'workspace-panel-store': value => typeof value.subscribe === 'function' && ['byThreadKey', 'show', 'openBrowser'].every(key => key in value.getState()),
    'atom-registry': value => '~effect/reactivity/AtomRegistry' in value && typeof value.get === 'function',
    'shell-state-atom': value => typeof value.stateValueAtom === 'function' && typeof value.hasSnapshotAtom === 'function',
  };
  // Last seen chunk names only decide the search order, so a renamed chunk costs time, not a hook.
  const likelyChunks = /\/(terminalFocus|session|utils)-[^/]*\.js$/;
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
  const shell = found['shell-state-atom'];

  // T3's own copy of the environment's projects and threads. Reading it needs no request,
  // so it works whichever way T3 authenticates to that environment.
  async function snapshotFor(environmentId) {
    const deadline = Date.now() + 15000;
    while (!api.signal.aborted && Date.now() < deadline) {
      const snapshot = registry.get(shell.stateValueAtom(environmentId))?.snapshot;
      if (snapshot?._tag === 'Some') return snapshot.value;
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    if (api.signal.aborted) throw new DOMException('Extension was reloaded', 'AbortError');
    throw new Error(`T3 adapter: environment ${environmentId} had no project list within 15 seconds. Check its connection in T3, then reload scripts.`);
  }
  async function context() {
    const route = new URL(location.hash.slice(1) || '/', 'https://t3.invalid');
    if (route.pathname.startsWith('/settings')) return null;
    const [, environmentId, encodedThreadId] = route.pathname.split('/');
    if (!environmentId || !encodedThreadId) return null;
    const threadId = decodeURIComponent(encodedThreadId);
    const snapshot = await snapshotFor(environmentId);
    const thread = snapshot.threads.find(thread => thread.id === threadId);
    const project = snapshot.projects.find(project => project.id === thread?.projectId);
    setHostProbe('route:thread', Boolean(project), project ? `#/${environmentId}/<thread>` : `#/${environmentId}/${threadId} names no thread with a project in that environment`);
    if (!project) return null;
    return { environmentId, threadId, projectId: project.id, projectName: project.title, repositoryKey: project.repositoryIdentity?.canonicalKey ?? null };
  }
  return {
    // null means the surfaces that need this hook are off; hostProbes says why.
    context: registry && shell ? context : null,
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
