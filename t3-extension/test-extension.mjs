import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import http from 'node:http';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';
import { connectPage } from './devtools.mjs';
import { fixture, startFixtureDesk } from './fixture-desk.mjs';

// The extension's folder, wherever the test is run from.
const root = path.dirname(fileURLToPath(import.meta.url));
const temp = await fs.mkdtemp(path.join(os.tmpdir(), 'atelier-extension-test-'));
const debugPort = 19222;
let deskBase, realDesk, fixtureDesk;
// The fixture repository's identity, as GET /api/repos reports it (fixture-desk.mjs).
const fixtureRepositoryKey = fixture.identity;
const referenceId = fixture.items.reference.id;
// Named only in thread-b's, thread-child's and other:thread-a's conversations.
const threadBReference = 'atelier-tb01';
const childReference = 'atelier-ch01';
const otherReference = 'atelier-oa01';
// A mission the conversation names; the first Desk answers 404 for it, so it resolves on the second.
const missionId = fixture.closedMission.id;
// A Chromium browser for the isolated fixture page; set BROWSER to override.
const browserPath = process.env.BROWSER ?? ({ win32: 'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe', darwin: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' }[process.platform] ?? '/usr/bin/google-chrome');
let browser, injector, page;
async function until(check, label, timeout = 20000) {
  const end = Date.now() + timeout; let lastError;
  while (Date.now() < end) {
    try { const result = await check(); if (result) return result; } catch (error) { lastError = error; }
    await delay(100);
  }
  throw new Error(`${label} timed out${lastError ? ': ' + lastError.message : ''}`);
}
async function control(action) {
  const child = spawn(process.execPath, [path.join(root, 't3-custom.mjs'), action, '--home', temp]);
  let output = ''; child.stdout.on('data', chunk => output += chunk); child.stderr.on('data', chunk => output += chunk);
  const code = await new Promise(resolve => child.on('close', resolve));
  if (code) throw new Error(`${action}: ${output}`);
  return JSON.parse(output);
}
const shell = {
  projects: [{ id: 'project-a', title: 'atelier', repositoryIdentity: { canonicalKey: fixtureRepositoryKey } }, { id: 'project-c', title: 'unlinked', repositoryIdentity: { canonicalKey: 'elsewhere/unlinked' } }],
  threads: [
    { id: 'thread-a', projectId: 'project-a', title: 'Thread A', status: 'idle', lineage: { parentThreadId: null } },
    { id: 'thread-b', projectId: 'project-a', title: 'Thread B', status: 'running', lineage: { parentThreadId: null } },
    { id: 'thread-c', projectId: 'project-c', title: 'Thread C', status: 'idle', lineage: { parentThreadId: null } },
    // A subagent of thread-a (delegate_task), and a thread a linked environment's agent started from thread-b.
    { id: 'thread-child', projectId: 'project-a', title: 'Child of A', status: 'idle', lineage: { parentThreadId: 'thread-a' } },
    { id: 'thread-delegated', projectId: 'project-a', title: 'Delegated from B', status: 'idle', lineage: { parentThreadId: null }, delegatedFrom: { environmentId: 'env-remote', threadId: 'thread-b', title: 'Thread B' } },
    // A thread of the mapped project that no mission holds.
    { id: 'thread-loose', projectId: 'project-a', title: 'Loose A', status: 'idle', lineage: { parentThreadId: null } },
    // An ID with a colon whose list item ends like thread-a's ("env:other:thread-a"); no mission holds it.
    { id: 'other:thread-a', projectId: 'project-c', title: 'Other A', status: 'idle', lineage: { parentThreadId: null } },
  ],
};
// T3's composer drafts: a new-thread screen for the mapped project and one for the unmapped one.
const drafts = { 'draft-a': { environmentId: 'env', projectId: 'project-a', logicalProjectKey: fixtureRepositoryKey }, 'draft-c': { environmentId: 'env', projectId: 'project-c', logicalProjectKey: 'elsewhere/unlinked' } };
// The Desk's launch data, served by the first Desk in place of the real Desk's: its rows include IDs
// no record can have. The sidebar comes from the real Desk, whose fixture repository holds one open
// mission led by thread-b, with a plan reviewer T3 does not know and an item held by thread-a.
const launchFixture = {
  ready: [{ id: 'atelier-r1', title: 'Ready one, with a title long enough to be cut short in its card', priority: 'high' }, { id: 'atelier-r2', title: 'Ready two', priority: 'medium' }, { id: 'atelier-r3; rm -rf /', title: 'Not an item ID' }, { id: 'atelier-r4', title: 'Ready four' }],
  waiting: 5, threads_waiting: [], waiting_items: [{ id: 'atelier-w1', title: 'Waiting one' }, { id: 'atelier-w2', title: 'Waiting two' }, { id: 'atelier-w3', title: 'Waiting three' }],
  in_motion: [{ id: 'atelier-x1', title: 'Held by thread B', stage: 'started', age_seconds: 7200, holder: { identity: 'thread-b', title: 'Thread B' } }, { id: 'atelier-x2', title: 'Held elsewhere', stage: 'review', age_seconds: 90000, holder: { identity: 'remote-holder', title: 'Remote holder' } }],
};
// Modules shaped like T3's private hooks, so the real discovery code runs. Their names
// differ from T3's chunk names, so discovery searches every declared module.
const chunks = {
  'panel-fixture.js': `const listeners = globalThis.fixtureListeners = new Set();
const byThreadKey = Object.fromEntries(['thread-a', 'thread-b', 'thread-c'].map(id => ['env:' + id, { isOpen: true, activeSurfaceId: 'files' }]));
const state = { byThreadKey, openBrowser() {}, show({ environmentId, threadId }) { byThreadKey[environmentId + ':' + threadId].isOpen = true; for (const listener of listeners) listener(); } };
export const s = { getState: () => state, subscribe: listener => { listeners.add(listener); return () => listeners.delete(listener); } };`,
  'registry-fixture.js': `export const r = { '~effect/reactivity/AtomRegistry': '~effect/reactivity/AtomRegistry', get: atom => atom };`,
  'shell-fixture.js': `let reads = 0; const snapshot = globalThis.fixtureSnapshot = ${JSON.stringify(shell)};
export const c = { familyAtom() {}, mountRows() {}, snapshotAtom: environmentId => environmentId === 'env-empty' ? { projects: [], threads: [] } : reads++ ? snapshot : { ...snapshot, threads: [] } };`,
  'environment-fixture.js': `export const e = { environmentIdsAtom: ['env', 'env-empty'], connectedEnvironmentIdsAtom: ['env', 'env-empty'] };`,
  'drafts-fixture.js': `const state = globalThis.fixtureDraftState = { draftThreadsByThreadKey: ${JSON.stringify(drafts)}, draftsByThreadKey: {} };
export const d = { getState: () => state };`,
  'index-fixture.js': `import './panel-fixture.js'; import './registry-fixture.js'; import './shell-fixture.js'; import './environment-fixture.js'; import './drafts-fixture.js';
new Worker('/assets/imageDecode.worker-fixture.js');`,
  // Like T3's image decoder worker: its top-level code must never run in the window.
  'imageDecode.worker-fixture.js': `if (typeof window !== 'undefined') globalThis.workerScriptRanInWindow = true;`,
};
// T3's boring sidebar: one flex column of li, a pinned header, each project folder's header with
// its thread rows after it, then the Settled shelf. Like T3, it closes a folder by removing its
// rows but the open thread's (routeThreadOnly), appending an inert, absolutely placed copy of each to the list, as T3 fades a removed row
// (Sidebar.motion.ts; here they stay, as in a hidden window, whose animations never finish);
// fadingCopy(node) makes one. A click on a row that reaches T3 is handled as T3 does on Windows:
// with Ctrl a toggle (fixtureList.toggles), else with Shift a range of its order
// (fixtureList.ranges), Meta or Alt held or not. And
// replaceList() draws a new list (as a search or a route change does). With
// fixtureList.boring false it is a flat list with no folders. A row's Settle button records a
// press that reaches T3's own handler (where T3 would start its drag-sweep) and a click.
const listFolders = [
  { key: `folder:${fixtureRepositoryKey}`, title: 'atelier', threads: ['thread-child', 'thread-loose', 'thread-a', 'thread-delegated', 'thread-b'] },
  { key: 'folder:elsewhere/unlinked', title: 'unlinked', threads: ['thread-c', 'other:thread-a'] },
];
const listScript = `const fixtureList=globalThis.fixtureList={boring:true,closed:new Set(),folders:${JSON.stringify(listFolders)},titles:${JSON.stringify(Object.fromEntries(shell.threads.map(thread => [thread.id, thread.title])))},sweeps:[],settled:[],ranges:[],toggles:[]};
const li=(attributes,html)=>{const node=document.createElement('li');for(const [name,value] of Object.entries(attributes))node.setAttribute(name,value);node.innerHTML=html;return node};
const threadNode=id=>{const node=li({'data-thread-item':'env:'+id},'<div data-sidebar-row="env:'+id+'" data-route="/env/'+id+'"><span></span><button aria-label="Settle thread">✓</button></div>');node.querySelector('span').textContent=fixtureList.titles[id]??id;return node};
globalThis.fadingCopy=row=>{const copy=row.cloneNode(true);copy.removeAttribute('data-thread-item');copy.setAttribute('aria-hidden','true');copy.inert=true;Object.assign(copy.style,{position:'absolute',top:'0px',left:'0px',width:'200px',opacity:'0.5'});return copy};
const routeRow=id=>location.hash==='#/env/'+id;
const folderRows=folder=>folder.threads.filter(id=>!fixtureList.closed.has(folder.key)||routeRow(id)).map(threadNode);
const folderNode=folder=>{const node=li({'data-testid':'sidebar-project-folder'},'<button data-sidebar-row="'+folder.key+'" aria-expanded="'+!fixtureList.closed.has(folder.key)+'"></button>');node.querySelector('button').textContent=folder.title;return node};
globalThis.replaceList=()=>{const list=document.createElement('ul');list.id='thread-list';list.append(...(fixtureList.boring?[li({'data-testid':'sidebar-pinned-header'},'Pinned'),...fixtureList.folders.flatMap(folder=>[folderNode(folder),...folderRows(folder)]),li({'data-testid':'sidebar-settled-header'},'Settled')]:fixtureList.folders.flatMap(folder=>folder.threads.map(threadNode))));document.querySelector('#thread-list').replaceWith(list)};
document.addEventListener('click',event=>{const folder=event.target.closest('#thread-list [data-testid=sidebar-project-folder] button');if(!folder)return;const spec=fixtureList.folders.find(entry=>entry.key===folder.dataset.sidebarRow);const header=folder.parentElement;const rowOf=id=>header.parentElement.querySelector('[data-thread-item="env:'+CSS.escape(id)+'"]');if(fixtureList.closed.has(spec.key)){fixtureList.closed.delete(spec.key);for(const id of spec.threads)rowOf(id)?.remove();header.after(...folderRows(spec))}else{fixtureList.closed.add(spec.key);for(const id of spec.threads){const row=rowOf(id);if(!row||routeRow(id))continue;header.parentElement.append(fadingCopy(row));row.remove()}}folder.setAttribute('aria-expanded',String(!fixtureList.closed.has(spec.key)))});
document.addEventListener('pointerdown',event=>{const button=event.target.closest('#thread-list [aria-label="Settle thread"]');if(button)fixtureList.sweeps.push(button.closest('[data-thread-item]').dataset.threadItem)});
document.addEventListener('click',event=>{const button=event.target.closest('#thread-list [aria-label="Settle thread"]');if(!button)return;event.stopPropagation();fixtureList.settled.push(button.closest('[data-thread-item]').dataset.threadItem)},true);
replaceList();`;
const html = `<!doctype html><html class="dark"><head><meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; connect-src 'self' blob: http: https: ws: wss:; style-src 'self' 'unsafe-inline'; frame-src 'self' blob: http: https:; worker-src 'self' blob:"><title>Atelier extension fixture</title><script type="module" src="/assets/index-fixture.js"></script>${Object.keys(chunks).filter(name => !name.includes('worker')).map(name => `<link rel="modulepreload" href="/assets/${name}">`).join('')}<style>html{--background:#202124;--foreground:#eeeeee;--card:#2a2b2e;--border:#555555;--muted-foreground:#a0a0a8;--secondary:#353639;--primary:#9bbcff}body{margin:0;background:var(--background);color:var(--foreground);font:14px Georgia, serif}#app{height:100vh;display:flex}[data-workspace-area=sidebar]{width:260px;flex-shrink:0;display:flex;flex-direction:column;border-right:1px solid #555}#thread-list{margin:0;padding:8px;list-style:none;overflow:auto;display:flex;flex-direction:column}[data-sidebar-row]{padding:8px;cursor:pointer;display:flex;gap:8px}[data-sidebar-row]>span{flex:1}main[data-slot=sidebar-inset]{position:relative;flex:1;min-width:0;display:flex;flex-direction:column}header{padding:12px;border-bottom:1px solid #555;display:flex;gap:12px}nav ol{display:flex;gap:8px;margin:0;padding:0;list-style:none}#workspace{display:flex;flex:1;min-height:0}[data-workspace-area=conversation]{flex:1;padding:20px;min-width:0;overflow:auto}[data-workspace-area=panel]{width:750px;display:flex;flex-direction:column;min-width:0}[data-right-panel-tabbar]{height:52px;display:flex;align-items:center}[data-right-panel-tabbar]>div{display:flex;align-items:center}[data-right-panel-surface-content]{position:relative;flex:1;min-height:0}#native-content{padding:20px}#settings{overflow:auto;padding:24px;max-width:850px}#composer{margin-top:30px;border:1px solid #555;padding:12px}#draft{flex:1;min-height:0}[data-chat-canvas]{position:relative;height:100%}[data-chat-composer-overlay]{position:absolute;inset:0;display:flex;align-items:center;pointer-events:none}[data-chat-composer-stack]{width:760px;max-width:calc(100% - 40px);margin:0 auto;pointer-events:auto}[data-testid=composer-editor]{min-height:60px;border:1px solid #555;padding:12px}</style></head><body><div id="app"><div data-workspace-area="sidebar"><div id="search-wrap"><div data-slot="sidebar-group"><input type="search" aria-label="Search threads"></div></div><ul id="thread-list"></ul></div><main data-slot="sidebar-inset"><header><nav aria-label="Thread breadcrumb"><ol><li><button id="crumb-project">atelier</button></li><li>/</li><li>Thread</li></ol></nav><span data-panel-layout-controls>Workspace controls</span></header><div id="workspace"><div data-workspace-area="conversation"><h1>Conversation</h1><p id="reference-prose">Investigate ${referenceId}, then <code>${referenceId}</code>. Ignore other-xabc and not-${referenceId}-extra.</p><pre>${referenceId}</pre><div id="composer" contenteditable="true">${referenceId}</div><p id="stream"></p><div id="thread-row"></div></div><div data-workspace-area="panel"><div data-right-panel-tabbar><div data-right-panel-tab-list><div data-active-tab="true"><button id="native-tab">Files</button></div><button aria-label="Add panel surface">+</button></div></div><div data-right-panel-surface-content><div id="native-content"><button id="native-control">Native file viewer</button></div></div></div></div><div id="draft" hidden><div data-chat-canvas><div data-workspace-area="composer" data-chat-composer-overlay><div data-chat-composer-stack><div data-testid="composer-editor" contenteditable="true" aria-label="Message"></div></div></div></div></div><div id="settings" hidden><div data-settings-page-scroll><div><h1>General</h1><p>T3 settings fixture</p></div></div></div></main></div><script>
const threadRows={'#/env/thread-a':{marked:'thread-a',text:''},'#/env/thread-b':{marked:'thread-b',text:'Thread B works on ${threadBReference}.'},'#/env/thread-child':{marked:'thread-child',text:'The child names ${childReference}.'},'#/env/other:thread-a':{text:'Other A names ${otherReference}.'}};
let rendered=0;
// Like T3, a thread's conversation is rendered as new row nodes, and each assistant message names its
// environment and thread; fixtureRenderDelay holds the previous conversation in the page that long.
const renderRows=route=>{const spec=threadRows[route]??{text:''};const row=document.createElement('div');row.id='thread-row';row.dataset.timelineRowId='row-'+(++rendered);const body=document.createElement('p');body.id='thread-prose';body.textContent=spec.text;if(spec.marked){const message=document.createElement('div');message.dataset.assistantCitationSource='message-'+rendered;message.dataset.assistantCitationEnvironment='env';message.dataset.assistantCitationThread=spec.marked;message.append(body);row.append(message)}else row.append(body);document.querySelector('#thread-row').replaceWith(row)};
const show=()=>{const route=location.hash;if(route.startsWith('#/env/')){const wait=globalThis.fixtureRenderDelay??0;if(wait)setTimeout(()=>renderRows(route),wait);else renderRows(route)}const settings=route.includes('/settings');const draft=route.startsWith('#/draft/');document.querySelector('#settings').hidden=!settings;document.querySelector('#draft').hidden=!draft;document.querySelector('#workspace').style.display=settings||draft?'none':'flex'};
addEventListener('hashchange',show);show();
// T3 starts a new thread from the breadcrumb's project, opens a thread from its row, and sends the composer on Enter.
document.querySelector('#crumb-project').addEventListener('click',()=>{globalThis.crumbStartedThread=true});
${listScript}
document.addEventListener('click',event=>{const row=event.target.closest('#thread-list [data-route]');if(!row)return;const key=row.closest('[data-thread-item]').dataset.threadItem;if(event.ctrlKey)fixtureList.toggles.push(key);else if(event.shiftKey)fixtureList.ranges.push(key);else location.hash=row.dataset.route});
const editor=document.querySelector('[data-testid=composer-editor]');
editor.addEventListener('keydown',event=>{if(event.key==='Enter')globalThis.fixtureSent=event.target.textContent});
// Like T3, the editor's text reaches the draft store a microtask later, under the draft the editor
// belongs to: the one shown, or fixtureEditorDraft for an editor T3 has not replaced yet.
editor.addEventListener('input',()=>{const draft=globalThis.fixtureEditorDraft??decodeURIComponent((location.hash.match(new RegExp('^#/draft/([^/]+)'))||[])[1]||'');if(draft)queueMicrotask(()=>{fixtureDraftState.draftsByThreadKey[draft]={prompt:editor.textContent.replaceAll(String.fromCharCode(160),' ')}})});
</script></body></html>`;
const server = http.createServer((request, response) => {
  const url = new URL(request.url, 'http://fixture');
  const chunk = chunks[url.pathname.replace('/assets/', '')];
  if (chunk) { response.writeHead(200, { 'content-type': 'text/javascript' }); response.end(chunk); return; }
  // A same-site frame in the page's process: it has the injector's binding and calls it directly.
  // The releases list, which the injector looks up when it starts: unavailable, so the
  // Customizations section must name the failed lookup.
  if (url.pathname === '/releases') { response.writeHead(503, { 'content-type': 'application/json' }); response.end('{}'); return; }
  if (url.pathname === '/spyframe') {
    response.writeHead(200, { 'content-type': 'text/html' });
    response.end(`<script>parent.postMessage({ spyBinding: typeof __t3CustomReport }, '*'); if (typeof __t3CustomReport === 'function') __t3CustomReport(JSON.stringify({ type: 'request', id: 'spy', action: 'desk-get', payload: { desk: new URL(location.href).searchParams.get('desk'), path: '/api/records/atelier-spyframe' } }));</script>`);
    return;
  }
  response.writeHead(200, { 'content-type': 'text/html' }); response.end(html);
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}`;
// The panel loads the real Desk app through these proxies, which answer GET /api/embed with the
// fixture's origin, so the real app talks the host contract to the fixture page. Each records every
// path it is asked for. /spoof is a page that posts Desk messages to its parent. The first Desk
// answers 404 for the mission's record, so it resolves on the second, which serves the same
// repositories; slowEmbed holds the first Desk's /api/embed answer back.
function deskProxyServer(requests, options) {
  return http.createServer(async (request, response) => {
    requests.push(request.url);
    if (request.url === '/api/embed') {
      if (options.slowEmbed) await delay(options.slowEmbed);
      response.writeHead(200, { 'content-type': 'application/json' }); response.end(JSON.stringify({ origins: [base] })); return;
    }
    // inboxError adds a repository the Desk reports it could not read, as GET /api/inbox does.
    if (request.url === '/api/inbox' && options.inboxError) {
      const inbox = await (await fetch(realDesk + request.url)).json();
      inbox.repos.push({ repo: 'unreadable', commit: null, error: 'git: not a repository', needs_human: [], proposed: [], threads: [] });
      response.writeHead(200, { 'content-type': 'application/json' }); response.end(JSON.stringify(inbox)); return;
    }
    if (request.url === '/api/sidebar' && options.sidebar) { response.writeHead(200, { 'content-type': 'application/json' }); response.end(JSON.stringify(options.sidebar ?? { repos: [] })); return; }
    if (request.url === '/api/launch?repo=atelier' && options.launch) { response.writeHead(200, { 'content-type': 'application/json' }); response.end(JSON.stringify(options.launch)); return; }
    if (options.missing?.includes(request.url)) { response.writeHead(404, { 'content-type': 'application/json' }); response.end(JSON.stringify({ error: 'not here' })); return; }
    if (request.url === '/spoof') {
      response.writeHead(200, { 'content-type': 'text/html' });
      response.end(`<script>parent.postMessage({ type: 'desk:navigated', path: '/spoofed', title: 'Spoofed' }, '*'); parent.postMessage({ type: 'desk:open-thread', thread: 'thread-c' }, '*'); parent.postMessage({ type: 'desk:key', key: 'Escape' }, '*');</script>`);
      return;
    }
    try {
      const upstream = await fetch(realDesk + request.url, { headers: { accept: request.headers.accept ?? '*/*' } });
      response.writeHead(upstream.status, { 'content-type': upstream.headers.get('content-type') ?? 'application/octet-stream' });
      response.end(Buffer.from(await upstream.arrayBuffer()));
    } catch (error) { response.writeHead(502); response.end(String(error)); }
  });
}
const deskRequests = [];
const secondDeskRequests = [];
const firstDesk = { slowEmbed: 0, missing: [`/api/records/${missionId}`], launch: launchFixture };
const deskProxy = deskProxyServer(deskRequests, firstDesk);
const secondDeskProxy = deskProxyServer(secondDeskRequests, { sidebar: { repos: [] } });
await new Promise(resolve => secondDeskProxy.listen(0, '127.0.0.1', resolve));
await new Promise(resolve => deskProxy.listen(0, '127.0.0.1', resolve));
try {
  const configPath = path.join(root, 'config.json');
  const config = JSON.parse(await fs.readFile(await fs.access(configPath).then(() => configPath, () => path.join(root, 'config.example.json')), 'utf8'));
  // The Desk the proxies forward to: DESK, a Desk serving the fixture repository, such as one
  // `node fixture-desk.mjs` started; else this checkout's atelier-desk, started here on it.
  if (!process.env.DESK) fixtureDesk = await startFixtureDesk();
  realDesk = (process.env.DESK ?? fixtureDesk.url).replace(/\/$/, '');
  const served = (await (await fetch(`${realDesk}/api/repos`)).json()).repos.find(repo => repo.name === fixture.repo);
  assert.equal(served?.identity, fixture.identity, `${realDesk} does not serve the fixture repository ${fixture.repo} (${fixture.identity}); start one with node fixture-desk.mjs`);
  // Another site than the fixture, as the real Desk is, so its frame is a separate target.
  deskBase = `http://localhost:${deskProxy.address().port}`;
  const secondDeskBase = `http://localhost:${secondDeskProxy.address().port}`;
  // No repository table: the project is matched by the identity the Desk reports.
  config.desk = { urls: [deskBase, secondDeskBase] }; delete config.settings;
  config.debugPort = debugPort; config.targetPrefixes = [base]; config.releases = `${base}/releases`;
  await fs.writeFile(path.join(temp, 'config.json'), JSON.stringify(config));
  await fs.copyFile(path.join(root, 'custom.css'), path.join(temp, 'custom.css'));
  // The built extension (pnpm build), with the real adapter and its module discovery; the wrapper
  // only exposes the bridge to the test.
  await fs.writeFile(path.join(temp, 'custom.js'), await fs.readFile(path.join(root, 'dist/custom.js'), 'utf8') + `
const startExtension = T3Custom.start;
T3Custom.start = api => { globalThis.fixtureApi = api; return startExtension(api); };`);
  const browserLog = await fs.open(path.join(temp, 'browser.log'), 'w');
  const env = { ...process.env }; delete env.ELECTRON_RUN_AS_NODE; delete env.NODE_OPTIONS;
  browser = spawn(browserPath, ['--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking', `--user-data-dir=${path.join(temp, 'profile')}`, `--remote-debugging-port=${debugPort}`, '--remote-debugging-address=127.0.0.1', '--window-size=1600,1000', base + '/#/env/thread-a'], { env, stdio: ['ignore', browserLog.fd, browserLog.fd] });
  await browserLog.close();
  // The first start of Chrome on a machine that has not read its files is slow: on 20 fresh
  // GitHub runners it took 0.6 to 47 seconds until this port answered, and 0.2 to 0.4 the second
  // time (atelier-4bxj). So this wait is longer than the others, and the time is printed.
  const browserStarted = Date.now();
  page = await until(() => connectPage(debugPort, base), 'Browser startup', 120000);
  console.log(`Browser started in ${((Date.now() - browserStarted) / 1000).toFixed(1)}s`);
  const injectorLog = await fs.open(path.join(temp, 'injector.log'), 'w');
  injector = spawn(process.execPath, [path.join(root, 't3-custom.mjs'), 'daemon', '--home', temp], { stdio: ['ignore', injectorLog.fd, injectorLog.fd] });
  await injectorLog.close();
  const evaluate = expression => page.evaluate(expression);
  await until(() => evaluate('Boolean(document.querySelector("[data-t3-desk-launch]:not([hidden])"))'), 'Extension initialization and thread context');
  const discovery = await evaluate(`({ workerLoaded: performance.getEntriesByType('resource').some(entry => entry.name.includes('.worker-')), ranInWindow: globalThis.workerScriptRanInWindow ?? false, modules: __t3Custom.diagnostics().modules, probes: Object.entries(__t3Custom.diagnostics().probes).filter(([name]) => name.startsWith('module:')).map(([name, probe]) => name + '=' + probe.ok) })`);
  assert(discovery.workerLoaded, 'the fixture worker should be among the page resources');
  assert.equal(discovery.ranInWindow, false, `discovery imported a worker script into the window: ${discovery.modules.join(', ')}`);
  assert.deepEqual(discovery.probes, ['module:workspace-panel-store=true', 'module:atom-registry=true', 'module:thread-snapshot-atom=true', 'module:environment-atoms=true', 'module:composer-draft-store=true']);
  console.log(`PASS: real discovery found every hook in ${discovery.modules.length} declared modules and never ran the worker script in the window`);
  await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  await until(() => evaluate('Boolean(document.querySelector("[data-right-panel-surface-content] > [data-t3-desk-mount]:not([hidden])"))'), 'Docked Desk');
  // The frame loads once every Desk has listed its repositories.
  const iframeSrc = await until(() => evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").src'), 'Desk iframe address');
  assert.equal(iframeSrc, deskBase + '/embed/', 'the panel loads the embedded mode and leaves the page to the Desk');
  assert.equal(await evaluate('__t3Custom.diagnostics().selectedRepository'), 'atelier');
  assert(await evaluate('document.querySelector("#native-content").inert'));
  const geometry = await evaluate('(()=>{const host=document.querySelector("[data-right-panel-surface-content]").getBoundingClientRect();const frame=document.querySelector("[data-t3-desk-mount]").getBoundingClientRect();return {hostWidth:host.width,frameWidth:frame.width,hostLeft:host.left,frameLeft:frame.left}})()');
  assert.equal(geometry.hostWidth, geometry.frameWidth); assert.equal(geometry.hostLeft, geometry.frameLeft);
  await evaluate('document.querySelector("[data-workspace-area=panel]").style.width="900px"');
  assert.equal(await evaluate('document.querySelector("[data-t3-desk-mount]").getBoundingClientRect().width'), 900);
  assert.equal(await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#extension-settings")'), null, 'no floating gear');
  console.log('PASS: Desk uses the real panel content slot, matches the project by the identity the Desk reports with no repository table, loads /embed, and resizes with the host');

  const iframeTarget = await until(async () => (await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json()).find(target => target.type === 'iframe' && target.url.startsWith(deskBase)), 'Real Desk iframe');
  const child = await page.call('Target.attachToTarget', { targetId: iframeTarget.id, flatten: true });
  await until(async () => (await page.call('Runtime.evaluate', { expression: 'document.readyState === "complete" && document.body.innerText.includes("atelier") && !document.querySelector(".skel")', returnByValue: true }, child.sessionId)).result.value, 'Real Desk application rendering');
  const inFrame = async expression => (await page.call('Runtime.evaluate', { expression, returnByValue: true }, child.sessionId)).result.value;
  const shadow = 'document.querySelector("[data-t3-desk-mount]").shadowRoot';
  // Theme: sent on desk:ready from T3's custom properties, as computed colours.
  await until(async () => (await inFrame('document.documentElement.style.getPropertyValue("--bg")')) === 'rgb(32, 33, 36)', 'host:theme on desk:ready');
  assert.equal(await inFrame('document.documentElement.style.getPropertyValue("--ink")'), 'rgb(238, 238, 238)');
  assert.equal(await inFrame('document.documentElement.style.getPropertyValue("--panel")'), 'rgb(42, 43, 46)');
  assert.equal(await inFrame('document.documentElement.style.getPropertyValue("--you")'), 'rgb(155, 188, 255)');
  assert.match(await inFrame('document.documentElement.style.getPropertyValue("--sans")'), /Georgia/);
  assert.equal(await inFrame('document.documentElement.dataset.theme ?? "dark"'), 'dark');
  // host:thread on desk:ready: the real Desk opens its "this thread" page for the thread, and reads
  // the thread, the ties of the child T3 lists for it (children) and the record its conversation
  // names (mentioned), so the whole message reached it.
  await until(() => deskRequests.includes('/api/ties?thread=thread-a'), 'host:thread on desk:ready');
  await until(async () => (await inFrame('location.pathname')) === '/embed/t', 'Desk follows the thread to /embed/t');
  await until(() => deskRequests.includes('/api/ties?thread=thread-child') && deskRequests.includes(`/api/records/${referenceId}`), 'The Desk read the children and mentioned of host:thread');
  // Title: the tab shows the Desk's page title from desk:navigated; Open externally is not embedded.
  const deskTitle = await until(async () => { const title = await inFrame('document.title'); return title && await evaluate('document.querySelector("[data-t3-desk-label]").textContent') === title && title; }, 'Tab label from desk:navigated');
  assert.notEqual(deskTitle, 'Desk');
  assert.equal(await evaluate(`${shadow}.querySelector("header")`), null, 'no doubled header in the panel');
  await until(async () => (await evaluate('document.querySelector("[data-t3-desk-external]").href')) === deskBase + '/t', 'Open externally follows desk:navigated');
  console.log(`PASS: embedded Desk gets T3's theme and thread-a on ready and opens its page; the tab shows "${deskTitle}" from desk:navigated; Open externally uses the normal address`);
  // A theme change on T3's root element is sent again.
  await evaluate('document.documentElement.classList.remove("dark"); document.documentElement.style.setProperty("--background", "#fafafa"); document.documentElement.style.setProperty("--foreground", "#111111")');
  await until(async () => (await inFrame('document.documentElement.style.getPropertyValue("--bg")')) === 'rgb(250, 250, 250)', 'host:theme after a theme change');
  assert.equal(await inFrame('document.documentElement.dataset.theme'), 'light');
  console.log('PASS: a change of T3 theme on its root element is sent to the Desk');
  // Messages count only from the Desk iframe's window AND the Desk's origin. First the Desk's own
  // origin from another window: a second frame loaded from the Desk.
  const hashBefore = await evaluate('location.hash');
  const spoofed = async label => {
    await delay(1000);
    assert.equal(await evaluate('location.hash'), hashBefore, `${label}: desk:open-thread was accepted`);
    assert.notEqual(await evaluate('document.querySelector("[data-t3-desk-label]").textContent'), 'Spoofed', `${label}: desk:navigated was accepted`);
    assert.notEqual(await evaluate('__t3Custom.diagnostics().frame.path'), '/spoofed', `${label}: desk:navigated was accepted`);
  };
  await evaluate(`(()=>{const f=document.createElement('iframe');f.id='spoof';f.src='${deskBase}/spoof';document.body.append(f)})()`);
  await until(() => deskRequests.includes('/spoof'), 'Spoofing frame from the Desk origin loaded');
  await spoofed('the Desk origin from another window');
  await evaluate('document.querySelector("#spoof").remove()');
  console.log('PASS: Desk messages from the Desk origin in another window are ignored');
  // desk:open-thread navigates T3 to the thread's route, or says it is in no connected environment.
  const post = message => inFrame(`parent.postMessage(${JSON.stringify(message)}, location.ancestorOrigins[0])`);
  // From here every host:thread the Desk frame receives is recorded.
  await inFrame(`window.hostThreads = []; window.hostMessages = []; addEventListener('message', event => { if (event.data?.type === 'host:thread') { hostThreads.push(event.data.thread); hostMessages.push(event.data); } })`);
  await post({ type: 'desk:open-thread', thread: 'thread-unknown' });
  await until(() => evaluate(`${shadow}.querySelector(".notice:not([hidden])")?.textContent.includes("thread-unknown is not in any connected T3 environment")`), 'Notice for an unknown thread');
  assert.equal(await evaluate('location.hash'), hashBefore);
  await post({ type: 'desk:open-thread', thread: 'thread-b' });
  await until(async () => (await evaluate('location.hash')) === '#/env/thread-b', 'desk:open-thread navigation');
  await evaluate('location.hash = "/env/thread-a"');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").hidden'), 'Desk back on thread-a');
  console.log('PASS: desk:open-thread opens the fixture route #/env/thread-b; an unknown thread is reported in the panel');
  // host:thread on each switch to another thread; a thread's mentions may follow as an update.
  const switches = () => inFrame('hostThreads.filter((thread, n, all) => thread !== all[n - 1])');
  await until(async () => JSON.stringify(await switches()) === '["thread-b","thread-a"]', 'host:thread on thread switches');
  assert(deskRequests.includes('/api/ties?thread=thread-b'));
  await delay(1000);
  const sentBeforeSettings = await inFrame('hostMessages.length');
  await evaluate('location.hash = "/settings/general"');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings"))'), 'Settings between threads');
  await evaluate('location.hash = "/env/thread-a"');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").hidden && __t3Custom.diagnostics().context?.threadId === "thread-a"'), 'Back on thread-a from Settings');
  await delay(1000);
  assert.equal(await inFrame('hostMessages.length'), sentBeforeSettings, 'the same message was sent again after Settings and back');
  // host:thread names the project's repository, the threads this one started in T3 (lineage.parentThreadId
  // for thread-a's subagent, delegatedFrom for the thread a linked environment started from thread-b) and
  // the IDs its conversation names, and is sent again when either list changes.
  // Each thread's message names only what its own conversation names: thread-b's never carries
  // thread-a's list, and thread-a's never carries thread-b's.
  // thread-b, never scanned before, is sent at once with no mentions; any later message carries its
  // own (it was shown only briefly here), never thread-a's.
  const threadBMessages = await inFrame('hostMessages.filter(message => message.thread === "thread-b")');
  assert.deepEqual(threadBMessages[0], { type: 'host:thread', thread: 'thread-b', repository: 'atelier', children: ['thread-delegated'], mentioned: [] });
  assert(threadBMessages.every(message => message.mentioned.length === 0 || JSON.stringify(message.mentioned) === JSON.stringify([referenceId, threadBReference])), JSON.stringify(threadBMessages));
  assert.deepEqual(await inFrame('hostMessages.filter(message => message.thread === "thread-a").map(message => message.mentioned)'), [[referenceId]]);
  const lastMessage = () => inFrame('hostMessages.at(-1)');
  assert.deepEqual(await lastMessage(), { type: 'host:thread', thread: 'thread-a', repository: 'atelier', children: ['thread-child'], mentioned: [referenceId] });
  await evaluate(`(()=>{const p=document.createElement('p');p.id='mention-change';p.textContent='${missionId}';document.querySelector('[data-workspace-area=conversation]').append(p)})()`);
  await until(async () => JSON.stringify((await lastMessage()).mentioned) === JSON.stringify([missionId, referenceId].sort()), 'host:thread again when an ID is mentioned');
  await evaluate('document.querySelector("#mention-change").remove()');
  await until(async () => JSON.stringify((await lastMessage()).mentioned) === JSON.stringify([referenceId]), 'host:thread again when a mention goes');
  await evaluate(`fixtureSnapshot.threads.push({ id: 'thread-new-child', projectId: 'project-a', title: 'New child', status: 'running', lineage: { parentThreadId: 'thread-a' } })`);
  await until(async () => JSON.stringify((await lastMessage()).children) === '["thread-child","thread-new-child"]', 'host:thread again when a child appears', 10000);
  await evaluate('fixtureSnapshot.threads.pop()');
  await until(async () => JSON.stringify((await lastMessage()).children) === '["thread-child"]', 'host:thread again when a child goes', 10000);
  // With reference links off nothing is scanned: the thread shown and the next one mention nothing.
  const referencesToggle = 'document.querySelector("#t3-custom-settings").shadowRoot.querySelector(\'[data-setting="references"]\')';
  await evaluate('location.hash = "/settings/general"');
  await until(() => evaluate(`Boolean(${referencesToggle})`), 'Settings section for the references toggle');
  await evaluate(`${referencesToggle}.click()`);
  await evaluate('location.hash = "/env/thread-a"');
  await until(async () => { const last = await lastMessage(); return last.thread === 'thread-a' && last.mentioned.length === 0; }, 'No mentions with references off');
  await evaluate('location.hash = "/env/thread-b"');
  await until(async () => (await lastMessage()).thread === 'thread-b', 'thread-b with references off');
  assert.deepEqual((await lastMessage()).mentioned, [], 'a thread kept mentions with references off');
  await evaluate('location.hash = "/settings/general"');
  await until(() => evaluate(`Boolean(${referencesToggle})`), 'Settings section again');
  await evaluate(`${referencesToggle}.click()`);
  await evaluate('location.hash = "/env/thread-a"');
  await until(async () => { const last = await lastMessage(); return last.thread === 'thread-a' && JSON.stringify(last.mentioned) === JSON.stringify([referenceId]); }, 'Mentions back with references on');
  console.log('PASS: host:thread carries the repository, children by lineage and by delegatedFrom, and the mentioned IDs of its own conversation only (none with references off), and is sent again when they change');
  // T3 can show the previous thread's conversation for a while after a switch; a scan of it is not
  // credited to the new thread. First by the thread its assistant messages name (thread-b, then thread-a)...
  const messagesSince = async (count, thread) => (await inFrame(`hostMessages.slice(${count})`)).filter(message => message.thread === thread);
  await evaluate('location.hash = "/env/thread-b"');
  await until(async () => { const last = await lastMessage(); return last.thread === 'thread-b' && last.mentioned.includes(threadBReference); }, 'thread-b scanned');
  let sentSoFar = await inFrame('hostMessages.length');
  await evaluate('globalThis.fixtureRenderDelay = 1200; location.hash = "/env/thread-a"');
  await delay(2500);
  assert.deepEqual((await messagesSince(sentSoFar, 'thread-a')).map(message => message.mentioned), [[referenceId]], 'thread-b\'s conversation was credited to thread-a');
  // ...then, with no assistant message, while its root and first row are the nodes last scanned for
  // the previous thread (thread-c, then other:thread-a).
  await evaluate('globalThis.fixtureRenderDelay = 0; location.hash = "/env/thread-c"');
  await until(async () => { const last = await lastMessage(); return last.thread === 'thread-c' && JSON.stringify(last.mentioned) === JSON.stringify([referenceId]); }, 'thread-c scanned');
  sentSoFar = await inFrame('hostMessages.length');
  await evaluate('globalThis.fixtureRenderDelay = 1200; location.hash = "/env/other:thread-a"');
  await until(async () => (await messagesSince(sentSoFar, 'other:thread-a')).some(message => message.mentioned.includes(otherReference)), 'other:thread-a scanned once T3 rendered it', 8000);
  assert.deepEqual((await messagesSince(sentSoFar, 'other:thread-a')).map(message => message.mentioned), [[], [otherReference, referenceId].sort()], 'thread-c\'s conversation was credited to other:thread-a');
  // A thread that streams is announced at once, and its mentions follow while it still streams.
  await evaluate('globalThis.fixtureRenderDelay = 0; globalThis.streamer = setInterval(() => document.querySelector("#stream").append(document.createTextNode(" more")), 100)');
  sentSoFar = await inFrame('hostMessages.length');
  await evaluate('location.hash = "/env/thread-child"');
  await until(async () => (await messagesSince(sentSoFar, 'thread-child')).length > 0, 'host:thread for a thread that streams', 2000);
  await until(async () => (await messagesSince(sentSoFar, 'thread-child')).some(message => message.mentioned.includes(childReference)), 'Mentions of a thread while it streams', 4000);
  assert(await evaluate('Boolean(globalThis.streamer)'), 'the stream stopped before the check');
  await evaluate('clearInterval(globalThis.streamer); globalThis.streamer = null; document.querySelector("#stream").textContent = ""; location.hash = "/env/thread-a"');
  await until(async () => (await lastMessage()).thread === 'thread-a', 'Back on thread-a');
  console.log('PASS: a scan of the previous thread\'s conversation is not credited to the next, by the thread its messages name or by its nodes; a streaming thread is sent at once and its mentions follow while it streams');
  // When T3 re-renders its panel slot the mount moves and the iframe reloads: the fresh Desk is told the thread again.
  const tiesBefore = deskRequests.filter(url => url === '/api/ties?thread=thread-a').length;
  await evaluate('document.querySelector("[data-t3-desk-mount]").remove()');
  await until(() => deskRequests.filter(url => url === '/api/ties?thread=thread-a').length === tiesBefore + 1, 'host:thread to a Desk reloaded by a re-rendered slot');
  assert.equal(await evaluate('__t3Custom.error'), null);
  console.log('PASS: host:thread is sent on ready, on each thread switch and to a reloaded Desk, never twice to the same Desk for the same thread');
  // The Desk iframe's own window with another origin: the frame is sent to a page of the second
  // Desk, which posts the same messages. Refresh brings the Desk back.
  const reloadedFrame = await until(async () => (await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json()).find(target => target.type === 'iframe' && target.url.startsWith(deskBase)), 'Reloaded Desk iframe');
  const reloadedSession = (await page.call('Target.attachToTarget', { targetId: reloadedFrame.id, flatten: true })).sessionId;
  await page.call('Runtime.evaluate', { expression: `location.href = '${secondDeskBase}/spoof'` }, reloadedSession).catch(() => {});
  await until(() => secondDeskRequests.includes('/spoof'), 'Desk frame sent to another origin');
  await spoofed('the Desk window with another origin');
  await evaluate('document.querySelector("[data-t3-desk-controls] [aria-label=\'Refresh Desk\']").click()');
  await until(() => evaluate('__t3Custom.diagnostics().frame.ready && __t3Custom.diagnostics().frame.title !== ""'), 'Desk back after Refresh');
  console.log('PASS: Desk messages from the Desk iframe window with another origin are ignored');

  // Real backend data is checked through the same allow-listed bridge as hovercards.
  const record = await (await fetch(`${realDesk}/api/records/${referenceId}`)).json();
  assert.equal((await evaluate(`fixtureApi.request('desk-get',{desk:'${deskBase}',path:'/api/repos/atelier/items/${referenceId}'})`)).row.id, referenceId);
  const denied = await evaluate(`fixtureApi.request('desk-get',{desk:'${deskBase}',path:'/api/action'}).then(()=>false,error=>error.message)`);
  assert.match(denied, /not allowed/);
  const otherDesk = await evaluate(`fixtureApi.request('desk-get',{desk:'http://127.0.0.1:1',path:'/api/repos'}).then(()=>false,error=>error.message)`);
  assert.match(otherDesk, /not in desk.urls/);
  // Paths that URL parsing would turn into another one are refused before anything is fetched.
  for (const sneaky of [String.raw`/api/repos/..\..\api\action?/items/x`, '/api/repos/%2e%2e/%2e%2e/api/action/items/x', '/api/repos/../../api/action/items/x', '/api/repos/a?b=/items/x']) {
    const refused = await evaluate(`fixtureApi.request('desk-get',{desk:'${deskBase}',path:${JSON.stringify(sneaky)}}).then(()=>false,error=>error.message)`);
    assert.match(String(refused), /not allowed/, `${sneaky} was read`);
  }
  assert(!deskRequests.some(url => url.startsWith('/api/action')), `a normalised path reached the Desk: ${deskRequests.filter(url => url.startsWith('/api/action')).join(', ')}`);
  // Settings that would break the extension are refused, and config.json is left as it was.
  const configBefore = await fs.readFile(path.join(temp, 'config.json'), 'utf8');
  for (const bad of [{ mappings: null }, { mappings: [] }, { mappings: { a: 1 } }, { enabled: null }, { references: 'yes' }, { toString: null }, { constructor: null }, { pinnedRepo: 3 }]) {
    const refused = await evaluate(`fixtureApi.request('save-settings', ${JSON.stringify(bad)}).then(()=>false,error=>error.message)`);
    assert.match(String(refused), /save-settings:/, `${JSON.stringify(bad)} was saved`);
  }
  assert.equal(await fs.readFile(path.join(temp, 'config.json'), 'utf8'), configBefore);
  // Only the top frame's main world may use the injector's binding.
  await evaluate(`addEventListener('message', event => { if (event.data?.spyBinding) globalThis.spyBinding = event.data.spyBinding; })`);
  await evaluate(`(()=>{const f=document.createElement('iframe');f.id='spyframe';f.src='/spyframe?desk=${encodeURIComponent(deskBase)}';document.body.append(f)})()`);
  assert.equal(await until(() => evaluate('globalThis.spyBinding'), 'Same-site frame loaded'), 'function', 'the same-site frame should see the binding, or this check proves nothing');
  await delay(1500);
  assert(!deskRequests.includes('/api/records/atelier-spyframe'), 'a same-site frame used the injector bridge');
  await evaluate('document.querySelector("#spyframe").remove()');
  console.log('PASS: authenticated-free Desk reads work through the bridge; action endpoints, unconfigured Desks, normalised paths, bad settings and other frames are rejected');

  await until(() => evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelectorAll("[data-atelier-id]").length === 2'), 'Reference scanning');
  const prose = await evaluate('document.querySelector("#reference-prose").textContent');
  await evaluate('globalThis.originalTextNode=document.querySelector("#reference-prose").firstChild;document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]").focus()');
  await until(() => evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard h3")?.textContent'), 'Hovercard backend response');
  assert.equal(await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard h3").textContent'), record.title);
  assert.equal(await evaluate('document.querySelector("#reference-prose").textContent'), prose);
  assert(await evaluate('document.querySelector("#reference-prose").firstChild === globalThis.originalTextNode'));
  await evaluate(`document.querySelector('#stream').append(document.createTextNode('Streaming ${referenceId}'));`);
  await until(() => evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelectorAll("[data-atelier-id]").length === 3'), 'Streaming reference update');
  assert.equal(await evaluate('document.querySelector("#stream").textContent'), `Streaming ${referenceId}`);
  await evaluate('(()=>{const link=document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]");link.blur();link.focus()})()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button"))'), 'Cached hovercard');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button").click()');
  await until(() => evaluate(`document.querySelector('[data-t3-desk-mount]').shadowRoot.querySelector('iframe').src === '${deskBase}/embed/r/atelier/item/${referenceId}'`), 'Reference opens in Desk');
  // The record stays: the reloaded Desk is not sent the same thread, so it does not move to the thread's page.
  await until(() => evaluate(`__t3Custom.diagnostics().frame.ready && __t3Custom.diagnostics().frame.title !== ''`), 'Desk ready on the record');
  await delay(1500);
  assert.equal(await evaluate('__t3Custom.diagnostics().frame.path'), `/r/atelier/item/${referenceId}`);
  // Open the reference again and switch to thread-b before the Desk is ready: thread-b is still sent.
  const tiesB = deskRequests.filter(url => url === '/api/ties?thread=thread-b').length;
  firstDesk.slowEmbed = 2500;
  await evaluate('(()=>{const link=document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]");link.blur();link.focus()})()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button"))'), 'Hovercard before a slow load');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button").click()');
  await until(() => evaluate('__t3Custom.diagnostics().frame.ready === false'), 'Reference load started');
  await evaluate('location.hash = "/env/thread-b"');
  await until(() => evaluate('__t3Custom.diagnostics().context?.threadId === "thread-b"'), 'Switched to thread-b during the load');
  assert.equal(await evaluate('__t3Custom.diagnostics().frame.ready'), false, 'the Desk was ready before the switch; the check proves nothing');
  firstDesk.slowEmbed = 0;
  await until(() => deskRequests.filter(url => url === '/api/ties?thread=thread-b').length === tiesB + 1, 'host:thread for a thread switched to during a reference load');
  console.log('PASS: a switch to another thread while a reference loads is sent when the Desk is ready');
  console.log('PASS: references by the Desk repository names, inline code, real hovercards, click-through, and streaming without text rewriting');

  await evaluate('globalThis.keptFrame=document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe");location.hash="/env/thread-b"');
  await delay(500); await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").hidden'), 'Desk on second thread');
  assert(await evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe") === globalThis.keptFrame'));
  assert((await evaluate('globalThis.keptFrame.src')).endsWith('/item/' + referenceId));
  await evaluate(`(()=>{const p=document.createElement('p');p.id='mission-reference';p.textContent='${missionId}';document.querySelector('[data-workspace-area=conversation]').append(p)})()`);
  await until(() => evaluate(`Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id=${missionId}]"))`), 'Mission reference scanning');
  await evaluate(`(()=>{document.getSelection().removeAllRanges();const p=document.querySelector('#mission-reference');const range=document.createRange();range.selectNodeContents(p);const r=range.getBoundingClientRect();p.dispatchEvent(new MouseEvent('click',{bubbles:true,cancelable:true,clientX:r.left+3,clientY:r.top+3}))})()`);
  await until(() => evaluate(`globalThis.keptFrame.src.includes("/embed/r/atelier/mission/${missionId}")`), 'Direct mission reference click');
  // The first Desk has no such record; the second, which also serves atelier, answered, so it opens there.
  assert.equal(await evaluate('globalThis.keptFrame.src'), `${secondDeskBase}/embed/r/atelier/mission/${missionId}`, 'the record opened on another Desk than the one that answered');
  assert(deskRequests.includes(`/api/records/${missionId}`) && secondDeskRequests.includes(`/api/records/${missionId}`), 'the mission was resolved through /api/records');
  assert(![...deskRequests, ...secondDeskRequests].some(url => url.includes(`/items/${missionId}`)), 'a mission was looked up as an item first');
  console.log('PASS: direct mouse activation resolves a mission with one /api/records read per Desk and opens it on the Desk that answered');
  await evaluate('location.hash="/env/thread-c"');
  await delay(500); await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  await until(() => evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").hidden'), 'Unmapped project empty state');
  console.log('PASS: same-project thread switches preserve Desk navigation; unmapped projects hide stale repository content');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]").focus()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard h3"))'), 'Reference in unmapped project');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button").click()');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").hidden'), 'Cross-project reference override');
  console.log('PASS: explicit reference navigation works from an unmapped project');

  // The sidebar views, the layer over T3's main area, and the project strip.
  const layerState = () => evaluate('__t3Custom.diagnostics().layer');
  const layerSrc = () => evaluate('document.querySelector("[data-t3-layer] iframe").src');
  const inboxCountOf = data => data.repos.reduce((sum, repo) => sum + (repo.needs_human ?? []).reduce((n, group) => n + group.items.length, 0) + (repo.proposed ?? []).length + (repo.threads ?? []).length, 0);
  // Both Desks forward /api/inbox to the same real Desk, so the count is twice what it reports.
  await until(async () => (await evaluate('document.querySelector("[data-t3-inbox-count]")?.textContent')) === String(2 * inboxCountOf(await (await fetch(`${realDesk}/api/inbox`)).json())), 'Inbox count from every Desk');
  assert(await evaluate('document.querySelector("#search-wrap").nextElementSibling.matches("[data-t3-ext-sidebar]")'), 'the Inbox row goes right under T3\'s search group');
  // A covered window throttles timers, so the views read the Desk again when the window is shown.
  const reads = path => deskRequests.filter(url => url === path).length;
  const [inboxReads, sidebarReads] = [reads('/api/inbox'), reads('/api/sidebar')];
  await evaluate('document.dispatchEvent(new Event("visibilitychange"))');
  await until(() => reads('/api/inbox') > inboxReads && reads('/api/sidebar') > sidebarReads, 'Inbox and missions read again when the window is shown');
  // A repository the Desk could not read is not a clean count: "!" beside what could be read, the error in the tooltip.
  firstDesk.inboxError = true;
  await evaluate('document.dispatchEvent(new Event("visibilitychange"))');
  const readable = 2 * inboxCountOf(await (await fetch(`${realDesk}/api/inbox`)).json());
  await until(async () => (await evaluate('document.querySelector("[data-t3-inbox-count]").textContent')) === `${readable} !`, 'Inbox count marked for an unreadable repository');
  assert(await evaluate('document.querySelector("[data-t3-inbox-count]").hasAttribute("data-error")'));
  assert.match(await evaluate('document.querySelector("[data-t3-inbox-row]").title'), new RegExp(`${readable} waiting in what could be read; ${deskBase} could not read unreadable: git: not a repository`));
  firstDesk.inboxError = false;
  await evaluate('document.dispatchEvent(new Event("visibilitychange"))');
  await until(async () => (await evaluate('document.querySelector("[data-t3-inbox-count]").textContent')) === String(readable), 'Clean inbox count again');
  const openInbox = async () => {
    await evaluate('document.querySelector("[data-t3-inbox-row]").click()');
    await until(async () => (await layerState())?.shown, 'Inbox layer open');
  };
  await openInbox();
  assert.equal(await layerSrc(), `${deskBase}/embed/inbox`);
  assert.equal(await evaluate('document.querySelector("[data-t3-inbox-row]").getAttribute("aria-pressed")'), 'true');
  // The layer is exactly T3's main area, with nothing of ours but the frame and a close button.
  assert.deepEqual(await evaluate('(()=>{const m=document.querySelector("main[data-slot=sidebar-inset]").getBoundingClientRect();const l=document.querySelector("[data-t3-layer]").getBoundingClientRect();return [l.left===m.left&&l.top===m.top&&l.width===m.width&&l.height===m.height,[...document.querySelector("[data-t3-layer]").children].map(c=>c.tagName).join()]})()'), [true, 'IFRAME,BUTTON']);
  // The frame has focus once its Desk is ready, so the Desk's keys work at once.
  await until(() => evaluate('__t3Custom.diagnostics().layer?.ready && document.activeElement === document.querySelector("[data-t3-layer] iframe")'), 'Layer frame focused');
  const layerSession = async path => {
    const target = await until(async () => (await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json()).find(item => item.type === 'iframe' && item.url.startsWith(`${deskBase}/embed${path}`)), `Layer frame ${path}`);
    return (await page.call('Target.attachToTarget', { targetId: target.id, flatten: true })).sessionId;
  };
  const postFromLayer = async (path, message) => page.call('Runtime.evaluate', { expression: `parent.postMessage(${JSON.stringify(message)}, location.ancestorOrigins[0])` }, await layerSession(path));
  // desk:key Escape from the layer's own frame closes it, and focus leaves the frame.
  await postFromLayer('/inbox', { type: 'desk:key', key: 'Escape' });
  await until(async () => !(await layerState()) && await evaluate('document.querySelector("[data-t3-layer]").hidden'), 'desk:key Escape closes the layer');
  assert.equal(await evaluate('document.activeElement === document.querySelector("[data-t3-layer] iframe")'), false, 'focus stayed in the closed layer');
  for (const [label, close] of [['Escape in T3', 'document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }))'], ['close button', 'document.querySelector("[data-t3-layer-close]").click()'], ['Inbox row again', 'document.querySelector("[data-t3-inbox-row]").click()'], ['a thread row in T3\'s sidebar', 'document.querySelector("[data-sidebar-row=\'env:thread-c\']").click()']]) {
    await openInbox(); await evaluate(close);
    await until(async () => !(await layerState()) && await evaluate('document.querySelector("[data-t3-layer]").hidden'), `Layer closed by ${label}`);
  }
  await openInbox();
  await postFromLayer('/inbox', { type: 'desk:open-thread', thread: 'thread-a' });
  await until(async () => (await evaluate('location.hash')) === '#/env/thread-a' && !(await layerState()), 'desk:open-thread from the layer navigates T3 and closes it');
  // The layer's messages count only from its own frame's window with its Desk's origin: a frame of the
  // same Desk elsewhere, and the layer's window sent to another origin, cannot close it or navigate T3.
  await openInbox();
  await evaluate(`(()=>{const f=document.createElement('iframe');f.id='spoof-layer';f.src='${deskBase}/spoof';document.body.append(f)})()`);
  await until(() => deskRequests.filter(url => url === '/spoof').length === 2, 'Spoofing frame beside the layer');
  await delay(1000);
  assert((await layerState())?.shown, 'desk:key from the Desk origin in another window closed the layer');
  assert.equal(await evaluate('location.hash'), '#/env/thread-a', 'desk:open-thread from the Desk origin in another window was accepted');
  await evaluate('document.querySelector("#spoof-layer").remove()');
  await page.call('Runtime.evaluate', { expression: `location.href = '${secondDeskBase}/spoof'` }, await layerSession('/inbox')).catch(() => {});
  await until(() => secondDeskRequests.filter(url => url === '/spoof').length === 2, 'Layer frame sent to another origin');
  await delay(1000);
  assert((await layerState())?.shown, 'desk:key from the layer window with another origin closed the layer');
  assert.equal(await evaluate('location.hash'), '#/env/thread-a', 'desk:open-thread from the layer window with another origin was accepted');
  await evaluate('document.querySelector("[data-t3-layer-close]").click()');
  console.log('PASS: the Inbox row counts every Desk\'s inbox and opens it in a layer over T3\'s main area that takes focus and closes on desk:key Escape, Escape, its button, the row, a thread row and desk:open-thread; messages from other windows or origins are ignored');

  // Missions in T3's own thread list (the boring sidebar): in its folder, a header per mission
  // with threads there, those threads in T3's order, indented with their role, the repository's
  // other missions in one "N more missions" row, then the folder's other threads. Everything is
  // read as drawn: visible children of the list by their top edge.
  const mission = fixture.mission;
  const missionKey = `${deskBase}/${fixture.repo}/${mission.id}`;
  const missionRow = 'document.querySelector("[data-t3-mission-row]")';
  const drawn = () => evaluate(`[...document.querySelector('#thread-list').children].filter(node => node.getBoundingClientRect().height > 0 && !node.inert).sort((a, b) => a.getBoundingClientRect().top - b.getBoundingClientRect().top).map(node => {
    const dot = node.querySelector('[data-t3-dot]') ? ' •' : '';
    if (node.dataset.threadItem) { const role = getComputedStyle(node.querySelector('[data-sidebar-row]'), '::after').content; return node.dataset.threadItem + (role === 'none' ? '' : ' ' + JSON.parse(role)) + (parseFloat(getComputedStyle(node).marginLeft) > 0 ? ' (indented)' : ''); }
    if (node.dataset.t3MissionRow) return 'mission: ' + node.textContent + dot;
    if (node.dataset.t3MoreRow) return 'more: ' + node.textContent + dot;
    return node.dataset.testid + ': ' + node.textContent;
  })`);
  const tail = ['sidebar-project-folder: unlinked', 'env:thread-c', 'env:other:thread-a', 'sidebar-settled-header: Settled'];
  const laidOut = ['sidebar-pinned-header: Pinned', 'sidebar-project-folder: atelier', 'mission: Fixture mission1/3', 'env:thread-child (indented)', 'env:thread-a build (indented)', 'env:thread-delegated (indented)', 'env:thread-b lead (indented)', 'more: 1 more mission •', 'env:thread-loose', ...tail];
  const untilDrawn = async (expected, label, timeout) => { try { await until(async () => JSON.stringify(await drawn()) === JSON.stringify(expected), label, timeout); } catch (error) { throw new Error(`${error.message}; drawn: ${JSON.stringify(await drawn())}`); } };
  const noOrders = () => evaluate('[...document.querySelector("#thread-list").children].every(node => !node.style.order) && !document.querySelector("[data-t3-list-row]") && !document.querySelector("[data-t3-mission-layout]")?.textContent');
  await untilDrawn(laidOut, 'Missions laid out in T3\'s list');
  assert.equal(await evaluate('__t3Custom.diagnostics().probes["layout:threadList"].ok'), true);
  assert.equal(await evaluate('document.querySelectorAll("[data-t3-mission-row]").length'), 1, 'the second Desk lists no missions');
  console.log('PASS: in T3\'s folder, the mission\'s header, its threads in T3\'s order (indented, with their role; Other threads with none), "1 more mission" with a dot, then the folder\'s other threads; other folders untouched');
  // Folding hides the mission's threads and shows their number and the waiting dot; remembered.
  await evaluate(`${missionRow}.querySelector('[data-t3-fold]').click()`);
  await untilDrawn(['sidebar-pinned-header: Pinned', 'sidebar-project-folder: atelier', 'mission: Fixture mission4 •', 'more: 1 more mission •', 'env:thread-loose', ...tail], 'Folded mission');
  assert.deepEqual(JSON.parse(await evaluate('localStorage.getItem("t3-custom:atelier:missions:v1")')), { folded: { [missionKey]: true } });
  await evaluate(`${missionRow}.querySelector('[data-t3-fold]').click()`);
  await untilDrawn(laidOut, 'Unfolded mission');
  console.log('PASS: the chevron folds a mission to its thread count and waiting dot, remembered in localStorage');
  // Pressing a row's Settle button never reaches T3 while missions are placed, so T3's drag-sweep
  // cannot start; a click on it still does.
  const settle = id => `document.querySelector('[data-thread-item="env:${id}"] [aria-label="Settle thread"]')`;
  await evaluate(`${settle('thread-a')}.dispatchEvent(new PointerEvent('pointerdown', { bubbles: true, isPrimary: true, button: 0 })); ${settle('thread-a')}.click()`);
  assert.deepEqual(await evaluate('[fixtureList.sweeps, fixtureList.settled]'), [[], ['env:thread-a']], 'a press on Settle reached T3, or the click did not');
  // No click with Shift held reaches T3 either, whatever other key is held, since which of them T3
  // turns into a range of its own order depends on the platform; a Ctrl-click does.
  const rowClick = (id, init) => `document.querySelector('[data-sidebar-row="env:${id}"] span').dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true, ...${JSON.stringify(init)} }))`;
  const withShift = [{}, { ctrlKey: true }, { metaKey: true }, { altKey: true }, { ctrlKey: true, metaKey: true, altKey: true }].map(keys => rowClick('thread-a', { shiftKey: true, ...keys }));
  await evaluate(`${rowClick('thread-child', { ctrlKey: true })}; ${withShift.join('; ')}; ${rowClick('thread-a', { ctrlKey: true })}`);
  assert.deepEqual(await evaluate('[fixtureList.ranges, fixtureList.toggles]'), [[], ['env:thread-child', 'env:thread-a']], 'a click with Shift held reached T3 while a mission was placed, or a Ctrl-click did not');
  // T3's fading copies of the extension's own rows are removed as they appear.
  await evaluate('document.querySelector("#thread-list").append(fadingCopy(document.querySelector("[data-t3-mission-row]")))');
  await until(() => evaluate('document.querySelectorAll("[data-t3-list-row]").length === 2'), 'A copy of a mission header removed');
  // T3 adds a row in place, closes and opens a folder, and replaces its list: each is laid out again.
  await evaluate('document.querySelector("#thread-list").insertBefore(threadNode("thread-new"), document.querySelectorAll("#thread-list [data-testid=sidebar-project-folder]")[1])');
  await untilDrawn([...laidOut.slice(0, 9), 'env:thread-new', ...tail], 'A row T3 added, in its place');
  // A closed folder keeps the open thread's row, which reads as a plain row: no mission rows.
  await evaluate('document.querySelector("[data-thread-item=\'env:thread-new\']").remove(); location.hash = "/env/thread-a"');
  await until(() => evaluate('__t3Custom.diagnostics().context?.threadId === "thread-a"'), 'On thread-a');
  await evaluate('document.querySelector("[data-sidebar-row^=\'folder:example\']").click()');
  await untilDrawn(['sidebar-pinned-header: Pinned', 'sidebar-project-folder: atelier', 'env:thread-a', ...tail], 'A closed folder has no mission rows');
  // T3 can open a folder by its button alone, when its rows do not change; laid out at once, not
  // on the next 3-second refresh.
  await evaluate('document.querySelector("[data-sidebar-row^=\'folder:example\']").setAttribute("aria-expanded", "true")');
  await untilDrawn(['sidebar-pinned-header: Pinned', 'sidebar-project-folder: atelier', 'mission: Fixture mission1/3', 'env:thread-a build (indented)', 'more: 1 more mission •', ...tail], 'A folder opened by its button alone', 1500);
  await evaluate('document.querySelector("[data-sidebar-row^=\'folder:example\']").click()');
  await untilDrawn(laidOut, 'The folder open again');
  // A fading copy React's later rows land after, here among the folder's rows, changes nothing.
  await evaluate('(()=>{const row=document.querySelector("[data-thread-item=\'env:thread-a\']");row.before(fadingCopy(row))})()');
  await untilDrawn(laidOut, 'A fading copy among the folder\'s rows');
  await evaluate('replaceList()');
  await untilDrawn(laidOut, 'A new list laid out');
  assert.equal(await evaluate('[...document.querySelectorAll("[data-t3-list-row]")].every(row => row.parentElement === document.querySelector("#thread-list")) && document.querySelectorAll("[data-t3-list-row]").length'), 2);
  console.log('PASS: a press on Settle never reaches T3 while missions are placed, a click does; a row T3 adds, a folder closed and opened, and a new list are laid out again');
  // With the boring sidebar off (no folders) nothing is reordered and a note says how to turn it on.
  await evaluate('fixtureList.boring = false; replaceList()');
  await until(async () => await noOrders() && await evaluate('!document.querySelector("[data-t3-boring-note]").hidden'), 'Boring sidebar off');
  await evaluate(`${rowClick('thread-a', { shiftKey: true })}`);
  assert.deepEqual(await evaluate('fixtureList.ranges'), ['env:thread-a'], 'a Shift-click was stopped with no missions placed');
  assert.match(await evaluate('document.querySelector("[data-t3-boring-note]").textContent'), /Settings → Labs → Boring sidebar/);
  await evaluate(`${settle('thread-a')}.dispatchEvent(new PointerEvent('pointerdown', { bubbles: true, isPrimary: true, button: 0 }))`);
  assert.deepEqual(await evaluate('fixtureList.sweeps'), ['env:thread-a'], 'a press on Settle was stopped with no missions placed');
  await evaluate('fixtureList.boring = true; fixtureList.sweeps = []; replaceList()');
  await untilDrawn(laidOut, 'Boring sidebar on again');
  assert(await evaluate('document.querySelector("[data-t3-boring-note]").hidden'));
  // With the Atelier views turned off T3's list is left as T3 drew it.
  await evaluate('location.hash = "/settings/general"');
  const settingToggle = name => `document.querySelector("#t3-custom-settings").shadowRoot.querySelector('[data-setting="${name}"]')`;
  await until(() => evaluate(`Boolean(${settingToggle('enabled')})`), 'Settings section for the views toggle');
  await evaluate(`${settingToggle('enabled')}.click()`);
  await until(async () => await noOrders() && await evaluate('!document.querySelector("[data-t3-ext-sidebar]")'), 'T3\'s list whole with the views off');
  // A copy T3 fades in after the rows went, from its last positions, goes too.
  await evaluate('globalThis.keptHeader = document.createElement("li"); keptHeader.setAttribute("data-t3-list-row", ""); keptHeader.setAttribute("data-t3-mission-row", "x"); keptHeader.textContent = "Fixture mission"; document.querySelector("#thread-list").append(fadingCopy(keptHeader))');
  await until(() => noOrders(), 'A copy of a mission header removed with the views off');
  await evaluate(`${settingToggle('enabled')}.click()`);
  await untilDrawn(laidOut, 'Missions back with the views on');
  await evaluate('location.hash = "/env/thread-b"');
  await until(() => evaluate('__t3Custom.diagnostics().context?.threadId === "thread-b"'), 'Back on thread-b');
  console.log('PASS: with the boring sidebar off, or the views off, T3\'s list keeps its order with none of the extension\'s rows; the first shows a note naming the setting');
  // A mission's title opens mission control in the layer, selected, with focus; again, it closes it.
  await evaluate(`${missionRow}.querySelector('[data-t3-mission-title]').click()`);
  await until(async () => (await layerState())?.key === `mission:${missionKey}` && (await layerState()).shown, 'Mission control in the layer');
  assert.equal(await layerSrc(), `${deskBase}/embed/mission/${fixture.repo}/${mission.id}`);
  assert.equal(await evaluate(`${missionRow}.getAttribute('aria-pressed')`), 'true');
  assert.equal(await evaluate('document.querySelector("[data-t3-inbox-row]").getAttribute("aria-pressed")'), 'false');
  await until(() => evaluate('__t3Custom.diagnostics().layer?.ready && document.activeElement === document.querySelector("[data-t3-layer] iframe")'), 'Mission control has focus');
  await evaluate(`${missionRow}.querySelector('[data-t3-mission-title]').click()`);
  await until(async () => !(await layerState()), 'The mission title again closes mission control');
  // "N more missions" opens the project home.
  await evaluate('document.querySelector("[data-t3-more-row] button").click()');
  await until(async () => (await layerState())?.key === `home:${deskBase}/r/atelier` && (await layerState()).shown, 'Project home from N more missions');
  assert.equal(await layerSrc(), `${deskBase}/embed/home/atelier`);
  assert.equal(await evaluate('document.querySelector("[data-t3-more-row]").getAttribute("aria-pressed")'), 'true');
  await evaluate('document.querySelector("[data-t3-layer-close]").click()');
  await until(async () => !(await layerState()), 'Project home closed');
  // The project's name in T3's breadcrumb opens the project home instead of starting a thread.
  await evaluate('document.querySelector("#crumb-project").click()');
  await until(async () => (await layerState())?.shown, 'Project home layer');
  assert.equal(await layerSrc(), `${deskBase}/embed/home/atelier`);
  assert.equal(await evaluate('globalThis.crumbStartedThread ?? false'), false, 'T3 started a new thread as well');
  await evaluate('document.querySelector("[data-t3-layer-close]").click()');
  console.log('PASS: a mission\'s title opens mission control in the layer, selected and focused; "N more missions" and the breadcrumb\'s project open the project home and T3 starts no thread');

  // The project strip under the new-thread composer, from /api/launch.
  await evaluate('location.hash = "/draft/draft-a"');
  const strip = 'document.querySelector("[data-t3-strip]")';
  await until(() => evaluate(`Boolean(${strip}) && !${strip}.hidden && Boolean(${strip}.querySelector('[data-t3-card="next"]'))`), 'Project strip');
  assert.deepEqual(await evaluate(`[...${strip}.querySelectorAll('[data-t3-card]')].map(card => [card.dataset.t3Card, card.querySelector('[data-t3-card-head]').textContent, [...card.querySelectorAll('[data-t3-card-row]')].map(row => row.textContent)])`), [
    ['waiting', 'Waiting on you5', ['Waiting one', 'Waiting two', '3 more']],
    ['motion', 'In motion2', ['atelier-x1Held by thread Bstarted · 2hThread B', 'atelier-x2Held elsewherereview · 1dRemote holder']],
    ['next', 'Next up4', ['atelier-r1Ready one, with a title long enough to be cut short in its card', 'atelier-r2Ready two', 'atelier-r3; rm -rf /Not an item ID']],
  ]);
  assert.equal(await evaluate(`${strip}.querySelector('[data-t3-strip-project]').textContent`), 'atelier');
  // The strip's box is placed in whole pixels.
  const stripBox = await evaluate(`(()=>{const s=${strip}.getBoundingClientRect();const c=document.querySelector('[data-chat-composer-stack]').getBoundingClientRect();return {strip:[s.left,s.top,s.width,s.height],composer:[c.left,c.bottom,c.width]}})()`);
  const [left, top, width, height] = stripBox.strip; const [composerLeft, composerBottom, composerWidth] = stripBox.composer;
  assert(Math.abs(width - composerWidth) <= 1 && Math.abs(left - composerLeft) <= 1 && top >= composerBottom - 1 && height === 176, `the strip is not as wide as the composer, under it, 176px high: ${JSON.stringify(stripBox)}`);
  // A Next up row fills the composer and nothing is sent; an ID not of Atelier's form makes no prompt.
  const nextRow = n => `[...${strip}.querySelectorAll('[data-t3-card="next"] [data-t3-card-row]')][${n}]`;
  // A plain contenteditable keeps a trailing space as a no-break space; T3's ProseMirror does not.
  const composerText = async () => (await evaluate('document.querySelector("[data-testid=composer-editor]").textContent')).replace(/ /g, ' ');
  await evaluate(`${nextRow(0)}.click()`);
  assert.equal(await composerText(), 'Start atelier-r1: run `atelier start atelier-r1` in a fresh checkout and follow what it prints.');
  assert.equal(await evaluate('fixtureDraftState.draftsByThreadKey["draft-a"]?.prompt'), 'Start atelier-r1: run `atelier start atelier-r1` in a fresh checkout and follow what it prints.', 'T3\'s draft store did not get the prompt');
  // While the editor on the page is still another draft's, a click fills nothing: that draft's
  // prompt is put back, draft-a's is untouched, and the reason is reported.
  await evaluate('fixtureDraftState.draftsByThreadKey["draft-c"] = { prompt: "Draft C text" }; globalThis.fixtureEditorDraft = "draft-c"; __t3Custom.error = null');
  await evaluate(`${nextRow(1)}.click()`);
  await until(() => evaluate('Boolean(__t3Custom.error)'), 'The refusal reported');
  assert.match(await evaluate('__t3Custom.error'), /the composer on the page was draft draft-c's, not draft-a's/);
  assert.equal(await evaluate('fixtureDraftState.draftsByThreadKey["draft-c"].prompt'), 'Draft C text', 'another draft\'s prompt was overwritten');
  assert.equal(await evaluate('fixtureDraftState.draftsByThreadKey["draft-a"]?.prompt'), 'Start atelier-r1: run `atelier start atelier-r1` in a fresh checkout and follow what it prints.');
  await evaluate('globalThis.fixtureEditorDraft = undefined; __t3Custom.error = null; document.querySelector("[data-testid=composer-editor]").textContent = "Start atelier-r1: run `atelier start atelier-r1` in a fresh checkout and follow what it prints."');
  assert.equal(await evaluate(`${nextRow(2)}.disabled`), true, 'a prompt can be made from an ID that is not an item ID');
  await evaluate(`${strip}.querySelector('[data-t3-strip-plan]').click()`);
  assert.equal(await composerText(), 'Let\'s plan a mission for ');
  assert.equal(await evaluate('globalThis.fixtureSent ?? null'), null, 'the composer was sent');
  // In motion opens the item in the layer when T3 does not know its holder; Waiting opens the inbox;
  // the project's name opens its home.
  for (const [label, click, src] of [['an item held elsewhere', `[...${strip}.querySelectorAll('[data-t3-card="motion"] [data-t3-card-row]')][1].click()`, `${deskBase}/embed/r/atelier/item/atelier-x2`], ['Waiting on you', `${strip}.querySelector('[data-t3-card="waiting"] [data-t3-card-head]').click()`, `${deskBase}/embed/inbox`], ['the project name', `${strip}.querySelector('[data-t3-strip-project]').click()`, `${deskBase}/embed/home/atelier`]]) {
    await evaluate(click);
    await until(async () => (await layerState())?.shown && (await layerSrc()) === src, `Strip: ${label} opens the layer`);
    await evaluate('document.querySelector("[data-t3-layer-close]").click()');
  }
  // Under 700px of composer width the In motion card goes.
  await evaluate('document.querySelector("[data-chat-canvas]").style.width = "640px"');
  await until(() => evaluate(`${strip}.hasAttribute('data-narrow') && ${strip}.querySelector('[data-t3-card="motion"]').getBoundingClientRect().width === 0 && ${strip}.getBoundingClientRect().width === 600`), 'Narrow strip');
  await evaluate('document.querySelector("[data-chat-canvas]").style.width = ""');
  await until(() => evaluate(`!${strip}.hasAttribute('data-narrow')`), 'Wide strip again');
  // In motion opens its holder's thread when T3 knows it.
  await evaluate(`[...${strip}.querySelectorAll('[data-t3-card="motion"] [data-t3-card-row]')][0].click()`);
  await until(async () => (await evaluate('location.hash')) === '#/env/thread-b', 'Strip: a holder T3 knows opens its thread');
  // Nothing for a project with no Desk repository.
  await evaluate('location.hash = "/draft/draft-c"');
  await until(() => evaluate(`${strip}.hidden`), 'No strip for an unmapped project');
  await delay(1000);
  assert(await evaluate(`${strip}.hidden`), 'a strip appeared for an unmapped project');
  assert(!deskRequests.some(url => url.startsWith('/api/launch') && url !== '/api/launch?repo=atelier'), 'launch data was read for an unmapped project');
  console.log('PASS: the project strip shows three cards from /api/launch as wide as the composer; Next up fills the composer without sending, and only from item IDs; In motion, Waiting and the project open the layer or the thread; under 700px In motion goes; nothing for an unmapped project');
  await evaluate('location.hash = "/env/thread-c"');
  await until(() => evaluate('__t3Custom.diagnostics().context?.threadId === "thread-c"'), 'Back on thread-c');


  const deskLoads = deskRequests.filter(url => url.startsWith('/embed')).length;
  const frameBeforeSave = await evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").src');
  await evaluate(`globalThis.beforeSave=document.querySelector("#t3-custom-root");document.querySelector('[data-t3-desk-controls] [aria-label="Atelier customizations"]').click()`);
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings")?.shadowRoot.textContent.includes("reachable, "))'), 'Settings section with Desk status');
  const settingsText = await evaluate('document.querySelector("#t3-custom-settings").shadowRoot.textContent');
  assert(settingsText.includes(`${deskBase}: reachable`) && settingsText.includes(`atelier (${fixtureRepositoryKey})`), settingsText);
  assert(settingsText.includes('module:environment-atoms') && settingsText.includes('css:theme'), settingsText);
  // The release: the injector's lookup at start failed, and the section names it.
  const settingsQuery = selector => `document.querySelector("#t3-custom-settings").shadowRoot.querySelector(${JSON.stringify(selector)})`;
  const refreshUntil = (expression, label) => until(() => evaluate(`(${expression})||(${settingsQuery('#refresh-status')}.click(),false)`), label);
  await refreshUntil(`${settingsQuery('#release-error')}.textContent === "Release check failed: Looking up releases: GET ${base}/releases: HTTP 503 Service Unavailable"`, 'The failed release lookup in Customizations');
  assert.match(await evaluate(`${settingsQuery('#release-status')}.textContent`), /^Extension development · checked /);
  assert.equal(await evaluate(`${settingsQuery('#update-release')}.hidden`), true, 'an update was offered after a failed lookup');
  // A newer release, as a later check records it, is offered with a button.
  await fs.writeFile(path.join(temp, 'update.json'), JSON.stringify({ checkedAt: new Date().toISOString(), installed: 'development', latest: { version: '0.5.0', tag: 't3-extension-v0.5.0' }, newer: true, error: null }));
  await refreshUntil(`${settingsQuery('#release-status')}.textContent.includes("· release 0.5.0 is available ·")`, 'The newer release in Customizations');
  assert.equal(await evaluate(`${settingsQuery('#release-error')}.hidden`), true);
  assert.equal(await evaluate(`${settingsQuery('#update-release')}.hidden ? null : ${settingsQuery('#update-release')}.textContent`), 'Update to 0.5.0');
  assert.equal(await evaluate(`${settingsQuery('[data-setting="autoUpdate"]')}.checked`), false, 'automatic updates are on by default');
  await evaluate(`(()=>{const select=document.querySelector("#t3-custom-settings").shadowRoot.querySelector("#project-mapping");select.value="${deskBase}/r/atelier";select.dispatchEvent(new Event("change"))})()`);
  // The views toggle was saved earlier; wait for this mapping.
  const saved = await until(async () => { const settings = JSON.parse(await fs.readFile(path.join(temp, 'config.json'), 'utf8')).settings; return settings?.mappings?.['env:project-c'] && settings; }, 'Settings written to config.json');
  assert.deepEqual(saved.mappings, { 'env:project-c': `${deskBase}/r/atelier` });
  assert.equal(saved.references, true);
  // A save from the settings page applies in place: no reinjection, no Desk reload.
  await delay(1500);
  assert(await evaluate('document.querySelector("#t3-custom-root") === globalThis.beforeSave'), 'a settings save reinjected the extension');
  assert.equal(deskRequests.filter(url => url.startsWith('/embed')).length, deskLoads, 'a settings save reloaded the Desk iframe');
  assert.equal(await evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").src'), frameBeforeSave);
  // Many saves at once: the last one sent is what config.json holds, and it is still applied in place.
  await evaluate(`Promise.all(Array.from({ length: 20 }, (_, n) => fixtureApi.request('save-settings', { ...${JSON.stringify(saved)}, pinnedRepo: 'concurrent-' + n })))`);
  assert.equal(JSON.parse(await fs.readFile(path.join(temp, 'config.json'), 'utf8')).settings.pinnedRepo, 'concurrent-19', 'an older concurrent save won');
  await delay(1500);
  assert(await evaluate('document.querySelector("#t3-custom-root") === globalThis.beforeSave'), 'concurrent saves reinjected the extension');
  // An edit of config.json from outside still reloads the extension.
  const edited = JSON.parse(await fs.readFile(path.join(temp, 'config.json'), 'utf8'));
  edited.settings.pinnedRepo = 'atelier';
  await fs.writeFile(path.join(temp, 'config.json'), JSON.stringify(edited, null, 2));
  await until(() => evaluate('document.querySelector("#t3-custom-root") !== globalThis.beforeSave'), 'Reapplied after an outside edit of config.json');
  const stored = await evaluate('JSON.stringify(Object.fromEntries(Object.keys(localStorage).map(key => [key, localStorage[key]])))');
  assert(!stored.includes('mappings') && stored.includes('openThreads'), `localStorage holds only view state: ${stored}`);
  await evaluate('location.hash="/env/thread-c"');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]")?.shadowRoot.querySelector("iframe").hidden'), 'Explicit project mapping');
  await evaluate('document.querySelector("#native-tab").click()');
  await until(() => evaluate('document.querySelector("[data-t3-desk-mount]").hidden && !document.querySelector("#native-content").inert'), 'Native tab restoration');
  console.log('PASS: Customizations names a failed release lookup and offers a newer release; automatic updates are off by default');
  console.log('PASS: a manual project mapping is written to config.json and applied in place without reloading the Desk; an outside edit reloads; switching back to native surfaces');

  const timeOrigin = await evaluate('performance.timeOrigin');
  await evaluate('location.hash="/settings/general"');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings"))'), 'Settings remount');
  // A folded mission, as a fold saves it, stays folded after reinjection.
  await evaluate(`localStorage.setItem("t3-custom:atelier:missions:v1", ${JSON.stringify(JSON.stringify({ folded: { [`${deskBase}/${fixture.repo}/${fixture.mission.id}`]: true } }))})`);
  await evaluate('globalThis.previousRoot=document.querySelector("#t3-custom-root");document.querySelector("#t3-custom-settings").shadowRoot.querySelector("#reload-scripts").click()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root")) && document.querySelector("#t3-custom-root") !== globalThis.previousRoot'), 'Settings reinjection control');
  assert.equal(await evaluate('performance.timeOrigin'), timeOrigin);
  assert.equal(await evaluate('fixtureListeners.size'), 1);
  await until(() => evaluate('document.querySelector("[data-t3-mission-row] [data-t3-fold]")?.getAttribute("aria-expanded") === "false"'), 'Folded mission remembered across reinjection');
  // By now the worker is in the page's resource timings, as after a TIFF/HEIF preview in T3.
  assert.equal(await evaluate('globalThis.workerScriptRanInWindow ?? false'), false, 'reinjection imported a worker script into the window');
  assert.equal(await evaluate('document.querySelector("#t3-custom-settings").shadowRoot.querySelector("#project-mapping").disabled'), false);
  await evaluate('location.hash="/env/thread-c"');
  await delay(500); await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  const finalFrame = await until(async () => (await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json()).find(target => target.type === 'iframe' && target.url.startsWith(deskBase)), 'Reinjected Desk iframe');
  const finalChild = await page.call('Target.attachToTarget', { targetId: finalFrame.id, flatten: true });
  await until(async () => (await page.call('Runtime.evaluate', { expression: 'document.readyState === "complete" && document.body.innerText.includes("atelier") && !document.querySelector(".skel")', returnByValue: true }, finalChild.sessionId)).result.value, 'Reinjected Desk rendering');
  await fs.mkdir(path.join(root, 'artifacts'), { recursive: true });
  const screenshot = await page.call('Page.captureScreenshot', { format: 'png' });
  await fs.writeFile(path.join(root, 'artifacts/extension-docked.png'), Buffer.from(screenshot.data, 'base64'));
  console.log('PASS: Settings reinjection preserves the document and cleans up old subscriptions');

  // Without T3's sidebar search field the sidebar views go, by name; the panel and references stay.
  await evaluate('document.querySelector("#search-wrap input").type = "text"');
  await until(() => evaluate('__t3Custom.diagnostics().probes["dom:sidebarSearch"].ok === false && !document.querySelector("[data-t3-ext-sidebar]")'), 'Sidebar views off without T3\'s search field', 10000);
  assert.deepEqual(await evaluate('__t3Custom.diagnostics().views.sidebar'), ['dom:sidebarSearch']);
  // Missions in T3's list need no search field.
  assert.equal(await evaluate('document.querySelectorAll("[data-t3-mission-row]").length'), 1, 'missions left T3\'s list with the search field');
  assert(await evaluate('!document.querySelector("[data-t3-desk-mount]").hidden && __t3Custom.diagnostics().deskAvailable'), 'the Desk panel went with the sidebar hook');
  await evaluate('(()=>{const link=document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]");link.blur();link.focus()})()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard h3"))'), 'References without the sidebar hook');
  console.log('PASS: a missing sidebar hook turns off the sidebar views by name and leaves the Desk panel and references working');
  // A thread list not shaped as missions need turns that view off by name and leaves T3's list
  // exactly as T3 drew it.
  await evaluate('document.querySelector("#thread-list").style.display = "block"; document.querySelector("#thread-list").append(document.createElement("li"))');
  await until(() => evaluate('__t3Custom.diagnostics().probes["layout:threadList"].ok === false'), 'The list structure probe');
  assert.match(await evaluate('__t3Custom.diagnostics().probes["layout:threadList"].detail'), /not a flex column/);
  assert.deepEqual(await evaluate('__t3Custom.diagnostics().views.missions'), ['layout:threadList']);
  await evaluate('document.querySelector("#thread-list").append(fadingCopy(keptHeader))');
  await until(() => evaluate('[...document.querySelector("#thread-list").children].every(node => !node.style.order) && !document.querySelector("[data-t3-list-row]") && !document.querySelector("[data-t3-mission-layout]").textContent'), 'T3\'s list whole with the missions view off');
  console.log('PASS: a thread list missions cannot use turns the missions view off by name, with no order, row or rule of the extension left in it');

  // If T3 renames its panel slot, reference clicks open Desk externally and the probe names the hook.
  await evaluate('window.open = url => { globalThis.openedExternally = url; }; const slot = document.querySelector("[data-right-panel-surface-content]"); slot.removeAttribute("data-right-panel-surface-content"); slot.setAttribute("data-renamed-surface-content", "")');
  const openReference = async () => {
    await evaluate('(()=>{const link=document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]");link.blur();link.focus()})()');
    await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button"))'), 'Hovercard with renamed panel slot');
    const clicked = Date.now();
    await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button").click()');
    const url = await until(() => evaluate('globalThis.openedExternally'), 'External Desk fallback', 10000);
    return { url, elapsed: Date.now() - clicked };
  };
  assert.equal((await openReference()).url, `${deskBase}/r/atelier/item/${referenceId}`);
  const slotProbe = await evaluate('__t3Custom.diagnostics().probes["dom:panelContent"]');
  assert.equal(slotProbe.ok, false); assert.match(slotProbe.detail, /not found 3 seconds after opening/);
  await evaluate('globalThis.openedExternally = null');
  const second = await openReference();
  assert.equal(second.url, `${deskBase}/r/atelier/item/${referenceId}`);
  assert(second.elapsed < 2500, `once the slot is known missing, references open externally without the 3 second wait (took ${second.elapsed} ms)`);
  assert.equal(await evaluate('__t3Custom.diagnostics().deskAvailable'), false);
  // DOM sync runs on setTimeout, which headless browsers may delay by up to a second.
  await until(() => evaluate('[...document.querySelectorAll("[data-t3-desk-launch], [data-t3-desk-tab]")].every(button => button.hidden)'), 'Desk buttons hidden without a panel slot');
  await evaluate('location.hash="/settings/general"');
  await until(() => evaluate('document.querySelector("#t3-custom-settings")?.shadowRoot.textContent.includes("Desk panel: off, missing dom:panelContent")'), 'Settings names the missing panel hook');
  const probeList = await evaluate('document.querySelector("#t3-custom-settings").shadowRoot.textContent');
  for (const line of ['Inbox row in the sidebar: off, missing dom:sidebarSearch', 'Missions in T3\'s thread list: off (T3\'s list is left as T3 drew it), missing layout:threadList', 'Desk layer over T3\'s main area: on', 'Project strip on the new-thread screen: on', 'Project home from the breadcrumb: on', 'dom:composerStack', 'dom:threadItem', 'state:thread-origin', 'module:composer-draft-store']) assert(probeList.includes(line), `the settings section does not say "${line}"`);
  console.log('PASS: a renamed panel slot turns the Desk panel off by name and reference clicks open Desk externally');
  await control('stop');
  await until(() => evaluate('!document.querySelector("#t3-custom-root") && !document.querySelector("[data-t3-desk-mount]") && !document.querySelector("[data-t3-desk-launch]") && !document.querySelector("[data-t3-ext-sidebar], [data-t3-layer], [data-t3-strip], [data-t3-list-row], [data-t3-mission-layout]")'), 'Full extension removal');
  assert.equal(await evaluate('document.querySelector("#native-content").inert'), false);
  console.log('PASS: stop removes custom panel, sidebar views, layer, strip, settings controls, links, and native-content overrides');
  console.log('Test logs:', temp);
} catch (error) {
  console.error(error.stack); console.error('Test logs:', temp);
  console.error('Last paths asked of the first Desk:', deskRequests.slice(-15).join(' '));
  try { console.error(await fs.readFile(path.join(temp, 'injector.log'), 'utf8')); } catch {}
  try { console.error('Browser log, last 40 lines:\n' + (await fs.readFile(path.join(temp, 'browser.log'), 'utf8')).split('\n').slice(-40).join('\n')); } catch {}
  if (fixtureDesk) console.error('Fixture Desk log:\n' + fixtureDesk.log());
  process.exitCode = 1;
} finally {
  try { await control('stop'); } catch {}
  page?.close();
  for (const child of [injector, browser]) if (child && child.exitCode === null) {
    // Windows reports EPERM when the process already exited on its own.
    try { child.kill('SIGTERM'); await Promise.race([new Promise(resolve => child.once('close', resolve)), delay(4000)]); if (child.exitCode === null) child.kill('SIGKILL'); } catch {}
  }
  await new Promise(resolve => server.close(resolve));
  for (const proxy of [deskProxy, secondDeskProxy]) { proxy.closeAllConnections(); await new Promise(resolve => proxy.close(resolve)); }
  await fixtureDesk?.stop();
}
