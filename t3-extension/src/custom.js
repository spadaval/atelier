// The views not yet rewritten as typed modules (atelier-40dc): the Inbox row and missions in T3's
// sidebar, the layer over T3's main area, the project strip under the new-thread composer and the
// project home from the breadcrumb. The typed core (src/core, src/host, src/panel) runs them;
// start() in main.ts calls startViews once the panel exists. The body is the script this file
// was, left unindented so its history follows it.
import { element, icon, icons } from './core/dom';
import { expectHostDom, hostDomExpected, hostListProblem, hostSelectors, observeHostTheme, readHostTheme, setHostProbe } from './host/adapter';
import { mountMissionLayer } from './mission/mount.tsx';
import { repositoryKey } from './repositories';
import { viewMissing, viewOn, views } from './views';

export function startViews(ext, { panel }) {
const api = ext.api;
const deskUrls = ext.deskUrls;
const settings = ext.settings;
const listen = (node, name, callback, options) => ext.listen(node, name, callback, options);
const scheduleDOM = () => ext.scheduleDOM();
const disposers = { push: dispose => ext.onDispose(dispose) };
const mappedRepository = project => ext.repositories.mapped(project);
const openThread = threadId => panel.openThread(threadId);
// Views outside the panel: the Inbox row in T3's sidebar, missions in T3's thread list, the layer
// over T3's main area, the project strip under the new-thread composer and the project home from
// the breadcrumb. Each needs only its own hooks; a missing one turns off that view alone.
// A DOM hook expected but not yet found is checked again each second until it is found or reported missing.
const expectTimer = setInterval(() => { if (Object.keys(hostDomExpected).length) scheduleDOM(); }, 1000);
// An item ID as Atelier writes them: lowercase letters and digits in dash-separated parts.
const itemIdPattern = /^[a-z0-9]+(?:-[a-z0-9]+)+$/;

// The layer over T3's main area, leaving the sidebar usable. It shows the extension's own mission
// page (src/mission), drawn in React with no frame, or a Desk page in one frame: the inbox, a
// project's home, an item or a proposed mission's record page. A framed page opened from the
// mission page has a Back button to it. The frame speaks the host contract: themed on desk:ready,
// desk:open-thread navigates T3 and closes the layer, desk:key Escape closes it. Its messages count
// only from its own frame's window with its Desk's origin, as the panel's do.
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
  if (message.type === 'desk:ready') { layerFrame.ready = true; sendLayerTheme(true); if (layer && !layer.mission) layerFrame.iframe.focus(); }
  else if (message.type === 'desk:open-thread' && typeof message.thread === 'string') { openThread(message.thread); closeLayer(); }
  else if (message.type === 'desk:key' && message.key === 'Escape') closeLayer();
});
disposers.push(observeHostTheme(() => sendLayerTheme(false)));
// layer: { key, desk, path, mission, back }; key is 'inbox', 'mission:…', 'home:…' or 'item:…'.
// `mission` ({ repo, id }) draws the native mission page instead of framing `path`; `back` is the
// layer a framed page was opened from.
let layer = null;
function openLayer(key, desk, path, mission = null, back = null) {
  // Without T3's main area the page opens in the browser at its normal address.
  if (!viewOn('layer')) { window.open(desk + path, '_blank', 'noopener'); return; }
  layer = { key, desk, path, mission, back };
  if (mission) {
    const from = layer;
    const openItem = id => openLayer(`item:${desk}/${mission.repo}/${id}`, desk, `/r/${encodeURIComponent(mission.repo)}/item/${encodeURIComponent(id)}`, null, from);
    missionLayer.show({ desk, repo: mission.repo, id: mission.id, onClose: closeLayer, openThread: thread => { openThread(thread); closeLayer(); }, openItem, openUrl: url => window.open(url, '_blank', 'noopener') });
  } else {
    missionLayer.hide();
    if (layerFrame.desk !== desk || layerFrame.path !== path) {
      Object.assign(layerFrame, { desk, path, ready: false, theme: '' });
      layerFrame.iframe.src = `${desk}/embed${path}`;
    }
  }
  // The layer follows at once: a covered window delays the DOM pass's timer.
  syncSidebar();
}
function closeLayer() { if (!layer) return; layer = null; missionLayer.hide(); syncSidebar(); }
// Opens the mission page for any mission of a Desk, as a mission's title does for the open ones the
// sidebar lists: for checks and the console, such as a done mission (README.md, "The mission page").
globalThis.__t3Custom.openMission = (repo, id, desk = deskUrls[0]) => openLayer(`mission:${desk}/${repo}/${id}`, desk, `/r/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}`, { repo, id });
const toggleLayer = (key, desk, path, mission = null) => layer?.key === key ? closeLayer() : openLayer(key, desk, path, mission);
// The mission page draws its own close and keys; a framed page gets the layer's close, and Back
// when it was opened from the mission page.
const layerClose = element('button', { 'data-t3-layer-close': '', 'aria-label': 'Close', title: 'Close (Esc)', html: icons.close });
const layerBack = element('button', { 'data-t3-layer-back': '', title: 'Back to the mission', html: `${icons.back}<span>Back</span>` });
const layerNative = element('div', { 'data-t3-layer-native': '' });
const missionLayer = mountMissionLayer(layerNative, api);
const layerNode = element('div', { 'data-t3-layer': '', hidden: '' });
listen(layerClose, 'click', closeLayer);
listen(layerBack, 'click', () => { const back = layer?.back; if (back) openLayer(back.key, back.desk, back.path, back.mission, back.back); });
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
  [data-t3-layer-close]:hover, [data-t3-layer-back]:hover { background: var(--accent); }
  [data-t3-layer-back] { position: absolute; top: 8px; right: 48px; z-index: 1; display: inline-flex; align-items: center; gap: 4px; height: 28px; padding: 0 10px 0 6px; border: 1px solid var(--border); border-radius: 6px; background: var(--card); color: var(--foreground); font: 13px var(--font-sans, system-ui, sans-serif); cursor: pointer; -webkit-app-region: no-drag; }
  [data-t3-layer-native] { width: 100%; height: 100%; }
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
const threadList = () => viewOn('missionThreads') ? ext.host?.threadList?.() ?? [] : [];
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
    const title = element('button', { 'data-t3-mission-title': '', title: `${mission.id}: ${mission.title} (${mission.state}). Open ${mission.state === 'proposed' ? 'its plan' : 'the mission page'}`, text: mission.title });
    // An agreed or running mission opens the mission page; a proposed plan, which that page does not
    // serve, its record page, framed.
    const path = `/r/${encodeURIComponent(entry.repo)}/mission/${encodeURIComponent(mission.id)}`;
    listen(title, 'click', () => toggleLayer(`mission:${entry.key}`, entry.desk, path, mission.state === 'proposed' ? null : { repo: entry.repo, id: mission.id }));
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
    // A folder's threads are the thread rows after its header. A closed folder keeps only the open
    // thread's row, which stays a plain row under it, with no mission rows.
    const threads = [];
    while (index < children.length && children[index].matches(hostSelectors.threadItem)) threads.push(children[index++]);
    folderThreads += threads.length;
    if (button.getAttribute('aria-expanded') !== 'true') { sequence.push(...threads); continue; }
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
// that thread), and every click with Shift held on a thread row, whatever other key is held, as
// T3's own rule for which of those is a range depends on the platform (a click and Ctrl- or
// Cmd-click without Shift still work).
listen(document, 'pointerdown', event => {
  if (layoutActive && event.target instanceof Element && event.target.closest(hostSelectors.settleButton)) event.stopPropagation();
}, { capture: true });
listen(document, 'click', event => {
  if (layoutActive && event.shiftKey && event.target instanceof Element && event.target.closest(hostSelectors.threadItem)) { event.preventDefault(); event.stopPropagation(); }
}, { capture: true });

