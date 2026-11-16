// T3 extension UI. Desk remains the backend; no T3 credentials enter its iframe.
// The Desk is shown in its embedded mode and talks to this page over its host contract
// (atelier: docs/architecture/desk.md, "The embedded mode" and "The host contract").
const deskUrls = api.config.urls;
// Manual repository overrides from config.json: extra repository keys or a different slug.
const overrides = api.config.repositories;
// The settings the Customizations section edits live in config.json; saving one rewrites it
// through the injector, which then reapplies the extension.
const settings = { enabled: true, references: true, followProject: true, autoUpdate: false, pinnedRepo: null, mappings: {}, ...api.config.settings };
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
    // Only for the constant icons below; anything from the Desk or T3 goes in as text.
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
  [data-t3-desk-tab] { display: flex; align-items: center; gap: 2px; height: 24px; max-width: 9rem; flex-shrink: 0; padding: 0 8px 0 6px; border-radius: 6px; font-size: 12px; color: var(--muted-foreground); -webkit-app-region: no-drag; }
  [data-t3-desk-tab][data-selected] { background: var(--accent); color: var(--foreground); }
  [data-t3-desk-tab] > [data-t3-desk-label] { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; background: none; border: 0; padding: 0; color: inherit; font: inherit; cursor: pointer; }
  [data-t3-desk-icon], [data-t3-desk-launch] { display: inline-flex; align-items: center; justify-content: center; gap: 4px; flex-shrink: 0; min-width: 24px; height: 24px; padding: 0 4px; border: 0; border-radius: 6px; background: transparent; color: var(--muted-foreground); font: inherit; font-size: 12px; cursor: pointer; text-decoration: none; -webkit-app-region: no-drag; }
  [data-t3-desk-tab] > [data-t3-desk-icon] { min-width: 16px; height: 16px; padding: 0; }
  [data-t3-desk-launch] { width: 28px; height: 28px; padding: 0; color: var(--foreground); }
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
// A repository is named by its Desk page, such as http://buildbot:3780/r/atelier, since two Desks
// may serve repositories of the same name. A bare name, as older settings hold, counts only with
// one Desk configured: with several it cannot say which Desk it meant.
const repositoryKey = repo => `${repo.desk}${repositoryPath(repo)}`;
const repositoryFor = value => repositories.find(repo => repositoryKey(repo) === value) ?? (deskUrls.length === 1 ? repositories.find(repo => repo.repo === value) : null) ?? null;
function mappedRepository(context = current) {
  const key = projectKey(context);
  if (Object.hasOwn(settings.mappings, key)) return repositoryFor(settings.mappings[key]);
  return repositories.find(repo => repo.keys.includes(context?.repositoryKey)) ?? null;
}
function selectedRepository() {
  if (referenceOverride?.projectKey === projectKey(current)) return repositoryFor(referenceOverride.repository);
  if (settings.followProject) return mappedRepository();
  // A pinned repository that cannot be found shows as not linked; it never falls back to another.
  return settings.pinnedRepo ? repositoryFor(settings.pinnedRepo) : mappedRepository() ?? repositories[0] ?? null;
}
async function saveSettings() {
  try { await api.request('save-settings', settings); } catch (error) { displayError(error); }
}
// The Desk panel needs T3's panel store, project context and a slot to mount in;
// references need only the conversation DOM and otherwise open Desk externally.
const deskHooks = ['module:workspace-panel-store', 'module:atom-registry', 'module:thread-snapshot-atom', 'dom:panel', 'dom:panelContent'];
const deskAvailable = () => Boolean(host?.panel && host?.context) && !deskHooks.some(name => hostProbes[name]?.ok === false);
globalThis.__t3Custom.diagnostics = () => ({ probes: hostProbes, modules: hostImportedModules, context: current, repositories, mappedRepository: mappedRepository()?.repo ?? null, selectedRepository: selectedRepository()?.repo ?? null, referenceOverride, desiredOpen, deskAvailable: deskAvailable(), frame: { desk: frameDesk, path: framePath, title: deskTitle, ready: deskReady }, hostThread: threadSent && JSON.parse(threadSent), layer: layer && { ...layer, shown: !layerNode.hidden, ready: layerFrame.ready }, views: Object.fromEntries(Object.keys(views).map(name => [name, viewMissing(name)])) });

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
// The panel loads the embedded mode; Open externally keeps the normal address. Opened plainly
// it loads /embed/ and the Desk chooses the page for the thread T3 shows (host:thread). A page
// asked for, a reference or a Refresh, is kept: the Desk is not told the current thread again.
function loadFrame(desk, path, follow) {
  frameDesk = desk; framePath = path; deskReady = false; deskTitle = ''; sentTheme = '';
  keepThread = follow ? null : current?.threadId ?? null;
  iframe.src = `${desk}/embed${path}`;
  scheduleDOM();
}
async function showDesk(reference) {
  if (!settings.enabled) { settings.enabled = true; void saveSettings(); }
  if (!current || !deskAvailable()) { openSettings(); return; }
  if (reference) {
    // The record opens on the Desk that answered for it.
    referenceOverride = { projectKey: projectKey(current), repository: repositoryKey(reference) };
    loadFrame(reference.desk, recordPath(reference), false);
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
  if (!deskReady || !iframe.contentWindow) return;
  const theme = readHostTheme(api.root);
  const text = JSON.stringify(theme);
  if (!always && text === sentTheme) return;
  sentTheme = text;
  iframe.contentWindow.postMessage({ type: 'host:theme', ...theme }, deskOrigin());
}
disposers.push(observeHostTheme(() => sendTheme(false)));
// host:thread tells the Desk which thread T3 shows, by T3's thread ID, whole, with the Desk
// repository its project maps to, the threads it started (T3's lineage) and the item and mission
// IDs its conversation names (the reference scanner). It is sent when the Desk becomes ready, when
// T3 shows another thread, and when the children or mentioned IDs change; never twice in a row
// with the same content.
// Each desk:ready is a fresh Desk that knows no thread, also when T3 re-rendered the panel slot
// and moving the iframe reloaded it. keepThread is the thread a load whose page was asked for
// (a reference or Refresh) shows: that thread is not sent, but a switch before the Desk is ready is.
let threadSent = null;
let keepThread = null;
// The IDs each thread's conversation names, as last scanned for that thread. A thread is sent at
// once with what is known for it (nothing if it was never scanned); its scan sends the rest.
const mentionsByThread = new Map();
function noteMentions(key, ids) { mentionsByThread.set(key, ids); tellDeskThread(); }
function threadMessage() {
  const message = { type: 'host:thread', thread: current.threadId };
  // The repository lets the Desk show this project, not the system inbox, for a thread that holds nothing.
  const repository = selectedRepository()?.repo;
  if (repository) message.repository = repository;
  message.children = host?.children?.(current.threadId) ?? [];
  // With reference links off nothing is scanned, so no thread is said to mention anything.
  message.mentioned = settings.references ? mentionsByThread.get(threadKey(current)) ?? [] : [];
  return message;
}
function tellDeskThread() {
  if (!deskReady || !iframe.contentWindow || !current) return;
  const message = threadMessage();
  const text = JSON.stringify(message);
  if (text === threadSent) return;
  threadSent = text;
  iframe.contentWindow.postMessage(message, deskOrigin());
}
// Children appear when a thread starts one; the snapshot is read again every 3 seconds.
const childrenTimer = setInterval(tellDeskThread, 3000);
disposers.push(() => clearInterval(childrenTimer));
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
  if (message.type === 'desk:ready') {
    deskReady = true; threadSent = keepThread && keepThread === current?.threadId ? JSON.stringify(threadMessage()) : null; keepThread = null;
    sendTheme(true); tellDeskThread();
  }
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
const launch = element('button', { 'data-t3-desk-launch': '', 'aria-label': 'Open Atelier Desk', title: 'Atelier Desk', html: icons.desk });
listen(tabLabel, 'click', () => desiredOpen ? hideDesk() : void showDesk());
listen(closeDesk, 'click', hideDesk);
listen(refreshDesk, 'click', () => { if (frameDesk) loadFrame(frameDesk, framePath, false); });
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
  if (add && tab.parentElement !== add.parentElement) add.before(tab);
  // The controls go after T3's scrolling tab list, so a narrow panel never clips them.
  const tabList = tabbar?.querySelector(hostSelectors.panelTabList);
  if (tabList && controls.previousElementSibling !== tabList) tabList.after(controls);
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
  if (repo && frameDesk !== repo.desk) loadFrame(repo.desk, '/', true);
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
// The installed release and the last release check, from the injector's update.json.
const releaseText = element('p', { id: 'release-status', class: 'muted', text: 'Reading release status…' });
const releaseError = element('p', { id: 'release-error', class: 'error', hidden: '' });
const updateRelease = element('button', { id: 'update-release', hidden: '' });
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
const refreshStatus = element('button', { id: 'refresh-status', text: 'Refresh status' });
const deskAction = element('button', { text: 'Open Desk' });
settingsRoot.append(element('div', { class: 'stack' }, element('h2', { text: 'Customizations' }), statusText, statusError,
  element('div', { class: 'row' }, reloadScripts, refreshStatus, openFolder, deskAction),
  element('h3', { text: 'Release' }), releaseText, releaseError, element('div', { class: 'row' }, updateRelease),
  checkbox('autoUpdate', 'Install new extension releases by themselves (checked at start and daily)'),
  element('h3', { text: 'Desks' }), deskList,
  element('h3', { text: 'Settings' }), element('p', { class: 'muted', text: `Saved in ${api.home}${api.home.includes('\\') ? '\\' : '/'}config.json` }),
  checkbox('enabled', 'Enable the Atelier views (Desk panel, sidebar, layer, project strip)'), checkbox('references', 'Show Atelier reference links and hovercards'), checkbox('followProject', 'Follow the current T3 project'),
  contextText, element('label', {}, document.createTextNode('Project repository'), mapping), element('label', { id: 'pinned-row' }, document.createTextNode('Pinned repository'), pinned),
  element('h3', { text: 'Manual overrides' }), overrideList,
  element('h3', { text: 'T3 hooks' }), probeList));
listen(reloadScripts, 'click', () => api.reloadScripts());
listen(openFolder, 'click', () => void api.request('open-folder').catch(error => displayError(error)));
listen(refreshStatus, 'click', () => { void readStatus(); void loadRepositories(); });
// The injector installs the release and reinjects the extension, which reads the status anew.
listen(updateRelease, 'click', () => {
  updateRelease.disabled = true; releaseText.textContent = `Installing ${updateRelease.dataset.version}…`;
  void api.request('update').then(() => readStatus(), error => { if (!api.signal.aborted) displayError(error); }).finally(() => { updateRelease.disabled = false; });
});
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
    showRelease(status);
  } catch (error) { if (!api.signal.aborted) displayError(error); }
}
function showRelease(status) {
  const update = status.update;
  const checked = update ? ` · checked ${new Date(update.checkedAt).toLocaleString()}` : ' · not checked yet';
  const newest = !update || update.error ? '' : update.newer ? ` · release ${update.latest.version} is available` : update.latest ? ` · newest release ${update.latest.version}, up to date` : ' · no extension release published';
  releaseText.textContent = `Extension ${status.release}${newest}${checked}`;
  const problems = [update?.error && `Release check failed: ${update.error}`, status.restartNeeded && 'An update changed t3-custom.mjs: run t3-custom stop, then open T3 Custom again to load it.'].filter(Boolean);
  releaseError.hidden = !problems.length; releaseError.textContent = problems.join(' ');
  updateRelease.hidden = !update?.newer;
  if (update?.newer) { updateRelease.dataset.version = update.latest.version; updateRelease.textContent = `Update to ${update.latest.version}`; }
}
function syncProbes() {
  const missing = name => hostProbes[name]?.ok === false;
  const deskMissing = deskHooks.filter(missing);
  const rows = Object.entries(hostProbes).map(([name, probe]) => element('li', { class: probe.ok === false ? 'missing' : '', text: `${probe.ok ? '✓' : probe.ok === false ? '✗' : '…'} ${name}: ${probe.detail}` }));
  rows.unshift(element('li', { class: deskMissing.length ? 'missing' : '', text: host ? `Desk panel: ${deskMissing.length ? `off, missing ${deskMissing.join(', ')}` : 'on'}` : 'Desk panel: looking for T3 hooks…' }));
  if (host) rows.splice(1, 0, element('li', { class: host.findThread ? '' : 'missing', text: `Opening threads from the Desk: ${host.findThread ? 'on' : 'off, missing module:environment-atoms'}` }),
    ...Object.entries(views).map(([name, view]) => { const off = viewMissing(name); return element('li', { class: off.length ? 'missing' : '', text: `${view.label}: ${off.length ? `off${view.off ? ` (${view.off})` : ''}, missing ${off.join(', ')}` : 'on'}` }); }));
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
  const label = repo => deskUrls.length > 1 ? `${repo.repo} (${repo.desk})` : repo.repo;
  options(mapping, [['@automatic', `Automatic: ${automatic ? label(automatic) : 'no Desk repository has this identity'}`], ['', 'Not linked'], ...repositories.map(repo => [repositoryKey(repo), label(repo)])]);
  options(pinned, repositories.map(repo => [repositoryKey(repo), label(repo)]));
  contextText.textContent = current ? `Current project: ${current.projectName}${current.repositoryKey ? ` (${current.repositoryKey})` : ' (no repository identity)'}` : 'Open a T3 thread to link its project.';
  mapping.disabled = !current;
  const mapped = current && Object.hasOwn(settings.mappings, projectKey(current)) ? settings.mappings[projectKey(current)] : '@automatic';
  mapping.value = repositoryFor(mapped) ? repositoryKey(repositoryFor(mapped)) : mapped;
  const shown = selectedRepository() ?? repositories[0];
  pinned.value = shown ? repositoryKey(shown) : '';
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
    // The thread is sent at once; its conversation's scan sends its mentions as an update.
    tellDeskThread(); queueScan();
    scheduleDOM();
  } catch (error) { if (!api.signal.aborted && generation === contextGeneration) { hostError = error; displayError(error); api.reportError(error); } }
}
function scheduleDOM() {
  if (domFrame || api.signal.aborted) return;
  // Electron suspends animation frames while its window is hidden.
  domFrame = setTimeout(() => { domFrame = 0; syncPanel(); syncSettings(); syncProjectStrip(); syncProjectHome(); syncSidebar(); }, 0);
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
const referenceUrl = ref => (ref.desk ?? repositoryFor(ref.repo)?.desk ?? deskUrls[0]) + recordPath(ref);
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
    if (external || !deskAvailable() || !repositoryFor(repositoryKey(ref))) window.open(referenceUrl(ref), '_blank', 'noopener');
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
  if (!settings.references || !pattern || !conversation) {
    hideCard(); referenceLayer.replaceChildren();
    tellDeskThread();
    return;
  }
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
  // The IDs are credited only to the thread whose conversation was read: after a switch T3 may
  // still show the previous thread's conversation for a while.
  const owner = conversationOwner(conversation);
  if (current && owner === threadKey(current)) {
    scannedConversation = { thread: owner, root: conversation, firstRow: conversation.querySelector(hostSelectors.timelineRow) };
    noteMentions(owner, [...new Set(references.map(ref => ref.id))].sort());
  }
}
// The thread whose conversation T3 has rendered. T3 marks each assistant message with its
// environment and thread; a conversation with none is still the thread's last scanned while its
// root and first row nodes are the ones that scan read, and the current thread's once either changed.
let scannedConversation = null;
function conversationOwner(conversation) {
  const marked = host?.conversationThread?.(conversation);
  if (marked) return marked;
  const firstRow = conversation.querySelector(hostSelectors.timelineRow);
  if (scannedConversation && scannedConversation.root === conversation && scannedConversation.firstRow === firstRow) return scannedConversation.thread;
  return threadKey(current);
}
// A scan runs 250 ms after the conversation stops changing, and at least once a second while it
// keeps changing, so a streaming thread is scanned too.
let scanDue = 0;
function queueScan() {
  clearTimeout(scanTimer);
  const now = Date.now();
  if (!scanDue) scanDue = now + 1000;
  scanTimer = setTimeout(() => { scanDue = 0; scanReferences(); }, Math.max(0, Math.min(250, scanDue - now)));
}
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

