// T3 extension UI. Desk remains the backend; no T3 credentials enter its iframe.
const repositories = api.config.repositories ?? [];
const deskBase = (api.config.baseUrl ?? 'http://127.0.0.1:3780').replace(/\/$/, '');
const preferenceKey = 't3-custom:atelier:v1';
let preferences;
try { preferences = JSON.parse(localStorage.getItem(preferenceKey) ?? '{}'); } catch { preferences = {}; }
preferences = { enabled: true, references: true, followProject: true, mappings: {}, openThreads: {}, ...preferences };
const save = () => localStorage.setItem(preferenceKey, JSON.stringify(preferences));
const disposers = [];
const listen = (node, name, callback, options = {}) => node.addEventListener(name, callback, { ...options, signal: api.signal });
function element(tag, attributes = {}, ...children) {
  const node = document.createElement(tag);
  for (const [name, value] of Object.entries(attributes)) {
    if (name === 'text') node.textContent = value;
    else if (name === 'class') node.className = value;
    else node.setAttribute(name, String(value));
  }
  node.append(...children); return node;
}
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
  .probes { margin: 0; padding-left: 18px; font-size: 12px; line-height: 1.6; } .probes .missing { color: var(--destructive, #ff9292); }
`;
const addStyle = root => root.append(element('style', { text: css }));
addStyle(api.root);
const globalStyle = element('style', { text: `
  [data-t3-desk-tab], [data-t3-desk-launch] { font: inherit; font-size: 12px; cursor: pointer; padding: 4px 9px; margin: 0 3px; border-radius: 6px; color: var(--foreground); background: var(--secondary); white-space: nowrap; -webkit-app-region: no-drag; }
  [data-t3-desk-tab][aria-pressed="true"] { background: var(--accent); box-shadow: inset 0 0 0 1px var(--border); }
  [data-t3-desk-tab]:focus-visible, [data-t3-desk-launch]:focus-visible { outline: 2px solid var(--ring); }
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
  try { current = JSON.parse(sessionStorage.getItem(preferenceKey + ':context') ?? 'null'); } catch {}
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
function mappedRepository() {
  const key = projectKey(current);
  if (Object.hasOwn(preferences.mappings, key)) return repositories.find(repo => repo.repo === preferences.mappings[key]) ?? null;
  return repositories.find(repo => repo.repositoryKeys?.includes(current?.repositoryKey)) ?? null;
}
function selectedRepository() {
  if (referenceOverride?.projectKey === projectKey(current)) return repositories.find(repo => repo.repo === referenceOverride.repo) ?? null;
  return preferences.followProject ? mappedRepository() : repositories.find(repo => repo.repo === preferences.pinnedRepo) ?? mappedRepository() ?? repositories[0] ?? null;
}
// The Desk panel needs T3's panel store, project context and a slot to mount in;
// references need only the conversation DOM and otherwise open Desk externally.
const deskHooks = ['module:workspace-panel-store', 'module:atom-registry', 'module:prepared-connection-atom', 'dom:panel', 'dom:panelContent'];
const deskAvailable = () => Boolean(host?.panel && host?.context) && !deskHooks.some(name => hostProbes[name]?.ok === false);
globalThis.__t3Custom.diagnostics = () => ({ probes: hostProbes, modules: hostImportedModules, context: current, mappedRepository: mappedRepository()?.repo ?? null, selectedRepository: selectedRepository()?.repo ?? null, referenceOverride, desiredOpen, deskAvailable: deskAvailable() });

// The independent recovery/settings button is created before any host adapter is used.
const settingsButton = element('button', { id: 'extension-settings', text: '⚙', 'aria-label': 'T3 Custom settings', title: 'Customizations' });
settingsButton.style.cssText = 'position:fixed;right:12px;bottom:12px;z-index:40;width:34px;height:34px;padding:0;font-size:18px';
api.root.append(settingsButton);
function openSettings() { focusSettings = true; location.hash = '/settings/general'; scheduleDOM(); }
listen(settingsButton, 'click', openSettings);

const mount = element('div', { 'data-t3-desk-mount': '', hidden: '' });
mount.style.cssText = 'position:absolute;inset:0;z-index:1;min-width:0;min-height:0';
const panelRoot = mount.attachShadow({ mode: 'open' }); addStyle(panelRoot);
panelRoot.append(element('style', { text: ':host{display:block;height:100%}.panel{height:100%;min-height:0;display:flex;flex-direction:column;background:var(--background,#202124)}header{padding:7px 10px;border-bottom:1px solid var(--border,#60636a)}iframe{flex:1;min-height:0;width:100%;border:0;background:var(--background,#202124)}.empty{padding:20px;max-width:55ch}' }));
const scopeLabel = element('span', { class: 'muted grow' });
const refreshDesk = element('button', { text: 'Refresh', title: 'Refresh Desk' });
const externalDesk = element('a', { text: 'Open ↗', target: '_blank', rel: 'noopener noreferrer' });
const closeDesk = element('button', { text: 'Close', 'aria-label': 'Close Desk surface' });
const panelError = element('p', { class: 'error', hidden: '' });
const iframe = element('iframe', { title: 'Atelier Desk' });
const linkProject = element('button', { text: 'Link project in Customizations' });
const empty = element('div', { class: 'empty stack' }, element('h3', { text: 'No Atelier repository linked' }), element('p', { class: 'muted', text: 'Choose the repository for this T3 project in Customizations.' }), linkProject);
const panel = element('div', { class: 'panel' }, element('header', { class: 'row' }, element('strong', { text: 'Desk' }), scopeLabel, externalDesk, refreshDesk, closeDesk), panelError, empty, iframe);
panelRoot.append(panel);
const nativeInert = new Map();
const positions = new Map();
const areaLabels = new Map();
const markedTabbars = new Set();
let surfaceContent;
let iframeRepository = null;
let iframePath = null;
function restoreNative() {
  for (const [node, inert] of nativeInert) node.inert = inert;
  nativeInert.clear(); surfaceContent?.removeAttribute('data-t3-desk-selected');
  for (const tabbar of markedTabbars) tabbar.removeAttribute('data-t3-desk-tabbar');
  markedTabbars.clear();
  for (const [area, label] of areaLabels) label === null ? area.removeAttribute('aria-label') : area.setAttribute('aria-label', label);
  areaLabels.clear();
}
function navigateDesk(repo, path = `/r/${encodeURIComponent(repo.repo)}`) {
  const url = deskBase + path;
  if (iframePath !== url) { iframe.src = url; iframePath = url; }
  iframeRepository = repo.repo; externalDesk.href = url;
}
async function showDesk(reference) {
  if (!preferences.enabled) { preferences.enabled = true; save(); }
  if (!current || !deskAvailable()) { openSettings(); return; }
  if (reference) {
    referenceOverride = { projectKey: projectKey(current), repo: reference.repo };
    const repo = repositories.find(repo => repo.repo === reference.repo);
    navigateDesk(repo, `/r/${encodeURIComponent(repo.repo)}/${reference.kind === 'mission' ? 'mission' : 'item'}/${encodeURIComponent(reference.id)}`);
  }
  desiredOpen = true; preferences.openThreads[threadKey(current)] = true; save();
  if (location.hash.includes('/settings')) location.hash = `/${current.environmentId}/${encodeURIComponent(current.threadId)}`;
  scheduleDOM();
  // A stop or reinjection during the wait must not open anything afterwards.
  if (await host.panel.show(current) || api.signal.aborted) return;
  // T3's panel did not render a slot (the probe now names the missing hook): use the browser.
  hideDesk();
  const repo = selectedRepository();
  window.open(reference ? referenceUrl(reference) : repo ? `${deskBase}/r/${encodeURIComponent(repo.repo)}` : deskBase, '_blank', 'noopener');
}
function hideDesk() {
  desiredOpen = false;
  if (current) { preferences.openThreads[threadKey(current)] = false; save(); }
  scheduleDOM();
}
listen(refreshDesk, 'click', () => { if (iframePath) iframe.src = iframePath; });
listen(closeDesk, 'click', hideDesk); listen(linkProject, 'click', openSettings);
listen(iframe, 'error', () => { panelError.hidden = false; panelError.textContent = `Could not load ${iframePath}. Check connectivity to ${deskBase} and use Refresh.`; });

const tab = element('button', { 'data-t3-desk-tab': '', text: 'Desk', 'aria-pressed': 'false', title: 'Atelier Desk' });
const launch = element('button', { 'data-t3-desk-launch': '', text: 'Desk', 'aria-label': 'Open Atelier Desk' });
listen(tab, 'click', () => desiredOpen ? hideDesk() : void showDesk());
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
  const toolbar = [...document.querySelectorAll(hostSelectors.layoutControls)].find(node => node.getBoundingClientRect().width > 0);
  if (toolbar && launch.parentElement !== toolbar) toolbar.prepend(launch);
  const allowed = preferences.enabled && deskAvailable() && Boolean(current) && !location.hash.includes('/settings');
  tab.hidden = launch.hidden = !allowed;
  const state = allowed && host.panel.state(current);
  const visible = allowed && desiredOpen && state?.isOpen && Boolean(content) && area.getBoundingClientRect().width > 0;
  tab.setAttribute('aria-pressed', String(Boolean(visible)));
  if (!visible) { mount.hidden = true; restoreNative(); return; }
  if (surfaceContent !== content) { restoreNative(); surfaceContent = content; }
  if (!positions.has(content)) { positions.set(content, content.style.position); if (getComputedStyle(content).position === 'static') content.style.position = 'relative'; }
  if (mount.parentElement !== content) content.append(mount);
  content.setAttribute('data-t3-desk-selected', '');
  tabbar?.setAttribute('data-t3-desk-tabbar', ''); if (tabbar) markedTabbars.add(tabbar);
  for (const child of content.children) if (child !== mount && !nativeInert.has(child)) { nativeInert.set(child, child.inert); child.inert = true; }
  mount.hidden = false;
  if (!areaLabels.has(area)) areaLabels.set(area, area.getAttribute('aria-label'));
  area.setAttribute('aria-label', 'Workspace panel: Atelier Desk');
  const repo = selectedRepository();
  scopeLabel.textContent = `${current.projectName} · ${preferences.followProject ? 'Following project' : 'Pinned'}`;
  empty.hidden = Boolean(repo); iframe.hidden = externalDesk.hidden = refreshDesk.hidden = !repo;
  if (repo && iframeRepository !== repo.repo) navigateDesk(repo);
}

const settingsHost = element('section', { id: 't3-custom-settings', 'data-t3-custom-settings': '' });
settingsHost.style.cssText = 'display:block;margin:24px 0;padding:20px 0;border-top:1px solid var(--border)';
const settingsRoot = settingsHost.attachShadow({ mode: 'open' }); addStyle(settingsRoot);
const statusText = element('p', { class: 'muted', text: 'Reading injector status…' });
const statusError = element('p', { class: 'error', hidden: '' });
const contextText = element('p', { class: 'muted' });
const probeList = element('ul', { class: 'probes', 'aria-label': 'T3 hooks used by this extension' });
const mapping = element('select', { 'aria-label': 'Atelier repository for current T3 project' }, element('option', { value: '', text: 'Not linked' }), ...repositories.map(repo => element('option', { value: repo.repo, text: repo.repo })));
const pinned = element('select', { 'aria-label': 'Pinned Atelier repository' }, ...repositories.map(repo => element('option', { value: repo.repo, text: repo.repo })));
function checkbox(key, title) {
  const input = element('input', { type: 'checkbox' }); input.checked = preferences[key];
  listen(input, 'change', () => { preferences[key] = input.checked; save(); referenceOverride = null; syncSettings(); scheduleDOM(); queueScan(); });
  return element('label', {}, input, document.createTextNode(title));
}
const reloadScripts = element('button', { id: 'reload-scripts', text: 'Reload scripts' });
const openFolder = element('button', { text: 'Open customization folder' });
const refreshStatus = element('button', { text: 'Refresh status' });
const deskAction = element('button', { text: 'Open Desk' });
settingsRoot.append(element('div', { class: 'stack' }, element('h2', { text: 'Customizations' }), statusText, statusError,
  element('div', { class: 'row' }, reloadScripts, refreshStatus, openFolder, deskAction),
  element('h3', { text: 'T3 hooks' }), probeList,
  checkbox('enabled', 'Enable the Atelier Desk surface'), checkbox('references', 'Show Atelier reference links and hovercards'), checkbox('followProject', 'Follow the current T3 project'),
  contextText, element('label', {}, document.createTextNode('Project repository'), mapping), element('label', { id: 'pinned-row' }, document.createTextNode('Pinned repository'), pinned),
  element('p', { class: 'muted', text: `Desk backend: ${deskBase}` }), element('code', { text: api.home })));
listen(reloadScripts, 'click', () => api.reloadScripts());
listen(openFolder, 'click', () => void api.request('open-folder').catch(error => displayError(error)));
listen(refreshStatus, 'click', () => void readStatus());
listen(deskAction, 'click', () => void showDesk());
listen(mapping, 'change', () => { if (!current) return; preferences.mappings[projectKey(current)] = mapping.value; save(); referenceOverride = null; scheduleDOM(); });
listen(pinned, 'change', () => { preferences.pinnedRepo = pinned.value; save(); referenceOverride = null; scheduleDOM(); });
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
  probeList.replaceChildren(...rows);
}
function syncSettings() {
  syncProbes();
  contextText.textContent = current ? `Current project: ${current.projectName}${current.repositoryKey ? ` (${current.repositoryKey})` : ''}` : 'Open a T3 thread to link its project.';
  mapping.disabled = !current; mapping.value = mappedRepository()?.repo ?? '';
  pinned.value = selectedRepository()?.repo ?? repositories[0]?.repo ?? '';
  settingsRoot.querySelector('#pinned-row').hidden = preferences.followProject;
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
    if (current) sessionStorage.setItem(preferenceKey + ':context', JSON.stringify(current));
    const key = projectKey(current);
    if (key !== lastProject) { referenceOverride = null; lastProject = key; }
    desiredOpen = Boolean(preferences.openThreads[threadKey(current)]);
    nativeActiveId = current && (host.panel?.state(current)?.activeSurfaceId ?? null);
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
const slugs = repositories.map(repo => repo.slug).filter(Boolean).sort((a, b) => b.length - a.length);
const escapeRegex = value => value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
const pattern = slugs.length ? new RegExp(`(?<![\\w-])(?:${slugs.map(escapeRegex).join('|')})-[a-z0-9]+(?![\\w-])`, 'g') : null;
const referenceUrl = ref => `${deskBase}/r/${encodeURIComponent(ref.repo)}/${ref.kind === 'mission' ? 'mission' : 'item'}/${encodeURIComponent(ref.id)}`;
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
async function readReference(ref) {
  const key = `${ref.repo}/${ref.id}`;
  let cached = referenceCache.get(key);
  if (!cached || Date.now() - cached.at > 30000) {
    const read = async () => {
      try { return { kind: 'item', data: await api.request('desk-get', { path: `/api/repos/${encodeURIComponent(ref.repo)}/items/${ref.id}` }) }; }
      catch (error) { if (error.status !== 404) throw error; }
      return { kind: 'mission', data: await api.request('desk-get', { path: `/api/repos/${encodeURIComponent(ref.repo)}/missions/${ref.id}` }) };
    };
    cached = { at: Date.now(), promise: read() }; referenceCache.set(key, cached);
  }
  return cached.promise;
}
async function activateReference(ref, external = false) {
  try {
    ref.kind = (await readReference(ref)).kind;
    if (api.signal.aborted) return;
    ref.anchor.href = referenceUrl(ref);
    hideCard();
    if (external || !deskAvailable()) window.open(referenceUrl(ref), '_blank', 'noopener');
    else await showDesk(ref);
  } catch { if (!api.signal.aborted) void showCard(ref); }
}
async function showCard(ref) {
  clearTimeout(dismissTimer); activeReference = ref;
  card.replaceChildren(element('strong', { text: ref.id }), element('p', { class: 'muted', text: 'Loading from Desk…' }));
  if (!card.matches(':popover-open')) card.showPopover(); positionCard(ref);
  try {
    const result = await readReference(ref);
    if (api.signal.aborted || activeReference !== ref) return;
    ref.kind = result.kind; ref.anchor.href = referenceUrl(ref);
    const data = result.data; const row = result.kind === 'item' ? data.row : data.mission;
    const detail = element('div', { class: 'stack' }, element('div', { class: 'row' }, element('strong', { class: 'grow', text: ref.id }), element('span', { class: 'muted', text: ref.repo })), element('h3', { text: row.title }),
      element('p', { text: [row.status ?? row.state, row.priority, row.holder && `Held by ${row.holder}`].filter(Boolean).join(' · ') }));
    if (data.done_when?.length) detail.append(element('p', { class: 'muted', text: `${data.done_when.filter(line => line.ticked).length}/${data.done_when.length} completion checks` }));
    if (row.waiting_on?.length || data.blocked_by?.length) detail.append(element('p', { text: `Blocked by ${(row.waiting_on?.length ? row.waiting_on : data.blocked_by).join(', ')}` }));
    if (data.goal) detail.append(element('p', { class: 'muted', text: data.goal.slice(0, 240) }));
    const open = element('button', { text: 'Open in Desk' }); open.addEventListener('click', () => void activateReference(ref));
    const refresh = element('button', { text: 'Refresh' }); refresh.addEventListener('click', () => { referenceCache.delete(`${ref.repo}/${ref.id}`); void showCard(ref); });
    detail.append(element('p', { class: 'muted', text: `Read from ${data.read_from ?? 'Desk'} · cached up to 30 seconds` }), element('div', { class: 'row' }, open, refresh, element('a', { text: 'Open externally ↗', href: referenceUrl(ref), target: '_blank', rel: 'noopener noreferrer' })));
    card.replaceChildren(detail); ref.anchor.setAttribute('aria-label', `${ref.id}: ${row.title}`); positionCard(ref);
  } catch (error) {
    if (api.signal.aborted || activeReference !== ref) return;
    const retry = element('button', { text: 'Retry' }); retry.addEventListener('click', () => { referenceCache.delete(`${ref.repo}/${ref.id}`); void showCard(ref); });
    card.replaceChildren(element('strong', { text: ref.id }), element('p', { class: 'error', text: error.status === 404 ? `No item or mission ${ref.id} was found in ${ref.repo}.` : error.message }), retry); positionCard(ref);
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
  if (!preferences.references || !pattern || !conversation) { hideCard(); referenceLayer.replaceChildren(); return; }
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
  observer.disconnect(); clearTimeout(scanTimer); clearTimeout(hoverTimer); clearTimeout(dismissTimer); clearTimeout(domFrame); cancelAnimationFrame(rectFrame);
  hideCard(); restoreNative(); mount.remove(); tab.remove(); launch.remove(); settingsHost.remove(); CSS.highlights?.delete('t3-atelier-refs');
  for (const [node, position] of positions) node.style.position = position;
});
const routeTimer = setInterval(() => { if (routeSeen !== location.hash) { routeSeen = location.hash; if (host) void updateContext(); else scheduleDOM(); } }, 300);
disposers.push(() => clearInterval(routeTimer));
listen(window, 'hashchange', () => { routeSeen = location.hash; if (host) void updateContext(); });
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
scheduleDOM(); queueScan();
api.log('Atelier extension ready', { repositories: repositories.map(repo => repo.repo) });