function syncSidebar() {
  const onMain = Boolean(ext.host) && !location.hash.includes('/settings');
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
  // The layer holds the mission page or the frame, never both, so a native page has no iframe.
  // Moving the iframe reloads it, so it is moved only when the layer changes between them.
  const content = !shown ? [] : layer.mission ? [layerNative] : [layerFrame.iframe, ...(layer.back ? [layerBack] : []), layerClose];
  if (content.length && (content.length !== layerNode.children.length || content.some((node, at) => layerNode.children[at] !== node))) layerNode.replaceChildren(...content);
  // The pages take keys: the frame gets focus as it appears, and again on desk:ready for a page
  // still loading; the mission page focuses itself as it is drawn (src/mission/MissionPage.tsx).
  const appearing = shown && (layerNode.hidden || layerNode.dataset.shown !== layer.key);
  layerNode.hidden = !shown;
  layerNode.dataset.shown = shown ? layer.key : '';
  if (appearing && !layer.mission) layerFrame.iframe.focus();
  if (!shown && (document.activeElement === layerFrame.iframe || document.activeElement === layerNative)) document.activeElement.blur();
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
  const draft = ext.host?.draftContext?.();
  const reason = draft && ext.host.fillComposer ? await ext.host.fillComposer(draft.draftId, text) : 'T3 does not show a new-thread screen';
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
    return stripRow(() => thread ? ext.host.showThread(thread.environmentId, thread.id) : openItem(item.id), `${item.id}: ${item.title}${item.holder ? `, held by ${item.holder.title}` : ''}${thread ? '' : ' (opens in the Desk)'}`,
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
  const draft = settings.enabled ? ext.host?.draftContext?.() : null;
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
  expectHostDom('breadcrumbProject', Boolean(ext.current) && !location.hash.includes('/settings'), 'T3 showed a thread');
}
listen(document, 'click', event => {
  const target = event.target instanceof Element ? event.target.closest(hostSelectors.breadcrumbProject) : null;
  if (!target || !settings.enabled) return;
  const repo = mappedRepository(ext.current ?? ext.host?.draftContext?.() ?? null);
  if (!repo) return;
  event.preventDefault(); event.stopPropagation();
  toggleLayer(`home:${repositoryKey(repo)}`, repo.desk, `/home/${encodeURIComponent(repo.repo)}`);
}, { capture: true });

disposers.push(() => {
  clearInterval(deskTimer); clearInterval(missionsTimer); clearInterval(expectTimer); stripResize.disconnect();
  clearLayout(); extSidebar.remove(); layerNode.remove(); strip.remove(); viewStyle.remove(); layoutStyle.remove();
  document.documentElement.removeAttribute('data-t3-layer-open');
});
// Missions are placed before the browser paints what T3 changed in its list, and on the new
// list when T3 replaces it (a search, a route change).
ext.onMutation(records => {
  const list = document.querySelector(hostSelectors.projectFolder)?.parentElement ?? null;
  const sidebar = document.querySelector('[data-workspace-area="sidebar"]');
  if (list !== layoutList || records.some(record => sidebar?.contains(record.target))) syncMissionLayout();
});
ext.onDOM(syncProjectStrip);
ext.onDOM(syncProjectHome);
ext.onDOM(syncSidebar);
ext.diagnostics.views = () => ({ layer: layer && { ...layer, shown: !layerNode.hidden, ready: layerFrame.ready }, views: Object.fromEntries(Object.keys(views).map(name => [name, viewMissing(name)])) });
// A covered window throttles timers: when it is shown again, every view reads afresh (main.ts).
return { refresh() { readDesks(); launchAt = 0; refreshMissions(); } };
}
