// T3 extension UI. Desk remains the backend; no T3 credentials enter its iframe.
// The Desk is shown in its embedded mode and talks to this page over its host contract
// (atelier: docs/architecture/desk.md, "The embedded mode" and "The host contract").
const deskUrls = api.config.urls;
// Manual repository overrides from config.json: extra repository keys or a different slug.
const overrides = api.config.repositories;
// The settings the Customizations section edits live in config.json; saving one rewrites it
// through the injector, which then reapplies the extension.
const settings = { enabled: true, references: true, followProject: true, pinnedRepo: null, mappings: {}, ...api.config.settings };
// localStorage keeps only view state: which threads had the Desk open.
const viewKey = 't3-custom:atelier:view:v1';
let view;
try { view = JSON.parse(localStorage.getItem(viewKey) ?? '{}'); } catch { view = {}; }
view = { openThreads: {}, ...view };
const saveView = () => localStorage.setItem(viewKey, JSON.stringify(view));
const disposers = [];
const listen = (node, name, callback, options = {}) => node.addEventListener(name, callback, { ...options, signal: api.signal });
function element(tag, attributes = {}, ...children) {
  const node = document.createElement(tag);
  for (const [name, value] of Object.entries(attributes)) {
    if (name === 'text') node.textContent = value;
    else if (name === 'class') node.className = value;
    else if (name === 'html') node.innerHTML = value;
    else node.setAttribute(name, String(value));
  }
  node.append(...children); return node;
}
// Lucide icons, as T3 draws its own panel controls.
const icon = paths => `<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${paths}</svg>`;
const icons = {
  close: icon('<path d="M18 6 6 18"/><path d="m6 6 12 12"/>'),
  refresh: icon('<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/>'),
  external: icon('<path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>'),
  settings: icon('<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>'),
  desk: icon('<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M9 21V9"/>'),
};
const css = `
  :host { color: var(--foreground, #eee); font: 13px var(--font-sans, system-ui, sans-serif); }
  * { box-sizing: border-box; } [hidden] { display: none !important; }
  button, input, select, a { font: inherit; } button, select { color: inherit; }
  button { cursor: pointer; background: var(--secondary, #303238); border: 1px solid var(--border, #60636a); border-radius: 6px; padding: 6px 10px; }
  button:hover { background: var(--accent, #41444b); } button:disabled { opacity: .5; cursor: default; }
  button:focus-visible, a:focus-visible, select:focus-visible { outline: 2px solid var(--ring, #86b5ff); outline-offset: 2px; }
  a { color: var(--primary, #8ab4ff); } h2 { margin: 0; font-size: 16px; } h3 { margin: 0; font-size: 14px; }
  p { margin: 0; line-height: 1.5; } .muted { color: var(--muted-foreground, #b1b4bc); }
  .row { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
  .stack { display: flex; flex-direction: column; gap: 12px; } .grow { flex: 1; min-width: 0; }
  .error { color: var(--destructive, #ff9292); white-space: pre-wrap; overflow-wrap: anywhere; }
  label { display: flex; align-items: center; gap: 8px; line-height: 1.5; }
  select { max-width: 100%; background: var(--background, #202124); border: 1px solid var(--border, #60636a); padding: 6px; border-radius: 6px; }
  code { overflow-wrap: anywhere; font-size: 12px; }
  ul { margin: 0; padding-left: 18px; font-size: 12px; line-height: 1.6; } .missing { color: var(--destructive, #ff9292); }
`;
const addStyle = root => root.append(element('style', { text: css }));
addStyle(api.root);
// The Desk tab and its controls copy T3's own panel tab and icon-button metrics and colours.
const globalStyle = element('style', { text: `
  [data-t3-desk-tab] { display: flex; align-items: center; gap: 2px; height: 24px; max-width: 12rem; flex-shrink: 0; padding: 0 8px 0 6px; border-radius: 6px; font-size: 12px; color: var(--muted-foreground); -webkit-app-region: no-drag; }
  [data-t3-desk-tab][data-selected] { background: var(--accent); color: var(--foreground); }
  [data-t3-desk-tab] > [data-t3-desk-label] { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; background: none; border: 0; padding: 0; color: inherit; font: inherit; cursor: pointer; }
  [data-t3-desk-icon], [data-t3-desk-launch] { display: inline-flex; align-items: center; justify-content: center; gap: 4px; flex-shrink: 0; min-width: 24px; height: 24px; padding: 0 4px; border: 0; border-radius: 6px; background: transparent; color: var(--muted-foreground); font: inherit; font-size: 12px; cursor: pointer; text-decoration: none; -webkit-app-region: no-drag; }
  [data-t3-desk-tab] > [data-t3-desk-icon] { min-width: 16px; height: 16px; padding: 0; }
  [data-t3-desk-launch] { height: 28px; padding: 0 6px; color: var(--foreground); }
  [data-t3-desk-icon]:hover, [data-t3-desk-launch]:hover { background: var(--accent); color: var(--foreground); }
  [data-t3-desk-controls] { display: flex; align-items: center; gap: 2px; flex-shrink: 0; }
  :is([data-t3-desk-tab], [data-t3-desk-controls]) :focus-visible, [data-t3-desk-launch]:focus-visible { outline: 2px solid var(--ring); outline-offset: -2px; }
  [data-t3-desk-selected] > :not([data-t3-desk-mount]) { visibility: hidden !important; }
  [data-t3-desk-tabbar] ${hostSelectors.panelTab} { background: transparent !important; color: var(--muted-foreground) !important; }
  ::highlight(t3-atelier-refs) { color: var(--primary, #8ab4ff); text-decoration: underline; }
` });
document.head.append(globalStyle);
api.onCleanup(() => { globalStyle.remove(); for (const dispose of disposers.reverse()) dispose(); });