// Views outside the panel: the Inbox row in T3's sidebar, missions in T3's thread list, the layer
// over T3's main area, the project strip under the new-thread composer and the project home from
// the breadcrumb. Each needs only its own hooks; a missing one turns off that view alone.
const views = {
  sidebar: { label: 'Inbox row in the sidebar', hooks: ['dom:sidebarSearch'] },
  missions: { label: 'Missions in T3\'s thread list', hooks: ['layout:threadList', 'dom:settleButton'], off: 'T3\'s list is left as T3 drew it' },
  layer: { label: 'Desk layer over T3\'s main area', hooks: ['dom:mainInset'], off: 'its pages open in the browser' },
  missionThreads: { label: 'Mission threads read from T3', hooks: ['module:atom-registry', 'module:thread-snapshot-atom', 'module:environment-atoms', 'state:thread-origin'] },
  strip: { label: 'Project strip on the new-thread screen', hooks: ['module:composer-draft-store', 'route:draft', 'dom:composerOverlay', 'dom:composerStack'] },
  composer: { label: 'Filling T3\'s composer from the strip', hooks: ['dom:composerEditor'] },
  home: { label: 'Project home from the breadcrumb', hooks: ['dom:breadcrumbProject'] },
};
const viewMissing = name => views[name].hooks.filter(hook => hostProbes[hook]?.ok === false);
const viewOn = name => viewMissing(name).length === 0;
// A DOM hook expected but not yet found is checked again each second until it is found or reported missing.
const expectTimer = setInterval(() => { if (Object.keys(hostDomExpected).length) scheduleDOM(); }, 1000);
// An item ID as Atelier writes them: lowercase letters and digits in dash-separated parts.
const itemIdPattern = /^[a-z0-9]+(?:-[a-z0-9]+)+$/;

