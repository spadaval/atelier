import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import http from 'node:http';
import { spawn } from 'node:child_process';
import { setTimeout as delay } from 'node:timers/promises';
import { connectPage } from './devtools.mjs';

const root = process.cwd();
const temp = await fs.mkdtemp(path.join(os.tmpdir(), 'atelier-extension-test-'));
const debugPort = 19222;
let deskBase, realDesk;
// The Desk's identity for the atelier repository, as GET /api/repos reports it.
const fixtureRepositoryKey = 'github.com/spadaval/atelier';
const referenceId = 'atelier-plr9';
const missionId = 'atelier-g961';
// A Chromium browser for the isolated fixture page; set BROWSER to override.
const browserPath = process.env.BROWSER ?? (process.platform === 'win32' ? 'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe' : '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome');
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
  threads: [{ id: 'thread-a', projectId: 'project-a' }, { id: 'thread-b', projectId: 'project-a' }, { id: 'thread-c', projectId: 'project-c' }],
};
// Modules shaped like T3's private hooks, so the real discovery code runs. Their names
// differ from T3's chunk names, so discovery searches every declared module.
const chunks = {
  'panel-fixture.js': `const listeners = globalThis.fixtureListeners = new Set();
const byThreadKey = Object.fromEntries(['thread-a', 'thread-b', 'thread-c'].map(id => ['env:' + id, { isOpen: true, activeSurfaceId: 'files' }]));
const state = { byThreadKey, openBrowser() {}, show({ environmentId, threadId }) { byThreadKey[environmentId + ':' + threadId].isOpen = true; for (const listener of listeners) listener(); } };
export const s = { getState: () => state, subscribe: listener => { listeners.add(listener); return () => listeners.delete(listener); } };`,
  'registry-fixture.js': `export const r = { '~effect/reactivity/AtomRegistry': '~effect/reactivity/AtomRegistry', get: atom => atom };`,
  'shell-fixture.js': `let reads = 0; const snapshot = ${JSON.stringify(shell)};
export const c = { familyAtom() {}, mountRows() {}, snapshotAtom: environmentId => environmentId === 'env-empty' ? { projects: [], threads: [] } : reads++ ? snapshot : { ...snapshot, threads: [] } };`,
  'environment-fixture.js': `export const e = { environmentIdsAtom: ['env', 'env-empty'], connectedEnvironmentIdsAtom: ['env', 'env-empty'] };`,
  'index-fixture.js': `import './panel-fixture.js'; import './registry-fixture.js'; import './shell-fixture.js'; import './environment-fixture.js';
new Worker('/assets/imageDecode.worker-fixture.js');`,
  // Like T3's image decoder worker: its top-level code must never run in the window.
  'imageDecode.worker-fixture.js': `if (typeof window !== 'undefined') globalThis.workerScriptRanInWindow = true;`,
};
const html = `<!doctype html><html class="dark"><head><meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; connect-src 'self' blob: http: https: ws: wss:; style-src 'self' 'unsafe-inline'; frame-src 'self' blob: http: https:; worker-src 'self' blob:"><title>Atelier extension fixture</title><script type="module" src="/assets/index-fixture.js"></script>${Object.keys(chunks).filter(name => !name.includes('worker')).map(name => `<link rel="modulepreload" href="/assets/${name}">`).join('')}<style>html{--background:#202124;--foreground:#eeeeee;--card:#2a2b2e;--border:#555555;--muted-foreground:#a0a0a8;--secondary:#353639;--primary:#9bbcff}body{margin:0;background:var(--background);color:var(--foreground);font:14px Georgia, serif}#app{height:100vh;display:flex;flex-direction:column}header{padding:12px;border-bottom:1px solid #555}#workspace{display:flex;flex:1;min-height:0}[data-workspace-area=conversation]{flex:1;padding:20px;min-width:0;overflow:auto}[data-workspace-area=panel]{width:750px;display:flex;flex-direction:column;min-width:0}[data-right-panel-tabbar]{height:52px;display:flex;align-items:center}[data-right-panel-tabbar]>div{display:flex;align-items:center}[data-right-panel-surface-content]{position:relative;flex:1;min-height:0}#native-content{padding:20px}#settings{overflow:auto;padding:24px;max-width:850px}#composer{margin-top:30px;border:1px solid #555;padding:12px}</style></head><body><div id="app"><header><span data-panel-layout-controls>Workspace controls</span></header><div id="workspace"><main data-workspace-area="conversation"><h1>Conversation</h1><p id="reference-prose">Investigate ${referenceId}, then <code>${referenceId}</code>. Ignore other-xabc and not-${referenceId}-extra.</p><pre>${referenceId}</pre><div id="composer" contenteditable="true">${referenceId}</div><p id="stream"></p></main><div data-workspace-area="panel"><div data-right-panel-tabbar><div data-right-panel-tab-list><div data-active-tab="true"><button id="native-tab">Files</button></div><button aria-label="Add panel surface">+</button></div></div><div data-right-panel-surface-content><div id="native-content"><button id="native-control">Native file viewer</button></div></div></div></div><div id="settings" hidden><div data-settings-page-scroll><div><h1>General</h1><p>T3 settings fixture</p></div></div></div></div><script>
addEventListener('hashchange',()=>{const settings=location.hash.includes('/settings');document.querySelector('#settings').hidden=!settings;document.querySelector('#workspace').style.display=settings?'none':'flex'});
</script></body></html>`;
const server = http.createServer((request, response) => {
  const url = new URL(request.url, 'http://fixture');
  const chunk = chunks[url.pathname.replace('/assets/', '')];
  if (chunk) { response.writeHead(200, { 'content-type': 'text/javascript' }); response.end(chunk); return; }
  // A same-site frame in the page's process: it has the injector's binding and calls it directly.
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
    if (options.missing?.includes(request.url)) { response.writeHead(404, { 'content-type': 'application/json' }); response.end(JSON.stringify({ error: 'not here' })); return; }
    if (request.url === '/spoof') {
      response.writeHead(200, { 'content-type': 'text/html' });
      response.end(`<script>parent.postMessage({ type: 'desk:navigated', path: '/spoofed', title: 'Spoofed' }, '*'); parent.postMessage({ type: 'desk:open-thread', thread: 'thread-c' }, '*');</script>`);
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
const firstDesk = { slowEmbed: 0, missing: [`/api/records/${missionId}`] };
const deskProxy = deskProxyServer(deskRequests, firstDesk);
const secondDeskProxy = deskProxyServer(secondDeskRequests, {});
await new Promise(resolve => secondDeskProxy.listen(0, '127.0.0.1', resolve));
await new Promise(resolve => deskProxy.listen(0, '127.0.0.1', resolve));
try {
  const configPath = path.join(root, 'config.json');
  const config = JSON.parse(await fs.readFile(await fs.access(configPath).then(() => configPath, () => path.join(root, 'config.example.json')), 'utf8'));
  realDesk = (config.desk.urls?.[0] ?? config.desk.baseUrl).replace(/\/$/, '');
  // Another site than the fixture, as the real Desk is, so its frame is a separate target.
  deskBase = `http://localhost:${deskProxy.address().port}`;
  const secondDeskBase = `http://localhost:${secondDeskProxy.address().port}`;
  // No repository table: the project is matched by the identity the Desk reports.
  config.desk = { urls: [deskBase, secondDeskBase] }; delete config.settings;
  config.debugPort = debugPort; config.targetPrefixes = [base];
  await fs.writeFile(path.join(temp, 'config.json'), JSON.stringify(config));
  await fs.copyFile(path.join(root, 'custom.js'), path.join(temp, 'custom.js'));
  await fs.copyFile(path.join(root, 'custom.css'), path.join(temp, 'custom.css'));
  // The real adapter, including module discovery; the wrapper only exposes the bridge to the test.
  await fs.writeFile(path.join(temp, 'host-adapter.js'), await fs.readFile(path.join(root, 'host-adapter.js'), 'utf8') + `
const discoverHost = createHostAdapter;
createHostAdapter = api => { globalThis.fixtureApi = api; return discoverHost(api); };`);
  const browserLog = await fs.open(path.join(temp, 'browser.log'), 'w');
  const env = { ...process.env }; delete env.ELECTRON_RUN_AS_NODE; delete env.NODE_OPTIONS;
  browser = spawn(browserPath, ['--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking', `--user-data-dir=${path.join(temp, 'profile')}`, `--remote-debugging-port=${debugPort}`, '--remote-debugging-address=127.0.0.1', '--window-size=1600,1000', base + '/#/env/thread-a'], { env, stdio: ['ignore', browserLog.fd, browserLog.fd] });
  await browserLog.close();
  page = await until(() => connectPage(debugPort, base), 'Browser startup');
  const injectorLog = await fs.open(path.join(temp, 'injector.log'), 'w');
  injector = spawn(process.execPath, [path.join(root, 't3-custom.mjs'), 'daemon', '--home', temp], { stdio: ['ignore', injectorLog.fd, injectorLog.fd] });
  await injectorLog.close();
  const evaluate = expression => page.evaluate(expression);
  await until(() => evaluate('Boolean(document.querySelector("[data-t3-desk-launch]:not([hidden])"))'), 'Extension initialization and thread context');
  const discovery = await evaluate(`({ workerLoaded: performance.getEntriesByType('resource').some(entry => entry.name.includes('.worker-')), ranInWindow: globalThis.workerScriptRanInWindow ?? false, modules: __t3Custom.diagnostics().modules, probes: Object.entries(__t3Custom.diagnostics().probes).filter(([name]) => name.startsWith('module:')).map(([name, probe]) => name + '=' + probe.ok) })`);
  assert(discovery.workerLoaded, 'the fixture worker should be among the page resources');
  assert.equal(discovery.ranInWindow, false, `discovery imported a worker script into the window: ${discovery.modules.join(', ')}`);
  assert.deepEqual(discovery.probes, ['module:workspace-panel-store=true', 'module:atom-registry=true', 'module:thread-snapshot-atom=true', 'module:environment-atoms=true']);
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
  // host:thread on desk:ready: the real Desk looks the thread up and opens its page, here the
  // inbox, since the fixture's thread holds nothing, with the line naming the thread's part.
  await until(() => deskRequests.includes('/api/ties?thread=thread-a'), 'host:thread on desk:ready');
  await until(async () => (await inFrame('location.pathname')) === '/embed/inbox', 'Desk follows the thread');
  assert.equal(await until(() => inFrame('document.querySelector(".thread-part")?.textContent'), 'Thread line'), 'This thread holds no open work.');
  // Title: the tab shows the Desk's page title from desk:navigated; Open externally is not embedded.
  const deskTitle = await until(async () => { const title = await inFrame('document.title'); return title && await evaluate('document.querySelector("[data-t3-desk-label]").textContent') === title && title; }, 'Tab label from desk:navigated');
  assert.notEqual(deskTitle, 'Desk');
  assert.equal(await evaluate(`${shadow}.querySelector("header")`), null, 'no doubled header in the panel');
  await until(async () => (await evaluate('document.querySelector("[data-t3-desk-external]").href')) === deskBase + '/inbox', 'Open externally follows desk:navigated');
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
  await inFrame(`window.hostThreads = []; addEventListener('message', event => { if (event.data?.type === 'host:thread') hostThreads.push(event.data.thread); })`);
  await post({ type: 'desk:open-thread', thread: 'thread-unknown' });
  await until(() => evaluate(`${shadow}.querySelector(".notice:not([hidden])")?.textContent.includes("thread-unknown is not in any connected T3 environment")`), 'Notice for an unknown thread');
  assert.equal(await evaluate('location.hash'), hashBefore);
  await post({ type: 'desk:open-thread', thread: 'thread-b' });
  await until(async () => (await evaluate('location.hash')) === '#/env/thread-b', 'desk:open-thread navigation');
  await evaluate('location.hash = "/env/thread-a"');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").hidden'), 'Desk back on thread-a');
  console.log('PASS: desk:open-thread opens the fixture route #/env/thread-b; an unknown thread is reported in the panel');
  // host:thread on each switch to another thread, and never twice in a row for the same one.
  await until(async () => JSON.stringify(await inFrame('hostThreads')) === '["thread-b","thread-a"]', 'host:thread on thread switches');
  assert(deskRequests.includes('/api/ties?thread=thread-b'));
  await evaluate('location.hash = "/settings/general"');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings"))'), 'Settings between threads');
  await evaluate('location.hash = "/env/thread-a"');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").hidden && __t3Custom.diagnostics().context?.threadId === "thread-a"'), 'Back on thread-a from Settings');
  await delay(1000);
  assert.deepEqual(await inFrame('hostThreads'), ['thread-b', 'thread-a'], 'the same thread was sent again');
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

  const deskLoads = deskRequests.filter(url => url.startsWith('/embed')).length;
  const frameBeforeSave = await evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").src');
  await evaluate(`globalThis.beforeSave=document.querySelector("#t3-custom-root");document.querySelector('[data-t3-desk-controls] [aria-label="Atelier customizations"]').click()`);
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings")?.shadowRoot.textContent.includes("reachable, "))'), 'Settings section with Desk status');
  const settingsText = await evaluate('document.querySelector("#t3-custom-settings").shadowRoot.textContent');
  assert(settingsText.includes(`${deskBase}: reachable`) && settingsText.includes(`atelier (${fixtureRepositoryKey})`), settingsText);
  assert(settingsText.includes('module:environment-atoms') && settingsText.includes('css:theme'), settingsText);
  await evaluate(`(()=>{const select=document.querySelector("#t3-custom-settings").shadowRoot.querySelector("#project-mapping");select.value="${deskBase}/r/atelier";select.dispatchEvent(new Event("change"))})()`);
  const saved = await until(async () => JSON.parse(await fs.readFile(path.join(temp, 'config.json'), 'utf8')).settings, 'Settings written to config.json');
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
  console.log('PASS: a manual project mapping is written to config.json and applied in place without reloading the Desk; an outside edit reloads; switching back to native surfaces');

  const timeOrigin = await evaluate('performance.timeOrigin');
  await evaluate('location.hash="/settings/general"');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings"))'), 'Settings remount');
  await evaluate('globalThis.previousRoot=document.querySelector("#t3-custom-root");document.querySelector("#t3-custom-settings").shadowRoot.querySelector("#reload-scripts").click()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root")) && document.querySelector("#t3-custom-root") !== globalThis.previousRoot'), 'Settings reinjection control');
  assert.equal(await evaluate('performance.timeOrigin'), timeOrigin);
  assert.equal(await evaluate('fixtureListeners.size'), 1);
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
  console.log('PASS: a renamed panel slot turns the Desk panel off by name and reference clicks open Desk externally');
  await control('stop');
  await until(() => evaluate('!document.querySelector("#t3-custom-root") && !document.querySelector("[data-t3-desk-mount]") && !document.querySelector("[data-t3-desk-launch]")'), 'Full extension removal');
  assert.equal(await evaluate('document.querySelector("#native-content").inert'), false);
  console.log('PASS: stop removes custom panel, settings controls, links, and native-content overrides');
  console.log('Test logs:', temp);
} catch (error) {
  console.error(error.stack); console.error('Test logs:', temp);
  try { console.error(await fs.readFile(path.join(temp, 'injector.log'), 'utf8')); } catch {}
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
}