let host;
let hostError = null;
let current = null;
if (location.hash.includes('/settings')) {
  try { current = JSON.parse(sessionStorage.getItem(viewKey + ':context') ?? 'null'); } catch {}
}
let routeSeen = '';
let contextGeneration = 0;
let lastProject = null;
let referenceOverride = null;
let nativeActiveId;
let desiredOpen = false;
let domFrame = 0;
let scanTimer;
let focusSettings = false;
const projectKey = context => context ? `${context.environmentId}:${context.projectId}` : '';
const threadKey = context => context ? `${context.environmentId}:${context.threadId}` : '';

// Repositories as the Desks report them (GET /api/repos), each with the Desk that serves it.
// A repository matches a T3 project when its identity equals the project's repositoryIdentity.canonicalKey.
let repositories = [];
const deskStatus = new Map(deskUrls.map(desk => [desk, { loading: true }]));
async function loadRepositories() {
  await Promise.all(deskUrls.map(async desk => {
    try { deskStatus.set(desk, { repos: (await api.request('desk-get', { desk, path: '/api/repos' })).repos }); }
    catch (error) { deskStatus.set(desk, { error }); }
  }));
  if (api.signal.aborted) return;
  repositories = deskUrls.flatMap(desk => (deskStatus.get(desk).repos ?? []).map(repo => {
    const override = overrides.find(entry => entry.repo === repo.name);
    return { repo: repo.name, identity: repo.identity, slug: override?.slug ?? repo.name, keys: [repo.identity, ...(override?.repositoryKeys ?? [])].filter(Boolean), desk };
  }));
  buildPattern(); scheduleDOM(); queueScan();
}
const repositoryNamed = name => repositories.find(repo => repo.repo === name) ?? null;
function mappedRepository() {
  const key = projectKey(current);
  if (Object.hasOwn(settings.mappings, key)) return repositoryNamed(settings.mappings[key]);
  return repositories.find(repo => repo.keys.includes(current?.repositoryKey)) ?? null;
}
function selectedRepository() {
  if (referenceOverride?.projectKey === projectKey(current)) return repositoryNamed(referenceOverride.repo);
  return settings.followProject ? mappedRepository() : repositoryNamed(settings.pinnedRepo) ?? mappedRepository() ?? repositories[0] ?? null;
}
async function saveSettings() {
  try { await api.request('save-settings', settings); } catch (error) { displayError(error); }
}
// The Desk panel needs T3's panel store, project context and a slot to mount in;
// references need only the conversation DOM and otherwise open Desk externally.
const deskHooks = ['module:workspace-panel-store', 'module:atom-registry', 'module:thread-snapshot-atom', 'dom:panel', 'dom:panelContent'];
const deskAvailable = () => Boolean(host?.panel && host?.context) && !deskHooks.some(name => hostProbes[name]?.ok === false);
globalThis.__t3Custom.diagnostics = () => ({ probes: hostProbes, modules: hostImportedModules, context: current, repositories, mappedRepository: mappedRepository()?.repo ?? null, selectedRepository: selectedRepository()?.repo ?? null, referenceOverride, desiredOpen, deskAvailable: deskAvailable(), frame: { desk: frameDesk, path: framePath, title: deskTitle, ready: deskReady } });

function openSettings() { focusSettings = true; location.hash = '/settings/general'; scheduleDOM(); }