// The layer: one Desk frame over T3's main area, leaving the sidebar usable. It shows the inbox,
// a mission's mission control, a project's home or an item. It speaks the host contract: themed on
// desk:ready, desk:open-thread navigates T3 and closes it, desk:key Escape closes it. Its messages
// count only from its own frame's window with its Desk's origin, as the panel's do.
const layerFrame = { iframe: element('iframe', { title: 'Atelier Desk' }), desk: null, path: null, ready: false, theme: '' };
layerFrame.iframe.style.cssText = 'display:block;width:100%;height:100%;border:0;background:var(--background)';
function sendLayerTheme(always) {
  if (!layerFrame.ready || !layerFrame.iframe.contentWindow) return;
  const theme = readHostTheme(api.root); const text = JSON.stringify(theme);
  if (!always && text === layerFrame.theme) return;
  layerFrame.theme = text;
  layerFrame.iframe.contentWindow.postMessage({ type: 'host:theme', ...theme }, new URL(layerFrame.desk).origin);
}
listen(window, 'message', event => {
  if (!layerFrame.desk || event.source !== layerFrame.iframe.contentWindow || event.origin !== new URL(layerFrame.desk).origin) return;
  const message = event.data;
  if (!message || typeof message !== 'object') return;
  if (message.type === 'desk:ready') { layerFrame.ready = true; sendLayerTheme(true); if (layer) layerFrame.iframe.focus(); }
  else if (message.type === 'desk:open-thread' && typeof message.thread === 'string') { openThread(message.thread); closeLayer(); }
  else if (message.type === 'desk:key' && message.key === 'Escape') closeLayer();
});
disposers.push(observeHostTheme(() => sendLayerTheme(false)));
// layer: { key, desk, path }; key is 'inbox', 'mission:…', 'home:…' or 'item:…'.
let layer = null;
function openLayer(key, desk, path) {
  // Without T3's main area the page opens in the browser at its normal address.
  if (!viewOn('layer')) { window.open(desk + path, '_blank', 'noopener'); return; }
  layer = { key, desk, path };
  if (layerFrame.desk !== desk || layerFrame.path !== path) {
    Object.assign(layerFrame, { desk, path, ready: false, theme: '' });
    layerFrame.iframe.src = `${desk}/embed${path}`;
  }
  // The layer follows at once: a covered window delays the DOM pass's timer.
  syncSidebar();
}
function closeLayer() { if (!layer) return; layer = null; syncSidebar(); }
const toggleLayer = (key, desk, path) => layer?.key === key ? closeLayer() : openLayer(key, desk, path);
const layerClose = element('button', { 'data-t3-layer-close': '', 'aria-label': 'Close', title: 'Close (Esc)', html: icons.close });
const layerNode = element('div', { 'data-t3-layer': '', hidden: '' }, layerFrame.iframe, layerClose);
listen(layerClose, 'click', closeLayer);
listen(document, 'keydown', event => { if (event.key === 'Escape' && layer) closeLayer(); });
// Going to a thread in the sidebar, or anywhere else in T3, closes the layer.
listen(document, 'click', event => { if (event.target instanceof Element && event.target.closest(hostSelectors.sidebarRow)) closeLayer(); }, { capture: true });
listen(window, 'hashchange', closeLayer);

