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
let deskBase;
const fixtureRepositoryKey = 'example.com/owner/atelier';
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
// Modules shaped like T3's private hooks, so the real discovery code runs. Their names
// differ from T3's chunk names, so discovery searches every declared module.
const chunks = {
  'panel-fixture.js': `const listeners = globalThis.fixtureListeners = new Set();
const byThreadKey = Object.fromEntries(['thread-a', 'thread-b', 'thread-c'].map(id => ['env:' + id, { isOpen: true, activeSurfaceId: 'files' }]));
const state = { byThreadKey, openBrowser() {}, show({ environmentId, threadId }) { byThreadKey[environmentId + ':' + threadId].isOpen = true; for (const listener of listeners) listener(); } };
export const s = { getState: () => state, subscribe: listener => { listeners.add(listener); return () => listeners.delete(listener); } };`,
  'registry-fixture.js': `export const r = { '~effect/reactivity/AtomRegistry': '~effect/reactivity/AtomRegistry', get: atom => atom };`,
  'connection-fixture.js': `export const c = { preparedConnectionValueAtom: environmentId => ({ _tag: 'Some', value: { httpBaseUrl: location.origin } }) };`,
  'index-fixture.js': `import './panel-fixture.js'; import './registry-fixture.js'; import './connection-fixture.js';
new Worker('/assets/imageDecode.worker-fixture.js');`,
  // Like T3's image decoder worker: its top-level code must never run in the window.
  'imageDecode.worker-fixture.js': `if (typeof window !== 'undefined') globalThis.workerScriptRanInWindow = true;`,
};
const shell = {
  projects: [{ id: 'project-a', title: 'atelier', repositoryIdentity: { canonicalKey: fixtureRepositoryKey } }, { id: 'project-c', title: 'unlinked', repositoryIdentity: { canonicalKey: 'elsewhere/unlinked' } }],
  threads: [{ id: 'thread-a', projectId: 'project-a' }, { id: 'thread-b', projectId: 'project-a' }, { id: 'thread-c', projectId: 'project-c' }],
};
const html = `<!doctype html><html><head><meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; connect-src 'self' blob: http: https: ws: wss:; style-src 'self' 'unsafe-inline'; frame-src 'self' blob: http: https:; worker-src 'self' blob:"><title>Atelier extension fixture</title><script type="module" src="/assets/index-fixture.js"></script>${Object.keys(chunks).filter(name => !name.includes('worker')).map(name => `<link rel="modulepreload" href="/assets/${name}">`).join('')}<style>html{--background:#202124;--foreground:#eee;--border:#555;--secondary:#353639;--primary:#9bbcff}body{margin:0;background:#202124;color:#eee;font:14px system-ui}#app{height:100vh;display:flex;flex-direction:column}header{padding:12px;border-bottom:1px solid #555}#workspace{display:flex;flex:1;min-height:0}[data-workspace-area=conversation]{flex:1;padding:20px;min-width:0;overflow:auto}[data-workspace-area=panel]{width:750px;display:flex;flex-direction:column;min-width:0}[data-right-panel-tabbar]{height:52px;display:flex;align-items:center}[data-right-panel-tabbar]>div{display:flex;align-items:center}[data-right-panel-surface-content]{position:relative;flex:1;min-height:0}#native-content{padding:20px}#settings{overflow:auto;padding:24px;max-width:850px}#composer{margin-top:30px;border:1px solid #555;padding:12px}</style></head><body><div id="app"><header><span data-panel-layout-controls>Workspace controls</span></header><div id="workspace"><main data-workspace-area="conversation"><h1>Conversation</h1><p id="reference-prose">Investigate ${referenceId}, then <code>${referenceId}</code>. Ignore other-xabc and not-${referenceId}-extra.</p><pre>${referenceId}</pre><div id="composer" contenteditable="true">${referenceId}</div><p id="stream"></p></main><div data-workspace-area="panel"><div data-right-panel-tabbar><div><div data-active-tab="true"><button id="native-tab">Files</button></div><button aria-label="Add panel surface">+</button></div></div><div data-right-panel-surface-content><div id="native-content"><button id="native-control">Native file viewer</button></div></div></div></div><div id="settings" hidden><div data-settings-page-scroll><div><h1>General</h1><p>T3 settings fixture</p></div></div></div></div><script>
addEventListener('hashchange',()=>{const settings=location.hash.includes('/settings');document.querySelector('#settings').hidden=!settings;document.querySelector('#workspace').style.display=settings?'none':'flex'});
</script></body></html>`;
const server = http.createServer((request, response) => {
  const url = new URL(request.url, 'http://fixture');
  const chunk = chunks[url.pathname.replace('/assets/', '')];
  if (chunk) { response.writeHead(200, { 'content-type': 'text/javascript' }); response.end(chunk); return; }
  if (url.pathname === '/api/orchestration/shell') { response.writeHead(200, { 'content-type': 'application/json' }); response.end(JSON.stringify(shell)); return; }
  response.writeHead(200, { 'content-type': 'text/html' }); response.end(html);
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}`;
try {
  const configPath = path.join(root, 'config.json');
  const config = JSON.parse(await fs.readFile(await fs.access(configPath).then(() => configPath, () => path.join(root, 'config.example.json')), 'utf8'));
  deskBase = config.desk.baseUrl.replace(/\/$/, '');
  const repository = config.desk.repositories.find(repo => repo.repo === 'atelier');
  assert(repository, 'Integration tests require an atelier repository in config.json');
  repository.repositoryKeys = [fixtureRepositoryKey];
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
  await until(() => evaluate('Boolean(document.querySelector("[data-t3-desk-launch]"))'), 'Extension initialization');
  const discovery = await evaluate(`({ workerLoaded: performance.getEntriesByType('resource').some(entry => entry.name.includes('.worker-')), ranInWindow: globalThis.workerScriptRanInWindow ?? false, modules: __t3Custom.diagnostics().modules, probes: Object.entries(__t3Custom.diagnostics().probes).filter(([name]) => name.startsWith('module:')).map(([name, probe]) => name + '=' + probe.ok) })`);
  assert(discovery.workerLoaded, 'the fixture worker should be among the page resources');
  assert.equal(discovery.ranInWindow, false, `discovery imported a worker script into the window: ${discovery.modules.join(', ')}`);
  assert.deepEqual(discovery.probes, ['module:workspace-panel-store=true', 'module:atom-registry=true', 'module:prepared-connection-atom=true']);
  console.log(`PASS: real discovery found every hook in ${discovery.modules.length} declared modules and never ran the worker script in the window`);
  await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  await until(() => evaluate('Boolean(document.querySelector("[data-right-panel-surface-content] > [data-t3-desk-mount]:not([hidden])"))'), 'Docked Desk');
  const iframeSrc = await evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").src');
  assert.equal(iframeSrc, deskBase + '/r/atelier');
  assert(await evaluate('document.querySelector("#native-content").inert'));
  const geometry = await evaluate('(()=>{const host=document.querySelector("[data-right-panel-surface-content]").getBoundingClientRect();const frame=document.querySelector("[data-t3-desk-mount]").getBoundingClientRect();return {hostWidth:host.width,frameWidth:frame.width,hostLeft:host.left,frameLeft:frame.left}})()');
  assert.equal(geometry.hostWidth, geometry.frameWidth); assert.equal(geometry.hostLeft, geometry.frameLeft);
  await evaluate('document.querySelector("[data-workspace-area=panel]").style.width="900px"');
  assert.equal(await evaluate('document.querySelector("[data-t3-desk-mount]").getBoundingClientRect().width'), 900);
  console.log('PASS: Desk uses the real panel content slot, follows repository identity, and resizes with the host');

  const iframeTarget = await until(async () => (await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json()).find(target => target.type === 'iframe' && target.url.startsWith(deskBase)), 'Real Desk iframe');
  const child = await page.call('Target.attachToTarget', { targetId: iframeTarget.id, flatten: true });
  await until(async () => (await page.call('Runtime.evaluate', { expression: 'document.readyState === "complete" && document.body.innerText.includes("atelier") && !document.querySelector(".skel")', returnByValue: true }, child.sessionId)).result.value, 'Real Desk application rendering');
  // Real backend data is checked through the same allow-listed bridge as hovercards.
  const record = await evaluate(`fixtureApi.request('desk-get',{path:'/api/repos/atelier/items/${referenceId}'})`);
  assert.equal(record.row.id, referenceId);
  const denied = await evaluate(`fixtureApi.request('desk-get',{path:'/api/action'}).then(()=>false,error=>error.message)`);
  assert.match(denied, /not allowed/);
  console.log('PASS: authenticated-free Desk reads work through the bridge; action endpoints are rejected');

  await until(() => evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelectorAll("[data-atelier-id]").length === 2'), 'Reference scanning');
  const prose = await evaluate('document.querySelector("#reference-prose").textContent');
  await evaluate('globalThis.originalTextNode=document.querySelector("#reference-prose").firstChild;document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]").focus()');
  await until(() => evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard h3")?.textContent'), 'Hovercard backend response');
  assert.equal(await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard h3").textContent'), record.row.title);
  assert.equal(await evaluate('document.querySelector("#reference-prose").textContent'), prose);
  assert(await evaluate('document.querySelector("#reference-prose").firstChild === globalThis.originalTextNode'));
  await evaluate(`document.querySelector('#stream').append(document.createTextNode('Streaming ${referenceId}'));`);
  await until(() => evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelectorAll("[data-atelier-id]").length === 3'), 'Streaming reference update');
  assert.equal(await evaluate('document.querySelector("#stream").textContent'), `Streaming ${referenceId}`);
  await evaluate('(()=>{const link=document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]");link.blur();link.focus()})()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button"))'), 'Cached hovercard');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button").click()');
  await until(() => evaluate(`document.querySelector('[data-t3-desk-mount]').shadowRoot.querySelector('iframe').src.endsWith('/item/${referenceId}')`), 'Reference opens in Desk');
  console.log('PASS: configured-prefix references, inline code, real hovercards, click-through, and streaming without text rewriting');

  await evaluate('globalThis.keptFrame=document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe");location.hash="/env/thread-b"');
  await delay(500); await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").hidden'), 'Desk on second thread');
  assert(await evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe") === globalThis.keptFrame'));
  assert((await evaluate('globalThis.keptFrame.src')).endsWith('/item/' + referenceId));
  await evaluate(`(()=>{const p=document.createElement('p');p.id='mission-reference';p.textContent='${missionId}';document.querySelector('[data-workspace-area=conversation]').append(p)})()`);
  await until(() => evaluate(`Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id=${missionId}]"))`), 'Mission reference scanning');
  await evaluate(`(()=>{document.getSelection().removeAllRanges();const p=document.querySelector('#mission-reference');const range=document.createRange();range.selectNodeContents(p);const r=range.getBoundingClientRect();p.dispatchEvent(new MouseEvent('click',{bubbles:true,cancelable:true,clientX:r.left+3,clientY:r.top+3}))})()`);
  await until(() => evaluate(`globalThis.keptFrame.src.endsWith("/mission/${missionId}")`), 'Direct mission reference click');
  console.log('PASS: direct mouse activation resolves missions before navigating, without requiring a hovercard');
  await evaluate('location.hash="/env/thread-c"');
  await delay(500); await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  await until(() => evaluate('document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").hidden'), 'Unmapped project empty state');
  console.log('PASS: same-project thread switches preserve Desk navigation; unmapped projects hide stale repository content');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("[data-atelier-id]").focus()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard h3"))'), 'Reference in unmapped project');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#atelier-hovercard button").click()');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").hidden'), 'Cross-project reference override');
  console.log('PASS: explicit reference navigation works from an unmapped project');

  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#extension-settings").click()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings"))'), 'Settings section');
  await evaluate('(()=>{const select=document.querySelector("#t3-custom-settings").shadowRoot.querySelector("select");select.value="atelier";select.dispatchEvent(new Event("change"))})()');
  await evaluate('location.hash="/env/thread-c"');
  await until(() => evaluate('!document.querySelector("[data-t3-desk-mount]").shadowRoot.querySelector("iframe").hidden'), 'Explicit project mapping');
  await evaluate('document.querySelector("#native-tab").click()');
  await until(() => evaluate('document.querySelector("[data-t3-desk-mount]").hidden && !document.querySelector("#native-content").inert'), 'Native tab restoration');
  console.log('PASS: Settings project mapping and switching back to native surfaces');

  const timeOrigin = await evaluate('performance.timeOrigin');
  await evaluate('document.querySelector("#t3-custom-root").shadowRoot.querySelector("#extension-settings").click()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-settings"))'), 'Settings remount');
  await evaluate('globalThis.previousRoot=document.querySelector("#t3-custom-root");document.querySelector("#t3-custom-settings").shadowRoot.querySelector("#reload-scripts").click()');
  await until(() => evaluate('Boolean(document.querySelector("#t3-custom-root")) && document.querySelector("#t3-custom-root") !== globalThis.previousRoot'), 'Settings reinjection control');
  assert.equal(await evaluate('performance.timeOrigin'), timeOrigin);
  assert.equal(await evaluate('fixtureListeners.size'), 1);
  // By now the worker is in the page's resource timings, as after a TIFF/HEIF preview in T3.
  assert.equal(await evaluate('globalThis.workerScriptRanInWindow ?? false'), false, 'reinjection imported a worker script into the window');
  assert.equal(await evaluate('document.querySelector("#t3-custom-settings").shadowRoot.querySelector("select").disabled'), false);
  await evaluate('location.hash="/env/thread-c"');
  await delay(500); await evaluate('document.querySelector("[data-t3-desk-launch]").click()');
  const finalFrame = await until(async () => (await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json()).find(target => target.type === 'iframe' && target.url.startsWith(deskBase)), 'Reinjected Desk iframe');
  const finalChild = await page.call('Target.attachToTarget', { targetId: finalFrame.id, flatten: true });
  await until(async () => (await page.call('Runtime.evaluate', { expression: 'document.readyState === "complete" && document.body.innerText.includes("atelier") && !document.querySelector(".skel")', returnByValue: true }, finalChild.sessionId)).result.value, 'Reinjected Desk rendering');
  await fs.mkdir(path.join(root, 'artifacts'), { recursive: true });
  const screenshot = await page.call('Page.captureScreenshot', { format: 'png' });
  await fs.writeFile(path.join(root, 'artifacts/extension-docked.png'), Buffer.from(screenshot.data, 'base64'));
  console.log('PASS: Settings reinjection preserves the document and cleans up old subscriptions');

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
}