const mount = element('div', { 'data-t3-desk-mount': '', hidden: '' });
mount.style.cssText = 'position:absolute;inset:0;z-index:1;min-width:0;min-height:0';
const panelRoot = mount.attachShadow({ mode: 'open' }); addStyle(panelRoot);
panelRoot.append(element('style', { text: ':host{display:block;height:100%}.panel{height:100%;min-height:0;display:flex;flex-direction:column;background:var(--background,#202124)}iframe{flex:1;min-height:0;width:100%;border:0;background:var(--background,#202124)}.empty{padding:20px;max-width:55ch}.notice,.panel>.error{padding:6px 10px;font-size:12px;border-bottom:1px solid var(--border,#60636a)}' }));
const notice = element('p', { class: 'notice muted', role: 'status', hidden: '' });
const panelError = element('p', { class: 'error', hidden: '' });
const iframe = element('iframe', { title: 'Atelier Desk' });
const linkProject = element('button', { text: 'Link project in Customizations' });
const empty = element('div', { class: 'empty stack' }, element('h3', { text: 'No Atelier repository linked' }), element('p', { class: 'muted', text: 'No Desk serves a repository with this project\'s identity. Choose one for this T3 project in Customizations.' }), linkProject);
panelRoot.append(element('div', { class: 'panel' }, notice, panelError, empty, iframe));
let noticeTimer;
function notify(text) {
  notice.textContent = text; notice.hidden = false;
  clearTimeout(noticeTimer); noticeTimer = setTimeout(() => { notice.hidden = true; }, 8000);
}
const nativeInert = new Map();
const positions = new Map();
const areaLabels = new Map();
const markedTabbars = new Set();
let surfaceContent;
let iframeRepository = null;
// The Desk shown in the iframe, its route without /embed (from desk:navigated once it is ready),
// and its page title.
let frameDesk = null;
let framePath = null;
let deskTitle = '';
let deskReady = false;
function restoreNative() {
  for (const [node, inert] of nativeInert) node.inert = inert;
  nativeInert.clear(); surfaceContent?.removeAttribute('data-t3-desk-selected');
  for (const tabbar of markedTabbars) tabbar.removeAttribute('data-t3-desk-tabbar');
  markedTabbars.clear();
  for (const [area, label] of areaLabels) label === null ? area.removeAttribute('aria-label') : area.setAttribute('aria-label', label);
  areaLabels.clear();
}
const repositoryPath = repo => `/r/${encodeURIComponent(repo.repo)}`;
const recordPath = record => `/r/${encodeURIComponent(record.repo)}/${record.kind === 'mission' ? 'mission' : 'item'}/${encodeURIComponent(record.id)}`;
// The panel loads the embedded mode; Open externally keeps the normal address.
function navigateDesk(repo, path = repositoryPath(repo)) {
  if (frameDesk !== repo.desk || framePath !== path) loadFrame(repo.desk, path);
  iframeRepository = repo.repo;
}
function loadFrame(desk, path) {
  frameDesk = desk; framePath = path; deskReady = false; deskTitle = ''; sentTheme = '';
  iframe.src = `${desk}/embed${path}`;
  scheduleDOM();
}
async function showDesk(reference) {
  if (!settings.enabled) { settings.enabled = true; void saveSettings(); }
  if (!current || !deskAvailable()) { openSettings(); return; }
  if (reference) {
    referenceOverride = { projectKey: projectKey(current), repo: reference.repo };
    navigateDesk(repositoryNamed(reference.repo), recordPath(reference));
  }
  desiredOpen = true; view.openThreads[threadKey(current)] = true; saveView();
  if (location.hash.includes('/settings')) location.hash = `/${current.environmentId}/${encodeURIComponent(current.threadId)}`;
  scheduleDOM();
  // A stop or reinjection during the wait must not open anything afterwards.
  if (await host.panel.show(current) || api.signal.aborted) return;
  // T3's panel did not render a slot (the probe now names the missing hook): use the browser.
  hideDesk();
  const repo = selectedRepository();
  window.open(reference ? referenceUrl(reference) : repo ? repo.desk + repositoryPath(repo) : deskUrls[0], '_blank', 'noopener');
}
function hideDesk() {
  desiredOpen = false;
  if (current) { view.openThreads[threadKey(current)] = false; saveView(); }
  scheduleDOM();
}
listen(linkProject, 'click', openSettings);
listen(iframe, 'error', () => { panelError.hidden = false; panelError.textContent = `Could not load ${iframe.src}. Check connectivity to ${frameDesk} and use Refresh.`; });

// The host contract. Messages count only from the Desk iframe's window and the Desk's origin,
// and every message to it names that origin as its target.
const deskOrigin = () => new URL(frameDesk).origin;
let sentTheme = '';
function sendTheme(always) {
  if (!deskReady) return;
  const theme = readHostTheme(api.root);
  const text = JSON.stringify(theme);
  if (!always && text === sentTheme) return;
  sentTheme = text;
  iframe.contentWindow.postMessage({ type: 'host:theme', ...theme }, deskOrigin());
}
disposers.push(observeHostTheme(() => sendTheme(false)));
// Seam: the Desk's contract is gaining a message that tells it which thread T3 shows. Send it
// from here once the contract defines it; this is called when the Desk becomes ready and when
// T3's current thread changes. Until then nothing is sent.
function tellDeskThread() {}
function openThread(threadId) {
  if (!host?.findThread) { notify(`Cannot open thread ${threadId}: T3 hook module:environment-atoms is missing (see Customizations).`); return; }
  const environmentId = host.findThread(threadId);
  if (!environmentId) { notify(`Thread ${threadId} is not in any connected T3 environment.`); return; }
  host.showThread(environmentId, threadId);
}
listen(window, 'message', event => {
  if (!frameDesk || event.source !== iframe.contentWindow || event.origin !== deskOrigin()) return;
  const message = event.data;
  if (!message || typeof message !== 'object') return;
  if (message.type === 'desk:ready') { deskReady = true; sendTheme(true); tellDeskThread(); }
  else if (message.type === 'desk:navigated' && typeof message.path === 'string' && message.path.startsWith('/')) {
    framePath = message.path; deskTitle = String(message.title ?? ''); scheduleDOM();
  } else if (message.type === 'desk:open-thread' && typeof message.thread === 'string') openThread(message.thread);
});