const sideIcons = {
  inbox: icon('<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>'),
  chevron: icon('<path d="m6 9 6 6 6-6"/>'),
};
// T3's own variables, so the views follow its theme: --sidebar-* in the sidebar, --card and
// --border for the strip's cards. The rows in T3's thread list copy its folder header's metrics:
// 32px high, 14px text, a 12px chevron, the count and dot in 12px muted text.
const viewStyle = element('style', { text: `
  [data-t3-ext-sidebar] { flex-shrink: 0; display: flex; flex-direction: column; padding: 0 var(--sidebar-content-inset, 8px) 4px; }
  [data-t3-side-row] { display: flex; align-items: center; gap: 10px; height: 32px; padding: 0 var(--sidebar-row-content-inset, 10px); border-radius: var(--control-radius, 6px); cursor: pointer; font-size: 14px; font-weight: 500; color: var(--sidebar-foreground); user-select: none; outline: none; -webkit-app-region: no-drag; min-width: 0; }
  [data-t3-side-row]:hover, [data-t3-list-row] > div:hover { background: var(--sidebar-row-hover); }
  [data-t3-side-row][aria-pressed="true"], [data-t3-list-row][aria-pressed="true"] > div { background: var(--sidebar-row-active); }
  [data-t3-side-row]:focus-visible, [data-t3-list-row] button:focus-visible { box-shadow: inset 0 0 0 2px var(--ring); outline: none; }
  [data-t3-side-icon] { display: inline-flex; flex-shrink: 0; width: 16px; height: 16px; color: var(--sidebar-muted-foreground); } [data-t3-side-icon] svg { width: 16px; height: 16px; }
  [data-t3-side-label] { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  [data-t3-inbox-count] { font-size: 12px; font-variant-numeric: tabular-nums; min-width: 20px; padding: 1px 6px; border-radius: 999px; text-align: center; background: var(--sidebar-row-active); color: var(--sidebar-foreground); }
  [data-t3-inbox-count][data-zero] { background: transparent; color: var(--sidebar-muted-foreground); }
  [data-t3-inbox-count][data-error] { color: var(--destructive); }
  [data-t3-missions-note] { font-size: 12px; color: var(--sidebar-muted-foreground); padding: 4px var(--sidebar-row-content-inset, 10px); overflow-wrap: anywhere; white-space: pre-line; }
  [data-t3-missions-note][data-error] { color: var(--destructive); }
  li[data-t3-list-row] { list-style: none; }
  [data-t3-list-row] > div { display: flex; align-items: center; gap: 10px; height: 32px; padding: 0 var(--sidebar-row-content-inset, 10px) 0 12px; border-radius: var(--control-radius, 6px); min-width: 0; font-size: 14px; color: var(--sidebar-foreground); user-select: none; -webkit-app-region: no-drag; }
  [data-t3-list-row] button { font: inherit; color: inherit; background: none; border: 0; padding: 0; cursor: pointer; text-align: start; height: 100%; }
  [data-t3-mission-title] { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-weight: 500; }
  [data-t3-ring] { display: inline-flex; flex-shrink: 0; color: var(--sidebar-muted-foreground); }
  [data-t3-count] { flex-shrink: 0; font-size: 12px; font-variant-numeric: tabular-nums; color: var(--sidebar-muted-foreground); }
  [data-t3-dot] { flex-shrink: 0; width: 6px; height: 6px; border-radius: 50%; background: var(--warning, #f5a524); }
  [data-t3-fold] { display: inline-flex; align-items: center; flex-shrink: 0; color: var(--sidebar-muted-foreground) !important; }
  [data-t3-fold] svg { width: 12px; height: 12px; transition: transform .1s; } [data-t3-fold][aria-expanded="false"] svg { transform: rotate(-90deg); }
  [data-t3-more-row] > div { height: 28px; padding-left: 36px; font-size: 12px; color: var(--sidebar-muted-foreground); }
  [data-t3-layer] { position: absolute; inset: 0; z-index: 60; background: var(--background); -webkit-app-region: no-drag; }
  [data-t3-layer-close] { position: absolute; top: 8px; right: 12px; z-index: 1; display: inline-flex; align-items: center; justify-content: center; width: 28px; height: 28px; border: 1px solid var(--border); border-radius: 6px; background: var(--card); color: var(--foreground); cursor: pointer; -webkit-app-region: no-drag; }
  [data-t3-layer-close]:hover { background: var(--accent); }
  [data-t3-layer-open] [data-sidebar-row][aria-current] { background: transparent !important; }
  [data-t3-strip] { position: absolute; z-index: 21; box-sizing: border-box; height: 176px; display: flex; flex-direction: column; gap: 8px; padding: 0 2px; font-size: 13px; color: var(--foreground); }
  [data-t3-strip] button { font: inherit; color: inherit; background: none; border: 0; padding: 0; cursor: pointer; text-align: start; }
  [data-t3-strip] button:disabled { cursor: default; opacity: .6; }
  [data-t3-strip-head] { display: flex; align-items: center; justify-content: space-between; height: 24px; flex-shrink: 0; padding: 0 4px; }
  [data-t3-strip-project] { font-size: 12px; color: var(--muted-foreground) !important; text-decoration: underline; text-decoration-color: color-mix(in oklab, currentColor 35%, transparent); text-underline-offset: 3px; }
  [data-t3-strip-project]:hover { color: var(--foreground) !important; }
  [data-t3-strip-plan] { font-size: 12px !important; height: 24px; padding: 0 10px !important; border: 1px solid var(--border) !important; border-radius: 8px; color: var(--foreground); }
  [data-t3-strip-plan]:hover { background: var(--accent) !important; }
  [data-t3-cards] { flex: 1; min-height: 0; display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 8px; }
  [data-t3-strip][data-narrow] [data-t3-cards] { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  [data-t3-strip][data-narrow] [data-t3-card="motion"] { display: none; }
  [data-t3-card] { min-width: 0; min-height: 0; overflow: hidden; display: flex; flex-direction: column; gap: 1px; padding: 8px 6px; background: var(--card); border: 1px solid var(--border); border-radius: 12px; }
  [data-t3-strip] [data-t3-card-head] { display: flex; align-items: center; gap: 6px; padding: 0 6px 4px; font-size: 12px; font-weight: 500; color: var(--muted-foreground) !important; flex-shrink: 0; }
  button[data-t3-card-head]:hover { color: var(--foreground) !important; }
  [data-t3-card-count] { font-variant-numeric: tabular-nums; font-weight: 400; }
  [data-t3-card-row] { display: flex; flex-direction: column; justify-content: center; min-width: 0; padding: 4px 6px !important; border-radius: 6px; line-height: 1.3; }
  [data-t3-card-row]:not(:disabled):hover { background: var(--accent) !important; }
  [data-t3-row-line] { display: flex; align-items: baseline; gap: 6px; min-width: 0; }
  [data-t3-strip] code { font-size: 11px; color: var(--muted-foreground); flex-shrink: 0; }
  [data-t3-row-title] { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  [data-t3-row-meta] { flex-shrink: 0; font-size: 11px; color: var(--muted-foreground); font-variant-numeric: tabular-nums; }
  [data-t3-row-faint] { font-size: 11px; color: var(--muted-foreground); opacity: .8; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  [data-t3-card-empty] { padding: 4px 6px; font-size: 12px; color: var(--muted-foreground); }
  [data-t3-strip-note] { font-size: 12px; color: var(--muted-foreground); padding: 4px; overflow-wrap: anywhere; }
` });
document.head.append(viewStyle);

