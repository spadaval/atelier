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
    'thread-snapshot-atom': value => ['snapshotAtom', 'familyAtom', 'mountRows'].every(key => typeof value[key] === 'function'),
    'environment-atoms': value => ['environmentIdsAtom', 'connectedEnvironmentIdsAtom'].every(key => key in value),
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
  const threads = found['thread-snapshot-atom'];
  const environments = found['environment-atoms'];

  // Reads T3's own list of the environment's projects and threads, with the rows T3 loads
  // separately for an older or child thread merged in. It needs no request, so it works
  // whichever way T3 authenticates to that environment.
  async function context() {
    const hash = location.hash;
    const route = new URL(hash.slice(1) || '/', 'https://t3.invalid');
    if (route.pathname.startsWith('/settings')) return null;
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
  return {
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