// The Desk tab sits in T3's tab strip like a native one: close icon, then the Desk's page title.
// Its controls (Refresh, Open externally, Customizations) follow it while the Desk is shown.
const closeDesk = element('button', { 'data-t3-desk-icon': '', 'aria-label': 'Close Desk', title: 'Close Desk', html: icons.close });
const tabLabel = element('button', { 'data-t3-desk-label': '', 'aria-pressed': 'false', text: 'Desk' });
const tab = element('div', { 'data-t3-desk-tab': '' }, closeDesk, tabLabel);
const refreshDesk = element('button', { 'data-t3-desk-icon': '', 'aria-label': 'Refresh Desk', title: 'Refresh Desk', html: icons.refresh });
const externalDesk = element('a', { 'data-t3-desk-icon': '', 'data-t3-desk-external': '', 'aria-label': 'Open Desk externally', title: 'Open externally', target: '_blank', rel: 'noopener noreferrer', html: icons.external });
const settingsDesk = element('button', { 'data-t3-desk-icon': '', 'aria-label': 'Atelier customizations', title: 'Customizations', html: icons.settings });
const controls = element('div', { 'data-t3-desk-controls': '', hidden: '' }, refreshDesk, externalDesk, settingsDesk);
const launch = element('button', { 'data-t3-desk-launch': '', 'aria-label': 'Open Atelier Desk', title: 'Atelier Desk', html: `${icons.desk}<span>Desk</span>` });
listen(tabLabel, 'click', () => desiredOpen ? hideDesk() : void showDesk());
listen(closeDesk, 'click', hideDesk);
listen(refreshDesk, 'click', () => { if (frameDesk) loadFrame(frameDesk, framePath); });
listen(settingsDesk, 'click', openSettings);
listen(launch, 'click', () => void showDesk());
listen(document, 'click', event => {
  const target = event.target instanceof Element ? event.target : null;
  if (target?.closest(`${hostSelectors.panelTabbar} ${hostSelectors.panelTab}`)) hideDesk();
}, { capture: true });
function syncPanel() {
  observeHostDom();
  const area = document.querySelector(hostSelectors.panel);
  const content = area?.querySelector(hostSelectors.panelContent);
  const tabbar = area?.querySelector(hostSelectors.panelTabbar);
  const add = tabbar?.querySelector(hostSelectors.panelAddButton);
  if (add && tab.parentElement !== add.parentElement) add.before(tab, controls);
  const toolbar = [...document.querySelectorAll(hostSelectors.layoutControls)].find(node => node.getBoundingClientRect().width > 0);
  if (toolbar && launch.parentElement !== toolbar) toolbar.prepend(launch);
  const allowed = settings.enabled && deskAvailable() && Boolean(current) && !location.hash.includes('/settings');
  tab.hidden = launch.hidden = !allowed;
  const state = allowed && host.panel.state(current);
  const visible = allowed && desiredOpen && state?.isOpen && Boolean(content) && area.getBoundingClientRect().width > 0;
  tabLabel.setAttribute('aria-pressed', String(Boolean(visible)));
  tab.toggleAttribute('data-selected', Boolean(visible));
  controls.hidden = !visible;
  if (!visible) { mount.hidden = true; restoreNative(); return; }
  if (surfaceContent !== content) { restoreNative(); surfaceContent = content; }
  if (!positions.has(content)) { positions.set(content, content.style.position); if (getComputedStyle(content).position === 'static') content.style.position = 'relative'; }
  if (mount.parentElement !== content) content.append(mount);
  content.setAttribute('data-t3-desk-selected', '');
  tabbar?.setAttribute('data-t3-desk-tabbar', ''); if (tabbar) markedTabbars.add(tabbar);
  for (const child of content.children) if (child !== mount && !nativeInert.has(child)) { nativeInert.set(child, child.inert); child.inert = true; }
  mount.hidden = false;
  const repo = selectedRepository();
  empty.hidden = Boolean(repo); iframe.hidden = refreshDesk.hidden = externalDesk.hidden = !repo;
  if (repo && iframeRepository !== repo.repo) navigateDesk(repo);
  const label = repo && deskTitle ? deskTitle : 'Desk';
  tabLabel.textContent = label; tab.title = repo ? `Atelier Desk: ${deskTitle || repo.repo}` : 'Atelier Desk';
  if (frameDesk) externalDesk.href = frameDesk + framePath;
  if (!areaLabels.has(area)) areaLabels.set(area, area.getAttribute('aria-label'));
  area.setAttribute('aria-label', `Workspace panel: Atelier Desk${deskTitle ? `, ${deskTitle}` : ''}`);
}

const settingsHost = element('section', { id: 't3-custom-settings', 'data-t3-custom-settings': '' });
settingsHost.style.cssText = 'display:block;margin:24px 0;padding:20px 0;border-top:1px solid var(--border)';
const settingsRoot = settingsHost.attachShadow({ mode: 'open' }); addStyle(settingsRoot);
const statusText = element('p', { class: 'muted', text: 'Reading injector status…' });
const statusError = element('p', { class: 'error', hidden: '' });
const deskList = element('ul', { 'aria-label': 'Desks' });
const contextText = element('p', { class: 'muted' });
const overrideList = element('ul', { 'aria-label': 'Manual repository overrides' });
const probeList = element('ul', { 'aria-label': 'T3 hooks used by this extension' });
const mapping = element('select', { id: 'project-mapping', 'aria-label': 'Atelier repository for current T3 project' });
const pinned = element('select', { id: 'pinned-repository', 'aria-label': 'Pinned Atelier repository' });
const toggles = {};
function checkbox(key, title) {
  const input = toggles[key] = element('input', { type: 'checkbox', 'data-setting': key }); input.checked = settings[key];
  listen(input, 'change', () => { settings[key] = input.checked; referenceOverride = null; void saveSettings(); syncSettings(); scheduleDOM(); queueScan(); });
  return element('label', {}, input, document.createTextNode(title));
}
const reloadScripts = element('button', { id: 'reload-scripts', text: 'Reload scripts' });
const openFolder = element('button', { text: 'Open customization folder' });
const refreshStatus = element('button', { text: 'Refresh status' });
const deskAction = element('button', { text: 'Open Desk' });
settingsRoot.append(element('div', { class: 'stack' }, element('h2', { text: 'Customizations' }), statusText, statusError,
  element('div', { class: 'row' }, reloadScripts, refreshStatus, openFolder, deskAction),
  element('h3', { text: 'Desks' }), deskList,
  element('h3', { text: 'Settings' }), element('p', { class: 'muted', text: `Saved in ${api.home}${api.home.includes('\\') ? '\\' : '/'}config.json` }),
  checkbox('enabled', 'Enable the Atelier Desk surface'), checkbox('references', 'Show Atelier reference links and hovercards'), checkbox('followProject', 'Follow the current T3 project'),
  contextText, element('label', {}, document.createTextNode('Project repository'), mapping), element('label', { id: 'pinned-row' }, document.createTextNode('Pinned repository'), pinned),
  element('h3', { text: 'Manual overrides' }), overrideList,
  element('h3', { text: 'T3 hooks' }), probeList));