// The Inbox row under T3's search; under it, a note when T3's list cannot hold missions and the
// errors of reading them.
const inboxCount = element('span', { 'data-t3-inbox-count': '', text: '…' });
const inboxRow = element('div', { 'data-t3-side-row': '', 'data-t3-inbox-row': '', 'data-t3-layer-key': 'inbox', role: 'button', tabindex: '0', 'aria-pressed': 'false', title: 'Atelier inbox' },
  element('span', { 'data-t3-side-icon': '', html: sideIcons.inbox }), element('span', { 'data-t3-side-label': '', text: 'Inbox' }), inboxCount);
const keyActivate = node => listen(node, 'keydown', event => { if (event.target === node && (event.key === 'Enter' || event.key === ' ')) { event.preventDefault(); node.click(); } });
listen(inboxRow, 'click', () => toggleLayer('inbox', deskUrls[0], '/inbox')); keyActivate(inboxRow);
const boringNote = element('div', { 'data-t3-missions-note': '', 'data-t3-boring-note': '', hidden: '', text: 'Missions show in T3\'s thread list with Settings → Labs → Boring sidebar on.' });
const missionsError = element('div', { 'data-t3-missions-note': '', 'data-error': '', hidden: '' });
const extSidebar = element('div', { 'data-t3-ext-sidebar': '' }, inboxRow, boringNote, missionsError);

// Which missions are folded, in localStorage; a mission starts open.
const missionViewKey = 't3-custom:atelier:missions:v1';
let missionView;
try { missionView = JSON.parse(localStorage.getItem(missionViewKey) ?? '{}'); } catch { missionView = {}; }
missionView = { folded: missionView.folded ?? {} };
const saveMissionView = () => localStorage.setItem(missionViewKey, JSON.stringify(missionView));

// GET /api/sidebar from each Desk; each mission keeps the Desk that served it.
let sidebarRepos = null;
async function readSidebar() {
  const repos = []; const errors = [];
  await Promise.all(deskUrls.map(async desk => {
    try { for (const repo of (await api.request('desk-get', { desk, path: '/api/sidebar' })).repos) repos.push({ ...repo, desk }); }
    catch (error) { errors.push(`GET ${desk}/api/sidebar: ${error.message}`); }
  }));
  if (api.signal.aborted) return;
  sidebarRepos = repos;
  missionsError.hidden = !errors.length; missionsError.textContent = errors.join('\n');
  refreshMissions();
}
const threadList = () => viewOn('missionThreads') ? host?.threadList?.() ?? [] : [];
// The Desk's role names, as the list labels a mission thread.
const roleLabels = { lead: 'lead', planner: 'plan', implementer: 'build', reviewer: 'review', decider: 'decide' };
// What T3's list needs, read on each Desk read and every 3 seconds: the open missions in the
// Desks' order; which T3 row (by its exact "<environment>:<thread>" key) belongs to which mission,
// with its role; and which repository each row's project maps to. A mission's "other threads" are
// T3 threads whose creator (lineage.parentThreadId, or delegatedFrom.threadId for a thread
// delegated from a linked environment) is one of its threads; they get no role.
let missions = []; let claims = new Map(); let rowRepos = new Map();
function refreshMissions() {
  const list = threadList();
  const rowKey = thread => `${thread.environmentId}:${thread.id}`;
  missions = (sidebarRepos ?? []).flatMap(repo => repo.missions.map(mission => ({ key: `${repo.desk}/${repo.repo}/${mission.id}`, repo: repo.repo, desk: repo.desk, mission })));
  rowRepos = new Map(list.map(thread => [rowKey(thread), mappedRepository(thread)]).filter(([, repo]) => repo));
  claims = new Map();
  for (const entry of missions) {
    const own = new Map();
    for (const thread of [...entry.mission.threads, ...entry.mission.items.flatMap(item => item.threads)]) if (!own.has(thread.identity)) own.set(thread.identity, thread.role);
    for (const thread of list) {
      if (claims.has(rowKey(thread))) continue;
      if (own.has(thread.id)) claims.set(rowKey(thread), { entry, role: own.get(thread.id) });
      else if (own.has(thread.parent) || own.has(thread.delegatedFrom)) claims.set(rowKey(thread), { entry, role: null });
    }
  }
  syncMissionLayout();
}
const missionsTimer = setInterval(refreshMissions, 3000);

