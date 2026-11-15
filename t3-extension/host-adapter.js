// This file is the only client code coupled to T3's private module exports.
async function createHostAdapter(api) {
  // Electron can omit cached module loads from resource timings after Cmd+R.
  // Vite's modulepreload links still identify the modules in that document.
  const resources = () => [
    ...document.querySelectorAll('link[rel="modulepreload"]'),
  ].map(link => link.href).concat(performance.getEntriesByType('resource').map(entry => entry.name));
  async function moduleStarting(prefix) {
    const deadline = Date.now() + 15000;
    while (!api.signal.aborted && Date.now() < deadline) {
      const url = resources().find(url => url.split('/').pop().startsWith(prefix) && url.endsWith('.js') && !(prefix === 'session-' && url.includes('/session-logic-')));
      if (url) return import(url);
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    if (api.signal.aborted) throw new DOMException('Extension was reloaded', 'AbortError');
    throw new Error(`T3 adapter: no loaded ${prefix} module after 15 seconds. The installed T3 build may have changed.`);
  }
  const panelModule = await moduleStarting('terminalFocus-');
  const panelStore = Object.values(panelModule).find(value => typeof value?.getState === 'function' && typeof value.getState().openBrowser === 'function');
  if (!panelStore?.subscribe || !panelStore.getState().show) throw new Error('T3 adapter: workspace-panel store is not compatible with this build.');
  const session = await moduleStarting('session-');
  const connections = Object.values(session).find(value => value?.preparedConnectionValueAtom);
  const utils = await moduleStarting('utils-');
  const registry = utils.Ol;
  if (!connections || !registry?.get) throw new Error('T3 adapter: the prepared-connection reader is not available.');
  async function connectionFor(environmentId) {
    const deadline = Date.now() + 15000;
    while (!api.signal.aborted && Date.now() < deadline) {
      const prepared = registry.get(connections.preparedConnectionValueAtom(environmentId));
      // Option is deliberately unwrapped here; credentials stay inside T3's renderer.
      const connection = prepared?.value ?? prepared;
      if (connection?.httpBaseUrl) return connection;
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    if (api.signal.aborted) throw new DOMException('Extension was reloaded', 'AbortError');
    throw new Error(`T3 adapter: environment ${environmentId} did not connect within 15 seconds. Check its connection in T3, then reload scripts.`);
  }
  const shells = new Map();
  async function context() {
    const route = new URL(location.hash.slice(1) || '/', 'https://t3.invalid');
    if (route.pathname.startsWith('/settings')) return null;
    const [, environmentId, encodedThreadId] = route.pathname.split('/');
    if (!environmentId || !encodedThreadId) return null;
    const threadId = decodeURIComponent(encodedThreadId);
    let cached = shells.get(environmentId);
    if (!cached || Date.now() - cached.at > 30000 || !cached.value.threads.some(thread => thread.id === threadId)) {
      const connection = await connectionFor(environmentId);
      const headers = { 'x-t3-orchestration-protocol': '2' };
      if (connection.httpAuthorization?.token) headers.Authorization = `Bearer ${connection.httpAuthorization.token}`;
      const url = connection.httpBaseUrl.replace(/\/$/, '') + '/api/orchestration/shell';
      const response = await fetch(url, { headers, signal: api.signal });
      if (!response.ok) throw new Error(`Read T3 project metadata for ${environmentId}: HTTP ${response.status}`);
      const value = await response.json();
      cached = { at: Date.now(), value }; shells.set(environmentId, cached);
    }
    const thread = cached.value.threads.find(thread => thread.id === threadId);
    const project = cached.value.projects.find(project => project.id === thread?.projectId);
    if (!project) return null;
    return { environmentId, threadId, projectId: project.id, projectName: project.title, repositoryKey: project.repositoryIdentity?.canonicalKey ?? null };
  }
  return {
    context,
    panelState: context => panelStore.getState().byThreadKey[`${context.environmentId}:${context.threadId}`],
    showPanel: context => panelStore.getState().show({ environmentId: context.environmentId, threadId: context.threadId }),
    subscribe: listener => panelStore.subscribe(listener),
    openSettings() { location.hash = '/settings/general'; },
  };
}