listen(reloadScripts, 'click', () => api.reloadScripts());
listen(openFolder, 'click', () => void api.request('open-folder').catch(error => displayError(error)));
listen(refreshStatus, 'click', () => { void readStatus(); void loadRepositories(); });
listen(deskAction, 'click', () => void showDesk());
// Automatic follows the Desk's repository identity; anything else is a manual override.
listen(mapping, 'change', () => {
  if (!current) return;
  if (mapping.value === '@automatic') delete settings.mappings[projectKey(current)];
  else settings.mappings[projectKey(current)] = mapping.value;
  referenceOverride = null; void saveSettings(); scheduleDOM();
});
listen(pinned, 'change', () => { settings.pinnedRepo = pinned.value; referenceOverride = null; void saveSettings(); scheduleDOM(); });
function displayError(error) { statusError.hidden = false; statusError.textContent = error.message ?? String(error); }
async function readStatus() {
  try {
    const status = await api.request('status');
    if (api.signal.aborted) return;
    statusText.textContent = `Injector ${status.phase} · version ${status.version} · last applied ${status.lastAppliedAt ? new Date(status.lastAppliedAt).toLocaleTimeString() : 'not yet'}`;
    statusError.hidden = !status.lastError && !hostError;
    statusError.textContent = hostError?.message ?? status.lastError ?? '';
  } catch (error) { if (!api.signal.aborted) displayError(error); }
}
function syncProbes() {
  const missing = name => hostProbes[name]?.ok === false;
  const deskMissing = deskHooks.filter(missing);
  const rows = Object.entries(hostProbes).map(([name, probe]) => element('li', { class: probe.ok === false ? 'missing' : '', text: `${probe.ok ? '✓' : probe.ok === false ? '✗' : '…'} ${name}: ${probe.detail}` }));
  rows.unshift(element('li', { class: deskMissing.length ? 'missing' : '', text: host ? `Desk panel: ${deskMissing.length ? `off, missing ${deskMissing.join(', ')}` : 'on'}` : 'Desk panel: looking for T3 hooks…' }));
  if (host) rows.splice(1, 0, element('li', { class: host.findThread ? '' : 'missing', text: `Opening threads from the Desk: ${host.findThread ? 'on' : 'off, missing module:environment-atoms'}` }));
  probeList.replaceChildren(...rows);
}
const options = (select, entries) => {
  const value = select.value;
  select.replaceChildren(...entries.map(([key, text]) => element('option', { value: key, text })));
  select.value = value;
};
function syncSettings() {
  syncProbes();
  deskList.replaceChildren(...deskUrls.map(desk => {
    const status = deskStatus.get(desk);
    const text = status.loading ? 'reading…' : status.error ? `unreachable: ${status.error.message}`
      : `reachable, ${status.repos.length} repositories: ${status.repos.map(repo => `${repo.name} (${repo.identity ?? 'no identity'})`).join(', ')}`;
    return element('li', { class: status.error ? 'missing' : '', text: `${desk}: ${text}` });
  }));
  const automatic = current && repositories.find(repo => repo.keys.includes(current.repositoryKey));
  options(mapping, [['@automatic', `Automatic: ${automatic ? automatic.repo : 'no Desk repository has this identity'}`], ['', 'Not linked'], ...repositories.map(repo => [repo.repo, repo.repo])]);
  options(pinned, repositories.map(repo => [repo.repo, repo.repo]));
  contextText.textContent = current ? `Current project: ${current.projectName}${current.repositoryKey ? ` (${current.repositoryKey})` : ' (no repository identity)'}` : 'Open a T3 thread to link its project.';
  mapping.disabled = !current;
  mapping.value = current && Object.hasOwn(settings.mappings, projectKey(current)) ? settings.mappings[projectKey(current)] : '@automatic';
  pinned.value = selectedRepository()?.repo ?? repositories[0]?.repo ?? '';
  settingsRoot.querySelector('#pinned-row').hidden = settings.followProject;
  for (const [key, input] of Object.entries(toggles)) input.checked = settings[key];
  const rows = Object.entries(settings.mappings).map(([key, repo]) => {
    const remove = element('button', { text: 'Remove', 'aria-label': `Remove override for ${key}` });
    remove.addEventListener('click', () => { delete settings.mappings[key]; void saveSettings(); syncSettings(); scheduleDOM(); });
    return element('li', { class: 'row' }, element('span', { text: `Project ${key} → ${repo || 'not linked'}` }), remove);
  });
  for (const entry of overrides) rows.push(element('li', { text: `config.json desk.repositories: ${entry.repo}${entry.slug ? `, slug ${entry.slug}` : ''}${entry.repositoryKeys?.length ? `, keys ${entry.repositoryKeys.join(', ')}` : ''}` }));
  overrideList.replaceChildren(...(rows.length ? rows : [element('li', { class: 'muted', text: 'None: projects follow the repository identity the Desk reports.' })]));
  deskAction.disabled = !current || !host;
  const target = location.hash.startsWith('#/settings/general') && document.querySelector(hostSelectors.settingsPage);
  if (target) {
    const container = target.firstElementChild ?? target;
    if (settingsHost.parentElement !== container) { container.append(settingsHost); void readStatus(); }
    if (focusSettings) { focusSettings = false; settingsHost.scrollIntoView({ block: 'start' }); }
  } else settingsHost.remove();
}