// Missions in T3's own thread list. T3's boring sidebar is one flex column of li: a header per
// project folder, the folder's thread rows after it, then other rows (pinned, shelves). The
// extension never moves or changes T3's nodes: it appends its own rows to that list and gives
// every child a CSS order, so each open folder reads: its header, each mission with threads in it
// (a header, then those threads, indented, with their role), "N more missions" for the
// repository's missions with none there, then the folder's other threads in T3's order. A
// generated stylesheet keyed on data-thread-item indents, labels and folds the mission threads.
const layoutStyle = element('style', { 'data-t3-mission-layout': '' });
document.head.append(layoutStyle);
const listRows = new Map(); const rowSignatures = new WeakMap();
let ordered = new Set(); let layoutList = null; let layoutActive = false;
// T3 fades a removed row by appending a copy of it, the extension's rows included, so every row
// carrying the extension's mark goes, copies too.
const removeListRows = keep => { for (const node of document.querySelectorAll('[data-t3-list-row]')) if (!keep?.has(node)) node.remove(); };
function clearLayout() {
  for (const node of ordered) node.style.order = '';
  removeListRows();
  listRows.clear(); ordered = new Set(); layoutList = null; layoutActive = false; layoutStyle.textContent = '';
}
// One of the extension's rows in T3's list, kept across syncs and rebuilt when its data changes.
function listRow(key, signature, build) {
  let node = listRows.get(key);
  if (!node) { node = element('li', { 'data-t3-list-row': '' }); listRows.set(key, node); }
  if (rowSignatures.get(node) !== signature) { rowSignatures.set(node, signature); build(node); }
  return node;
}
const ring = (done, total) => {
  const length = 2 * Math.PI * 5; const part = total > 0 ? length * Number(done) / Number(total) : 0;
  return element('span', { 'data-t3-ring': '', html: `<svg viewBox="0 0 14 14" width="14" height="14" aria-hidden="true"><circle cx="7" cy="7" r="5" fill="none" stroke="currentColor" stroke-opacity=".25" stroke-width="2"/><circle cx="7" cy="7" r="5" fill="none" stroke="var(--success, #10b981)" stroke-width="2" stroke-dasharray="${part} ${length}" transform="rotate(-90 7 7)"/></svg>` });
};
// A mission's header, like T3's folder header: open, it shows items done of total; folded, the
// number of its threads here and a dot when something in it waits on the human.
function missionHeader(key, entry, count, folded) {
  const { mission } = entry;
  return listRow(key, JSON.stringify([mission, count, folded]), node => {
    const title = element('button', { 'data-t3-mission-title': '', title: `${mission.id}: ${mission.title} (${mission.state}). Open mission control`, text: mission.title });
    listen(title, 'click', () => toggleLayer(`mission:${entry.key}`, entry.desk, `/mission/${encodeURIComponent(entry.repo)}/${encodeURIComponent(mission.id)}`));
    const fold = element('button', { 'data-t3-fold': '', 'aria-expanded': String(!folded), 'aria-label': `${folded ? 'Unfold' : 'Fold'} ${mission.title}`, html: sideIcons.chevron });
    listen(fold, 'click', () => { if (folded) delete missionView.folded[entry.key]; else missionView.folded[entry.key] = true; saveMissionView(); syncMissionLayout(); });
    const marks = folded
      ? [...(mission.waiting > 0 ? [element('span', { 'data-t3-dot': '', title: `${mission.waiting} waiting on you` })] : []), element('span', { 'data-t3-count': '', title: `${count} of its threads here`, text: String(count) })]
      : [element('span', { 'data-t3-count': '', title: `${mission.done} of ${mission.total} items done`, text: `${mission.done}/${mission.total}` })];
    node.setAttribute('data-t3-mission-row', mission.id); node.setAttribute('data-t3-layer-key', `mission:${entry.key}`);
    node.replaceChildren(element('div', {}, ring(mission.done, mission.total), title, ...marks, fold));
  });
}
// The repository's missions with no thread in this folder, in one row that opens the project home.
function moreRow(key, repo, rest) {
  const waiting = rest.filter(entry => entry.mission.waiting > 0).length;
  return listRow(key, JSON.stringify([repositoryKey(repo), rest.map(entry => [entry.mission.title, entry.mission.waiting])]), node => {
    const open = element('button', { 'data-t3-side-label': '', title: `${rest.map(entry => `${entry.mission.id}: ${entry.mission.title}`).join('\n')}\nOpen the ${repo.repo} project home`, text: `${rest.length} more mission${rest.length === 1 ? '' : 's'}` });
    listen(open, 'click', () => toggleLayer(`home:${repositoryKey(repo)}`, repo.desk, `/home/${encodeURIComponent(repo.repo)}`));
    node.setAttribute('data-t3-more-row', repo.repo); node.setAttribute('data-t3-layer-key', `home:${repositoryKey(repo)}`);
    node.replaceChildren(element('div', {}, open, ...(waiting ? [element('span', { 'data-t3-dot': '', title: `${waiting} of them wait on you` })] : [])));
  });
}
function syncMissionLayout() {
  const on = settings.enabled && viewOn('missions');
  const list = on ? document.querySelector(hostSelectors.projectFolder)?.parentElement : null;
  // T3 lists threads but no folders, and is not searching: the boring sidebar is off.
  boringNote.hidden = !on || Boolean(list) || Boolean(document.querySelector(hostSelectors.sidebarSearch)?.value) || !document.querySelector(hostSelectors.threadItem);
  const problem = list && hostListProblem(list);
  if (list) setHostProbe('layout:threadList', !problem, problem ? `T3's thread list: ${problem}` : 'project folders and their thread rows in one flex column');
  if (!list || problem) { if (layoutList || ordered.size || document.querySelector('[data-t3-list-row]')) clearLayout(); return; }
  // T3 fades a row it removes by appending an inert, absolutely placed copy (Sidebar.motion.ts);
  // being out of flow, those copies, like the extension's own rows, are left out of the walk.
  const children = [...list.children].filter(node => !node.hasAttribute('data-t3-list-row') && !(node.inert && node.style.position === 'absolute'));
  const sequence = []; const rules = []; const used = new Set(); let folderThreads = 0;
  const add = (key, node) => { used.add(key); sequence.push(node); };
  for (let index = 0; index < children.length;) {
    const node = children[index++];
    sequence.push(node);
    const button = node.matches(hostSelectors.projectFolder) && node.querySelector(hostSelectors.folderButton);
    if (!button) continue;
    // A folder's threads are the thread rows after its header; a closed folder has none.
    const threads = [];
    while (index < children.length && children[index].matches(hostSelectors.threadItem)) threads.push(children[index++]);
    folderThreads += threads.length;
    const groups = new Map(); const loose = [];
    for (const item of threads) {
      const claim = claims.get(item.dataset.threadItem);
      if (!claim) { loose.push(item); continue; }
      if (!groups.has(claim.entry)) groups.set(claim.entry, []);
      groups.get(claim.entry).push([item, claim]);
    }
    const folderKey = button.dataset.sidebarRow;
    for (const entry of missions.filter(entry => groups.has(entry))) {
      const items = groups.get(entry); const folded = Boolean(missionView.folded[entry.key]);
      add(`${folderKey}|${entry.key}`, missionHeader(`${folderKey}|${entry.key}`, entry, items.length, folded));
      for (const [item, claim] of items) {
        const selector = `li[data-thread-item="${CSS.escape(item.dataset.threadItem)}"]`;
        rules.push(folded ? `${selector} { display: none !important; }` : `${selector} { margin-left: 18px; box-shadow: inset 1px 0 var(--border); }`);
        if (claim.role && !folded) rules.push(`${selector} [data-sidebar-row]::after { content: "${CSS.escape(roleLabels[claim.role] ?? claim.role)}"; flex-shrink: 0; font-size: 11px; color: var(--sidebar-muted-foreground, var(--muted-foreground)); }`);
        sequence.push(item);
      }
    }
    const repo = threads.map(item => rowRepos.get(item.dataset.threadItem)).find(Boolean);
    const rest = repo ? missions.filter(entry => entry.desk === repo.desk && entry.repo === repo.repo && !groups.has(entry)) : [];
    if (rest.length) add(`${folderKey}|more`, moreRow(`${folderKey}|more`, repo, rest));
    sequence.push(...loose);
  }
  for (const key of listRows.keys()) if (!used.has(key)) listRows.delete(key);
  removeListRows(new Set(listRows.values()));
  for (const node of listRows.values()) if (node.parentElement !== list) list.append(node);
  const next = new Set(sequence);
  for (const node of ordered) if (!next.has(node)) node.style.order = '';
  sequence.forEach((node, position) => { if (node.style.order !== String(position)) node.style.order = String(position); });
  ordered = next; layoutList = list;
  layoutActive = [...used].some(key => !key.endsWith('|more'));
  const css = rules.join('\n');
  if (layoutStyle.textContent !== css) layoutStyle.textContent = css;
  for (const node of listRows.values()) node.setAttribute('aria-pressed', String(Boolean(layer) && node.dataset.t3LayerKey === layer.key));
  expectHostDom('settleButton', folderThreads > 0, 'T3 listed threads in a project folder');
}
// Two of T3's gestures act on a run of rows taken from T3's own order, which differs from what is
// drawn once a mission is placed: dragging from a row's Settle button settles every row between,
// and Shift-click selects every row between the anchor and the row clicked, which bulk Settle,
// Snooze, Delete and a drag to the composer then act on. While a mission is placed, both are
// stopped before T3 sees them: a press on Settle, so no drag starts (a plain click still settles
// that thread), and a Shift-click on a thread row (a click and Ctrl- or Cmd-click still work).
listen(document, 'pointerdown', event => {
  if (layoutActive && event.target instanceof Element && event.target.closest(hostSelectors.settleButton)) event.stopPropagation();
}, { capture: true });
listen(document, 'click', event => {
  if (!layoutActive || !event.shiftKey || event.ctrlKey || event.metaKey || !(event.target instanceof Element)) return;
  if (event.target.closest(hostSelectors.threadItem) && !event.target.closest('button, a')) { event.preventDefault(); event.stopPropagation(); }
}, { capture: true });