async function updateContext() {
  const generation = ++contextGeneration;
  if (location.hash.includes('/settings')) { scheduleDOM(); return; }
  current = null; desiredOpen = false; scheduleDOM();
  if (!host.context) return;
  try {
    const next = await host.context();
    if (api.signal.aborted || generation !== contextGeneration) return;
    current = next;
    if (current) sessionStorage.setItem(viewKey + ':context', JSON.stringify(current));
    const key = projectKey(current);
    if (key !== lastProject) { referenceOverride = null; lastProject = key; }
    desiredOpen = Boolean(view.openThreads[threadKey(current)]);
    nativeActiveId = current && (host.panel?.state(current)?.activeSurfaceId ?? null);
    tellDeskThread();
    scheduleDOM(); queueScan();
  } catch (error) { if (!api.signal.aborted && generation === contextGeneration) { hostError = error; displayError(error); api.reportError(error); } }
}
function scheduleDOM() {
  if (domFrame || api.signal.aborted) return;
  // Electron suspends animation frames while its window is hidden.
  domFrame = setTimeout(() => { domFrame = 0; syncPanel(); syncSettings(); }, 0);
}

// Ranges and transparent keyboard links leave React's message text untouched.
const referenceLayer = element('div', { id: 'reference-links' });
api.root.append(referenceLayer);
const card = element('div', { id: 'atelier-hovercard', popover: 'manual', role: 'dialog', 'aria-label': 'Atelier reference details', class: 'stack' });
card.style.cssText = 'position:fixed;inset:auto;margin:0;width:360px;max-width:calc(100vw - 24px);max-height:70vh;overflow:auto;padding:14px;border:1px solid var(--border,#666);border-radius:8px;background:var(--popover,var(--background,#202124));color:var(--foreground,#eee);font:13px var(--font-sans,system-ui,sans-serif)';
api.root.append(card);
let references = []; let hoverTimer; let dismissTimer; let activeReference = null; let pointerOverCard = false; let rectFrame = 0;
const referenceCache = new Map();
let pattern = null;
// A reference is a repository's slug, a dash and a lowercase-letter/digit suffix. The slug is the
// Desk's repository name unless config.json's desk.repositories gives another.
function buildPattern() {
  const escape = value => value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const slugs = [...new Set(repositories.map(repo => repo.slug))].sort((a, b) => b.length - a.length);
  pattern = slugs.length ? new RegExp(`(?<![\\w-])(?:${slugs.map(escape).join('|')})-[a-z0-9]+(?![\\w-])`, 'g') : null;
}
// A reference's address before it is resolved names the repository its slug belongs to.
const referenceUrl = ref => (ref.desk ?? repositoryNamed(ref.repo)?.desk ?? deskUrls[0]) + recordPath(ref);
function hideCard() {
  clearTimeout(hoverTimer); clearTimeout(dismissTimer); activeReference = null; pointerOverCard = false;
  if (card.matches(':popover-open')) card.hidePopover();
}
function dismissSoon() { clearTimeout(dismissTimer); dismissTimer = setTimeout(() => { if (!pointerOverCard && !card.contains(api.root.activeElement)) hideCard(); }, 200); }
listen(card, 'pointerenter', () => { pointerOverCard = true; clearTimeout(dismissTimer); });
listen(card, 'pointerleave', () => { pointerOverCard = false; dismissSoon(); });
listen(document, 'keydown', event => { if (event.key === 'Escape') hideCard(); });
function positionCard(ref) {
  const rect = ref.range.getBoundingClientRect();
  card.style.left = `${Math.max(12, Math.min(rect.left, innerWidth - 372))}px`;
  const height = card.getBoundingClientRect().height;
  card.style.top = `${Math.max(12, rect.bottom + height + 12 > innerHeight ? rect.top - height - 8 : rect.bottom + 8)}px`;
}
// One read resolves an ID: GET /api/records/{id} names its repository and kind. Each Desk is asked
// in the configured order until one has it. Cached for 30 seconds.
function cached(key, read) {
  let entry = referenceCache.get(key);
  if (!entry || Date.now() - entry.at > 30000) { entry = { at: Date.now(), promise: read() }; referenceCache.set(key, entry); }
  return entry.promise;
}
const readRecord = id => cached(`record:${id}`, async () => {
  for (const desk of deskUrls) {
    try { return { ...(await api.request('desk-get', { desk, path: `/api/records/${id}` })), desk }; }
    catch (error) { if (error.status !== 404) throw error; }
  }
  throw Object.assign(new Error(`No item or mission ${id} was found on ${deskUrls.join(', ')}.`), { status: 404 });
});
// The hovercard's further lines (priority, holder, checks, blockers, goal) come from the record's own route.
const readDetails = record => cached(`details:${record.desk}${recordPath(record)}`, () =>
  api.request('desk-get', { desk: record.desk, path: `/api/repos/${encodeURIComponent(record.repo)}/${record.kind === 'mission' ? 'missions' : 'items'}/${record.id}` }));
async function resolveReference(ref) {
  const record = await readRecord(ref.id);
  Object.assign(ref, { repo: record.repo, kind: record.kind, desk: record.desk });
  ref.anchor.href = referenceUrl(ref);
  return record;
}
async function activateReference(ref, external = false) {
  try {
    await resolveReference(ref);
    if (api.signal.aborted) return;
    hideCard();
    if (external || !deskAvailable() || !repositoryNamed(ref.repo)) window.open(referenceUrl(ref), '_blank', 'noopener');
    else await showDesk(ref);
  } catch { if (!api.signal.aborted) void showCard(ref); }
}
function forget(ref) { for (const key of referenceCache.keys()) if (key === `record:${ref.id}` || key.endsWith(`/${ref.id}`)) referenceCache.delete(key); }
async function showCard(ref) {
  clearTimeout(dismissTimer); activeReference = ref;
  card.replaceChildren(element('strong', { text: ref.id }), element('p', { class: 'muted', text: 'Loading from Desk…' }));
  if (!card.matches(':popover-open')) card.showPopover(); positionCard(ref);
  try {
    const record = await resolveReference(ref);
    if (api.signal.aborted || activeReference !== ref) return;
    const status = element('p', { text: `${record.kind} · ${record.status}` });
    const more = element('div', { class: 'stack' }, element('p', { class: 'muted', text: 'Reading details…' }));
    const open = element('button', { text: 'Open in Desk' }); open.addEventListener('click', () => void activateReference(ref));
    const refresh = element('button', { text: 'Refresh' }); refresh.addEventListener('click', () => { forget(ref); void showCard(ref); });
    const detail = element('div', { class: 'stack' }, element('div', { class: 'row' }, element('strong', { class: 'grow', text: ref.id }), element('span', { class: 'muted', text: record.repo })), element('h3', { text: record.title }),
      status, more, element('div', { class: 'row' }, open, refresh, element('a', { text: 'Open externally ↗', href: referenceUrl(ref), target: '_blank', rel: 'noopener noreferrer' })));
    card.replaceChildren(detail); ref.anchor.setAttribute('aria-label', `${ref.id}: ${record.title}`); positionCard(ref);
    try {
      const data = await readDetails(record);
      if (api.signal.aborted || activeReference !== ref) return;
      const row = record.kind === 'item' ? data.row : data.mission;
      status.textContent = [record.kind, record.status, row?.priority, row?.holder && `Held by ${row.holder}`].filter(Boolean).join(' · ');
      const lines = [];
      if (data.done_when?.length) lines.push(element('p', { class: 'muted', text: `${data.done_when.filter(line => line.ticked).length}/${data.done_when.length} completion checks` }));
      if (row?.waiting_on?.length || data.blocked_by?.length) lines.push(element('p', { text: `Blocked by ${(row.waiting_on?.length ? row.waiting_on : data.blocked_by).join(', ')}` }));
      if (data.goal) lines.push(element('p', { class: 'muted', text: data.goal.slice(0, 240) }));
      lines.push(element('p', { class: 'muted', text: `Read from ${data.read_from ?? 'Desk'} · cached up to 30 seconds` }));
      more.replaceChildren(...lines);
    } catch (error) { if (!api.signal.aborted) more.replaceChildren(element('p', { class: 'error', text: error.message })); }
    positionCard(ref);
  } catch (error) {
    if (api.signal.aborted || activeReference !== ref) return;
    const retry = element('button', { text: 'Retry' }); retry.addEventListener('click', () => { forget(ref); void showCard(ref); });
    card.replaceChildren(element('strong', { text: ref.id }), element('p', { class: 'error', text: error.message }), retry); positionCard(ref);
  }
}
function updateReferenceRects() {
  for (const ref of references) {
    ref.rects = [...ref.range.getClientRects()].filter(rect => rect.width && rect.height && rect.bottom >= 0 && rect.top < innerHeight);
    const rect = ref.rects[0]; ref.anchor.hidden = !rect;
    if (rect) ref.anchor.style.cssText = `position:fixed;left:${rect.left}px;top:${rect.top}px;width:${rect.width}px;height:${rect.height}px;pointer-events:none;z-index:30;border-radius:2px`;
  }
  if (activeReference) positionCard(activeReference);
}
function queueRects() { if (!rectFrame) rectFrame = requestAnimationFrame(() => { rectFrame = 0; updateReferenceRects(); }); }
function scanReferences() {
  if (api.signal.aborted) return;
  const previous = references; references = []; CSS.highlights?.delete('t3-atelier-refs');
  const conversation = document.querySelector(hostSelectors.conversation);
  if (!settings.references || !pattern || !conversation) { hideCard(); referenceLayer.replaceChildren(); return; }
  const existing = new Map();
  for (const ref of previous) {
    if (!existing.has(ref.node)) existing.set(ref.node, new Map());
    existing.get(ref.node).set(`${ref.offset}:${ref.id}`, ref);
  }
  const walker = document.createTreeWalker(conversation, NodeFilter.SHOW_TEXT, { acceptNode(node) {
    return node.parentElement?.closest('pre,a,button,input,textarea,script,style,svg,[contenteditable="true"]') ? NodeFilter.FILTER_REJECT : NodeFilter.FILTER_ACCEPT;
  } });
  const ranges = [];
  while (walker.nextNode()) {
    const node = walker.currentNode; pattern.lastIndex = 0;
    for (const match of node.textContent.matchAll(pattern)) {
      const slug = match[0].slice(0, match[0].lastIndexOf('-'));
      const repo = repositories.find(repo => repo.slug === slug); if (!repo) continue;
      const range = document.createRange(); range.setStart(node, match.index); range.setEnd(node, match.index + match[0].length); ranges.push(range);
      const retained = existing.get(node)?.get(`${match.index}:${match[0]}`);
      if (retained) { retained.range = range; references.push(retained); continue; }
      const ref = { repo: repo.repo, id: match[0], kind: 'item', node, offset: match.index, range, rects: [] };
      const anchor = element('a', { href: referenceUrl(ref), 'aria-label': `Preview Atelier reference ${ref.id}`, 'aria-haspopup': 'dialog', 'data-atelier-id': ref.id });
      ref.anchor = anchor; references.push(ref); referenceLayer.append(anchor);
      anchor.addEventListener('focus', () => void showCard(ref)); anchor.addEventListener('blur', dismissSoon);
      anchor.addEventListener('click', event => { event.preventDefault(); void activateReference(ref, event.metaKey || event.ctrlKey); });
    }
  }
  const kept = new Set(references);
  for (const ref of previous) if (!kept.has(ref)) { if (activeReference === ref) hideCard(); ref.anchor.remove(); }
  if (CSS.highlights) CSS.highlights.set('t3-atelier-refs', new Highlight(...ranges));
  updateReferenceRects();
}
function queueScan() { clearTimeout(scanTimer); scanTimer = setTimeout(scanReferences, 250); }
const hitReference = event => references.find(ref => ref.rects.some(rect => event.clientX >= rect.left && event.clientX <= rect.right && event.clientY >= rect.top && event.clientY <= rect.bottom));
let pointerReference = null;
listen(document, 'pointermove', event => {
  const ref = event.target instanceof Element && event.target.closest(hostSelectors.conversation) ? hitReference(event) : null;
  if (ref === pointerReference) return;
  pointerReference = ref; clearTimeout(hoverTimer);
  if (ref) { clearTimeout(dismissTimer); hoverTimer = setTimeout(() => void showCard(ref), 220); }
  else dismissSoon();
});
listen(document, 'click', event => {
  const ref = event.target instanceof Element && event.target.closest(hostSelectors.conversation) ? hitReference(event) : null;
  if (!ref || event.button !== 0 || !document.getSelection()?.isCollapsed) return;
  event.preventDefault(); event.stopPropagation();
  void activateReference(ref, event.metaKey || event.ctrlKey);
}, { capture: true });
listen(document, 'scroll', queueRects, { capture: true }); listen(window, 'resize', queueRects);
listen(document, 'visibilitychange', () => { scheduleDOM(); queueRects(); });