function syncSidebar() {
  const onMain = Boolean(host) && !location.hash.includes('/settings');
  expectHostDom('sidebarSearch', onMain, 'T3 showed a thread or new-thread screen');
  expectHostDom('mainInset', onMain, 'T3 showed a thread or new-thread screen');
  const t3Threads = threadList().length > 0;
  expectHostDom('threadItem', onMain && t3Threads, 'T3 listed its threads');
  expectHostDom('sidebarRow', onMain && t3Threads, 'T3 listed its threads');
  const search = viewOn('sidebar') && document.querySelector(hostSelectors.sidebarSearch);
  const group = search?.closest('[data-slot="sidebar-group"]')?.parentElement;
  if (group && settings.enabled) { if (extSidebar.previousElementSibling !== group) group.after(extSidebar); }
  else extSidebar.remove();
  syncMissionLayout();
  const main = document.querySelector(hostSelectors.mainInset);
  const shown = Boolean(layer) && Boolean(main) && settings.enabled;
  for (const row of document.querySelectorAll('[data-t3-layer-key]')) row.setAttribute('aria-pressed', String(shown && row.dataset.t3LayerKey === layer.key));
  document.documentElement.toggleAttribute('data-t3-layer-open', shown);
  if (shown && layerNode.parentElement !== main) main.append(layerNode);
  // The Desk's pages take keys (mission control is keyboard-driven): its frame gets focus as the
  // layer appears, and again on desk:ready for a page still loading.
  const appearing = shown && layerNode.hidden;
  layerNode.hidden = !shown;
  if (appearing) layerFrame.iframe.focus();
  if (!shown && document.activeElement === layerFrame.iframe) layerFrame.iframe.blur();
}

// The count: needs-human items, proposed plans and waiting threads, across repositories and Desks.
// A Desk that cannot be read, or a repository it reports with an error (its lists are then empty),
// shows "!" beside the count of what could be read, with each error in the row's tooltip.
async function readInbox() {
  const errors = []; let count = 0; let read = 0;
  await Promise.all(deskUrls.map(async desk => {
    try {
      for (const repo of (await api.request('desk-get', { desk, path: '/api/inbox' })).repos) {
        if (repo.error) errors.push(`${desk} could not read ${repo.repo}: ${repo.error}`);
        count += (repo.needs_human ?? []).reduce((sum, group) => sum + group.items.length, 0) + (repo.proposed ?? []).length + (repo.threads ?? []).length;
      }
      read++;
    } catch (error) { errors.push(`GET ${desk}/api/inbox: ${error.message}`); }
  }));
  if (api.signal.aborted) return;
  inboxCount.textContent = !errors.length ? String(count) : read ? `${count} !` : '!';
  inboxCount.toggleAttribute('data-error', errors.length > 0);
  inboxCount.toggleAttribute('data-zero', !errors.length && count === 0);
  inboxRow.title = errors.length ? `Atelier inbox: ${read ? `${count} waiting in what could be read; ` : ''}${errors.join('; ')}` : `Atelier inbox: ${count} waiting (read ${new Date().toLocaleTimeString()})`;
}
const readDesks = () => { void readInbox(); void readSidebar(); };
readDesks();
const deskTimer = setInterval(readDesks, 60000);