const observer = new MutationObserver(records => {
  scheduleDOM();
  if (records.some(record => (record.target.nodeType === Node.ELEMENT_NODE ? record.target : record.target.parentElement)?.closest(hostSelectors.conversation))) queueScan();
});
observer.observe(document.body, { childList: true, subtree: true, characterData: true });
disposers.push(() => {
  observer.disconnect(); clearTimeout(scanTimer); clearTimeout(hoverTimer); clearTimeout(dismissTimer); clearTimeout(domFrame); clearTimeout(noticeTimer); cancelAnimationFrame(rectFrame);
  hideCard(); restoreNative(); mount.remove(); tab.remove(); controls.remove(); launch.remove(); settingsHost.remove(); CSS.highlights?.delete('t3-atelier-refs');
  for (const [node, position] of positions) node.style.position = position;
});
const routeTimer = setInterval(() => { if (routeSeen !== location.hash) { routeSeen = location.hash; if (host) void updateContext(); else scheduleDOM(); } }, 300);
disposers.push(() => clearInterval(routeTimer));
listen(window, 'hashchange', () => { routeSeen = location.hash; if (host) void updateContext(); });
// The Customizations section and references do not wait for T3's hooks.
const repositoriesLoaded = loadRepositories();
scheduleDOM();
try {
  host = await createHostAdapter(api);
  if (host.panel) disposers.push(host.panel.subscribe(() => {
    const active = current && (host.panel.state(current)?.activeSurfaceId ?? null);
    if (active !== nativeActiveId) { nativeActiveId = active; hideDesk(); }
    scheduleDOM();
  }));
  await updateContext();
  if (hostError) throw hostError;
} catch (error) {
  if (!api.signal.aborted) { hostError = error; displayError(error); throw error; }
}
await repositoriesLoaded;
scheduleDOM(); queueScan();
api.log('Atelier extension ready', { desks: deskUrls, repositories: repositories.map(repo => repo.repo) });