// Project strip: under the new-thread composer of a project with a Desk repository, three cards
// from GET /api/launch?repo=<repo>: Waiting on you, In motion, Next up. Fields the Desk does not
// send count as empty. A Next up row puts a prompt in the composer; it is never sent.
const strip = element('div', { 'data-t3-strip': '', hidden: '' });
// The composer's width changes without a window resize (panels, sidebar); follow the canvas.
const stripResize = new ResizeObserver(() => scheduleDOM());
let launchKey = null; let launchAt = 0;
// Only into the composer of the draft the strip was drawn for: a click never fills another thread's.
async function setComposerText(text) {
  const draft = host?.draftContext?.();
  const reason = draft && host.fillComposer ? await host.fillComposer(draft.draftId, text) : 'T3 does not show a new-thread screen';
  if (reason) api.reportError(new Error(`Project strip: the composer was not filled: ${reason}`));
}
const age = seconds => seconds < 3600 ? `${Math.max(1, Math.round(seconds / 60))}m` : seconds < 86400 ? `${Math.round(seconds / 3600)}h` : `${Math.round(seconds / 86400)}d`;
function stripCard(name, title, count, rows, empty, onHeader) {
  const head = element(onHeader ? 'button' : 'div', { 'data-t3-card-head': '' }, element('span', { text: title }), element('span', { 'data-t3-card-count': '', text: String(count) }));
  if (onHeader) head.addEventListener('click', onHeader);
  return element('div', { 'data-t3-card': name }, head, ...(rows.length ? rows : [element('div', { 'data-t3-card-empty': '', text: empty })]));
}
function stripRow(onClick, title, ...children) {
  const node = element('button', { 'data-t3-card-row': '', title }, ...children);
  if (onClick) node.addEventListener('click', onClick); else node.disabled = true;
  return node;
}
async function readLaunch(repo) {
  const key = repositoryKey(repo);
  let data;
  try { data = await api.request('desk-get', { desk: repo.desk, path: `/api/launch?repo=${encodeURIComponent(repo.repo)}` }); }
  catch (error) { if (launchKey === key) strip.replaceChildren(element('div', { 'data-t3-strip-note': '', 'data-error': '', text: `GET ${repo.desk}/api/launch?repo=${repo.repo}: ${error.message}` })); return; }
  if (api.signal.aborted || launchKey !== key) return;
  const ready = data.ready ?? []; const waitingItems = data.waiting_items ?? []; const inMotion = data.in_motion ?? [];
  const waiting = data.waiting ?? waitingItems.length;
  const t3 = new Map(threadList().map(thread => [thread.id, thread]));
  const openInbox = () => openLayer('inbox', deskUrls[0], '/inbox');
  const openItem = id => openLayer(`item:${repo.desk}/${repo.repo}/${id}`, repo.desk, `/r/${encodeURIComponent(repo.repo)}/item/${encodeURIComponent(id)}`);
  const waitingRows = waitingItems.slice(0, 2).map(item => stripRow(openInbox, `${item.id}: ${item.title}`, element('span', { 'data-t3-row-title': '', text: item.title })));
  if (waiting > waitingRows.length) waitingRows.push(stripRow(openInbox, 'Open the inbox', element('span', { 'data-t3-row-faint': '', text: waitingRows.length ? `${waiting - waitingRows.length} more` : `${waiting} waiting · open the inbox` })));
  const motionRows = inMotion.slice(0, 3).map(item => {
    const thread = item.holder && t3.get(item.holder.identity);
    return stripRow(() => thread ? host.showThread(thread.environmentId, thread.id) : openItem(item.id), `${item.id}: ${item.title}${item.holder ? `, held by ${item.holder.title}` : ''}${thread ? '' : ' (opens in the Desk)'}`,
      element('span', { 'data-t3-row-line': '' }, element('code', { text: item.id }), element('span', { 'data-t3-row-title': '', text: item.title }), element('span', { 'data-t3-row-meta': '', text: [item.stage, item.age_seconds != null && age(item.age_seconds)].filter(Boolean).join(' · ') })),
      ...(item.holder ? [element('span', { 'data-t3-row-faint': '', text: item.holder.title })] : []));
  });
  // The prompt is built only from an ID of Atelier's form; any other ID is shown but cannot be used.
  const nextRows = ready.slice(0, 3).map(item => {
    const usable = itemIdPattern.test(item.id) && viewOn('composer');
    return stripRow(usable && (() => setComposerText(`Start ${item.id}: run \`atelier start ${item.id}\` in a fresh checkout and follow what it prints.`)),
      usable ? `${item.id}: ${item.title}${item.priority ? ` (${item.priority})` : ''}. Puts a start prompt in the composer` : `${item.id}: ${itemIdPattern.test(item.id) ? `T3's composer is not available (${viewMissing('composer').join(', ')})` : 'not an Atelier item ID, so no prompt is made from it'}`,
      element('span', { 'data-t3-row-line': '' }, element('code', { text: item.id }), element('span', { 'data-t3-row-title': '', text: item.title })));
  });
  const project = element('button', { 'data-t3-strip-project': '', title: `Open the ${repo.repo} project home`, text: repo.repo });
  project.addEventListener('click', () => openLayer(`home:${repositoryKey(repo)}`, repo.desk, `/home/${encodeURIComponent(repo.repo)}`));
  const plan = element('button', { 'data-t3-strip-plan': '', title: 'Put a planning prompt in the composer', text: 'Plan a mission' });
  plan.disabled = !viewOn('composer');
  plan.addEventListener('click', () => setComposerText('Let\'s plan a mission for '));
  strip.replaceChildren(element('div', { 'data-t3-strip-head': '' }, project, plan),
    element('div', { 'data-t3-cards': '' },
      stripCard('waiting', 'Waiting on you', waiting, waitingRows, 'Nothing waiting on you', openInbox),
      stripCard('motion', 'In motion', inMotion.length, motionRows, 'Nothing in motion'),
      stripCard('next', 'Next up', ready.length, nextRows, 'Nothing ready')));
}
function syncProjectStrip() {
  const draft = settings.enabled ? host?.draftContext?.() : null;
  const repo = draft && mappedRepository(draft);
  expectHostDom('composerOverlay', Boolean(repo), 'T3 showed a new-thread screen');
  expectHostDom('composerEditor', Boolean(draft), 'T3 showed a new-thread screen');
  const overlay = repo && viewOn('strip') && document.querySelector(hostSelectors.composerOverlay);
  expectHostDom('composerStack', Boolean(overlay), 'T3 showed its composer overlay');
  const stack = overlay?.querySelector(hostSelectors.composerStack);
  if (!stack) { strip.hidden = true; launchKey = null; return; }
  const canvas = overlay.parentElement;
  if (strip.parentElement !== canvas) { canvas.append(strip); stripResize.disconnect(); stripResize.observe(canvas); }
  if (launchKey !== repositoryKey(repo) || Date.now() - launchAt > 60000) {
    if (launchKey !== repositoryKey(repo)) strip.replaceChildren(element('div', { 'data-t3-strip-note': '', text: `Reading ${repo.repo} from the Desk…` }));
    launchKey = repositoryKey(repo); launchAt = Date.now(); void readLaunch(repo);
  }
  const area = canvas.getBoundingClientRect(); const composer = stack.getBoundingClientRect();
  // Below 700px the In motion card goes; the composer-filling Next up card stays.
  strip.toggleAttribute('data-narrow', composer.width < 700);
  strip.style.cssText = `top:${Math.round(composer.bottom - area.top + 12)}px;left:${Math.round(composer.left - area.left)}px;width:${Math.round(composer.width)}px`;
  strip.hidden = false;
}

// Project home: the project's name in T3's thread breadcrumb opens /embed/home/<repo> in the
// layer instead of starting a new thread, for a project with a Desk repository.
function syncProjectHome() {
  expectHostDom('breadcrumbProject', Boolean(current) && !location.hash.includes('/settings'), 'T3 showed a thread');
}
listen(document, 'click', event => {
  const target = event.target instanceof Element ? event.target.closest(hostSelectors.breadcrumbProject) : null;
  if (!target || !settings.enabled) return;
  const repo = mappedRepository(current ?? host?.draftContext?.() ?? null);
  if (!repo) return;
  event.preventDefault(); event.stopPropagation();
  toggleLayer(`home:${repositoryKey(repo)}`, repo.desk, `/home/${encodeURIComponent(repo.repo)}`);
}, { capture: true });

// A covered window throttles timers: when it is shown again, every view reads afresh.
listen(document, 'visibilitychange', () => {
  if (document.hidden) return;
  readDesks(); launchAt = 0; refreshMissions(); tellDeskThread(); scheduleDOM();
});
disposers.push(() => {
  clearInterval(deskTimer); clearInterval(missionsTimer); clearInterval(expectTimer); stripResize.disconnect();
  clearLayout(); extSidebar.remove(); layerNode.remove(); strip.remove(); viewStyle.remove(); layoutStyle.remove();
  document.documentElement.removeAttribute('data-t3-layer-open');
});
listen(window, 'resize', scheduleDOM);

const observer = new MutationObserver(records => {
  // Missions are placed before the browser paints what T3 changed in its list, and on the new
  // list when T3 replaces it (a search, a route change).
  const list = document.querySelector(hostSelectors.projectFolder)?.parentElement ?? null;
  const sidebar = document.querySelector('[data-workspace-area="sidebar"]');
  if (list !== layoutList || records.some(record => sidebar?.contains(record.target))) syncMissionLayout();
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
