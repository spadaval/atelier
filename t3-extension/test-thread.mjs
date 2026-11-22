// Checks This thread (src/thread) against fixture threads (check/thread-fixtures.ts): a thread that
// implements an item, one that leads a mission, one with children and mentions only, one that
// planned a mission still proposed, and one with nothing, which shows its project; what each lists,
// what its rows and titles open, and that a failed read, or a repository the Desk answers it could
// not read, is named and never read as "holds nothing", "nothing is started" or "nothing can start".
// It builds check/thread.tsx with the extension's own Vite config and draws it in a page that
// carries T3 Code's stylesheet, as last copied, for T3's colours, in light and dark. Screenshots go
// to artifacts/thread-*.png.
//
//   node test-thread.mjs        # BROWSER=<chromium> to choose the browser
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import http from 'node:http';
import os from 'node:os';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';
import tailwindcss from '@tailwindcss/vite';
import { build } from 'vite';
import { connectPage } from './devtools.mjs';

const root = path.dirname(fileURLToPath(import.meta.url));
const temp = await fs.mkdtemp(path.join(os.tmpdir(), 'atelier-thread-test-'));
const debugPort = 19225;
const browserPath = process.env.BROWSER ?? ({ win32: 'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe', darwin: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' }[process.platform] ?? '/usr/bin/google-chrome');

let browser, server, page;
const failures = [];
async function check(name, body) {
  try { await body(); console.log(`PASS: ${name}`); }
  catch (error) { console.log(`FAIL: ${name}: ${error.message}`); failures.push(name); }
}
try {
  const quiet = { logLevel: 'warn', root };
  await build({ ...quiet, configFile: path.join(root, 'vite.config.ts'), build: { outDir: path.join(temp, 'page'), emptyOutDir: true, lib: { entry: path.join(root, 'check/thread.tsx'), formats: ['iife'], name: 'ThreadCheck', fileName: () => 'thread.js' } } });
  const warn = console.warn;
  console.warn = (message, ...rest) => { if (!/while optimizing generated CSS/.test(message)) warn(message, ...rest); };
  try {
    await build({ ...quiet, configFile: false, plugins: [tailwindcss()], build: { outDir: path.join(temp, 'reference'), emptyOutDir: true, lib: { entry: path.join(root, 'check/reference.ts'), formats: ['es'], fileName: () => 'reference.js', cssFileName: 'reference' } } });
  } finally { console.warn = warn; }
  const script = await fs.readFile(path.join(temp, 'page/thread.js'), 'utf8');
  const t3Css = await fs.readFile(path.join(temp, 'reference/reference.css'), 'utf8');
  server = http.createServer((request, response) => {
    const url = new URL(request.url, 'http://localhost');
    if (url.pathname === '/thread.js') { response.setHeader('content-type', 'text/javascript'); response.end(script); return; }
    response.setHeader('content-type', 'text/html');
    response.end(`<!doctype html><html class="${url.searchParams.get('appearance') === 'dark' ? 'dark' : ''}"><head><meta charset="utf-8"><style>${t3Css} body { margin: 0; padding: 8px; background: var(--background); }</style></head><body><script src="/thread.js"></script></body></html>`);
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  const env = { ...process.env }; delete env.ELECTRON_RUN_AS_NODE; delete env.NODE_OPTIONS;
  const browserLog = await fs.open(path.join(temp, 'browser.log'), 'w');
  browser = spawn(browserPath, ['--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking', `--user-data-dir=${path.join(temp, 'profile')}`, `--remote-debugging-port=${debugPort}`, '--remote-debugging-address=127.0.0.1', '--window-size=1500,1000', `${base}/`], { env, stdio: ['ignore', browserLog.fd, browserLog.fd] });
  await browserLog.close();
  page = await until(() => connectPage(debugPort, base), 'Browser startup', 120000);
  await fs.mkdir(path.join(root, 'artifacts'), { recursive: true });

  // Opens a scenario at a width and waits until the page has drawn from its reads.
  // `fail` names a path the stand-in injector fails (check/thread.tsx).
  // `unreadable` names a path answered as the Desk answers for a repository it could not read.
  const open = async (scenario, width = 520, appearance = 'light', fail = null, unreadable = null) => {
    const url = `${base}/?${new URLSearchParams({ scenario, width, appearance, ...(fail ? { fail } : {}), ...(unreadable ? { unreadable } : {}) })}`;
    await page.call('Page.navigate', { url });
    await until(() => page.evaluate(`location.href === ${JSON.stringify(url)} && document.readyState === 'complete' && document.querySelector('#thread')?.shadowRoot?.querySelector('[data-this-thread]')?.dataset.thisThread !== 'loading'`), `${scenario} drawn`);
    // The fixture answers at once; the reads that follow the ties (pages, chips, the glance) are in by then.
    await delay(300);
  };
  const shot = async name => {
    const shot = await page.call('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false });
    await fs.writeFile(path.join(root, `artifacts/thread-${name}.png`), Buffer.from(shot.data, 'base64'));
  };
  const read = expression => page.evaluate(`(() => { const p = document.querySelector('#thread').shadowRoot.querySelector('[data-this-thread]'); const text = el => el?.textContent.replace(/\\s+/g, ' ').trim() ?? null; return ${expression}; })()`);
  const rows = section => read(`[...p.querySelectorAll('[data-section="${section}"] [data-row]')].map(r => r.dataset.row)`);
  const sections = () => read(`[...p.querySelectorAll('[data-section]')].map(s => s.dataset.section)`);
  const notes = () => read(`[...p.querySelectorAll('[data-note]')].map(n => text(n))`);
  const calls = () => page.evaluate('threadCalls.splice(0)');
  const click = selector => read(`(p.querySelector(${JSON.stringify(selector)}).click(), true)`);
  const noFrame = async () => assert.equal(await page.evaluate(`document.querySelector('#thread').shadowRoot.querySelector('iframe')`), null, 'This thread drew an iframe');

  await open('implements-item');
  await check('a thread that implements an item: its card under Holds, with Done when, the pull request, and what each opens', async () => {
    assert.equal(await read('p.dataset.thisThread'), 'holds');
    assert.equal(await read(`text(p.querySelector('h1'))`), 'Implement atelier-ti01');
    assert.deepEqual(await sections(), ['holds']);
    assert.match(await read(`text(p.querySelector('[data-card="item:atelier-ti01"]'))`), /^atelier-ti01 Draw This thread in T3's panelBuildingThis thread implements this item\.Done when 1\/3The panel shows This thread with no iframe\.A read that fails is named\.Screenshots in light and dark\.#301 openOpen item$/);
    assert.equal(await read(`p.querySelectorAll('[data-card="item:atelier-ti01"] [data-ticked]').length`), 1);
    // It holds something now, so what it did before is not said.
    assert.doesNotMatch(await read('text(p)'), /Earlier it/);
    await calls();
    await click('[data-title="atelier-ti01"]');
    await click('[data-pr="301"]');
    await click('[data-action="inbox"]');
    assert.deepEqual(await calls(), [['item', 'atelier', 'atelier-ti01'], ['url', 'https://example.invalid/pull/301'], ['inbox']]);
    assert.deepEqual(await notes(), []);
    await noFrame();
    assert.deepEqual(await page.evaluate('threadErrors'), []);
  });
  await check('a failed read of its ties is named, and the page does not say the thread holds nothing', async () => {
    await open('implements-item', 520, 'light', '/api/ties?thread=thread%3Aimpl');
    assert.notEqual(await read('p.dataset.thisThread'), 'empty');
    assert.doesNotMatch(await read('text(p)'), /holds nothing/);
    assert.deepEqual(await notes(), ["Could not read this thread's records (GET /api/ties?thread=thread%3Aimpl): GET /api/ties?thread=thread%3Aimpl: HTTP 502: the fixture failed it"]);
  });
  await check('a failed read of its item\'s page leaves the card\'s details out and names it', async () => {
    await open('implements-item', 520, 'light', '/api/repos/atelier/items/atelier-ti01');
    assert.match(await read(`text(p.querySelector('[data-card="item:atelier-ti01"]'))`), /^atelier-ti01 Draw This thread in T3's panelBuildingThis thread implements this item\.Open item$/);
    assert.match((await notes())[0], /^Could not read the page of atelier-ti01 \(GET \/api\/repos\/atelier\/items\/atelier-ti01\)/);
  });

  await open('leads-mission');
  await check('a thread that leads a mission: the mission under Holds and under Work, a chip per thread, and its title opens the mission page', async () => {
    assert.deepEqual(await sections(), ['holds', 'work']);
    // Holds lists the open items; Work lists every item, with a chip for each implementer and reviewer.
    assert.deepEqual(await rows('holds'), ['item:atelier-tl01', 'item:atelier-tl02']);
    assert.deepEqual(await rows('work'), ['item:atelier-tl01', 'item:atelier-tl02', 'item:atelier-tl03']);
    assert.match(await read(`text(p.querySelector('[data-card="mission:atelier-tlm1"]'))`), /1\/3 done.*This thread leads this mission\./);
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-chip]')].map(c => text(c))`), ['implrunningopus-5-5', 'reviewrunningopus-5-5', 'implcompletedopus-5-5']);
    // Its child works on an item the mission shows, so it is not listed apart.
    assert.equal(await read(`p.querySelector('[data-section="started"]')`), null);
    await calls();
    await click('[data-card="mission:atelier-tlm1"] [data-title]');
    await click('[data-card="work:atelier-tlm1"] [data-title]');
    await click('[data-chip="thread:tl-review"]');
    await click('[data-section="work"] [data-row="item:atelier-tl03"]');
    assert.deepEqual(await calls(), [['mission', 'atelier', 'atelier-tlm1', false], ['mission', 'atelier', 'atelier-tlm1', false], ['thread', 'thread:tl-review'], ['item', 'atelier', 'atelier-tl03']]);
    await noFrame();
  });

  await open('children-and-mentions');
  await check('a thread with children and mentions only: Waiting on you, the threads it started, and Mentioned', async () => {
    assert.equal(await read('p.dataset.thisThread'), 'holds');
    assert.deepEqual(await sections(), ['waiting', 'started', 'mentioned']);
    assert.deepEqual(await rows('waiting'), ['item:atelier-tm01']);
    assert.deepEqual(await rows('started'), ['thread:thread:c-quiet', 'thread:thread:c-gone']);
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:c-quiet"]'))`), /^Review atelier-tm01Runninggpt-6\.1-solreviews atelier-tm01Quiet 47m$/);
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:c-gone"]'))`), /T3 Code does not know it/);
    // The open records first, then the done ones after a divider; the 404 is not named and not a failure.
    assert.deepEqual(await rows('mentioned'), ['item:atelier-tm01', 'mission:atelier-tmm1', 'item:atelier-tm02']);
    assert.match(await read(`text(p.querySelector('[data-section="mentioned"]'))`), /Done or dropped: 1 of 3/);
    assert.deepEqual(await notes(), []);
    await calls();
    await click('[data-row="thread:thread:c-quiet"]');
    await click('[data-section="mentioned"] [data-row="mission:atelier-tmm1"]');
    await click('[data-section="waiting"] [data-row="item:atelier-tm01"]');
    assert.deepEqual(await calls(), [['thread', 'thread:c-quiet'], ['mission', 'atelier', 'atelier-tmm1', false], ['item', 'atelier', 'atelier-tm01']]);
  });
  await check('a failed read of a mentioned record is named, and the page does not say the thread holds nothing', async () => {
    await open('children-and-mentions', 520, 'light', '/api/records/atelier-tmm1');
    assert.deepEqual(await rows('mentioned'), ['item:atelier-tm01', 'item:atelier-tm02']);
    assert.match((await notes())[0], /^Could not read atelier-tmm1, which its conversation names \(GET \/api\/records\/atelier-tmm1\)/);
  });

  await check('a failed read of a child\'s status is named, and its row says it could not be read, not that T3 Code does not know it', async () => {
    await open('children-and-mentions', 520, 'light', '/api/thread?id=thread%3Ac-quiet');
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:c-quiet"]'))`), /Could not be read/);
    assert.doesNotMatch(await read(`text(p.querySelector('[data-row="thread:thread:c-quiet"]'))`), /does not know/);
    // The thread T3 Code does not know (404) is still said to be so, and is not a failure.
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:c-gone"]'))`), /T3 Code does not know it/);
    assert.deepEqual(await notes(), ["Could not read T3 Code's status of thread …-quiet, a thread it started (GET /api/thread?id=thread%3Ac-quiet): GET /api/thread?id=thread%3Ac-quiet: HTTP 502: the fixture failed it"]);
  });
  await check('a failed read of a thread on a mission\'s item is named, and its chip says not read', async () => {
    await open('leads-mission', 520, 'light', '/api/thread?id=thread%3Atl-impl2');
    assert.equal(await read(`text(p.querySelector('[data-chip="thread:tl-impl2"]'))`), 'implnot read');
    assert.match((await notes())[0], /^Could not read T3 Code's status of thread …-impl2, a thread on an item of its missions \(GET \/api\/thread\?id=thread%3Atl-impl2\)/);
  });
  await check('a failed read of the thread\'s own title is named', async () => {
    await open('implements-item', 520, 'light', '/api/thread?id=thread%3Aimpl');
    assert.equal(await read(`text(p.querySelector('h1'))`), 'This thread');
    assert.match((await notes())[0], /^Could not read this thread's title and status \(GET \/api\/thread\?id=thread%3Aimpl\)/);
  });
  await check('a failed read of a holder\'s title in the project at a glance is named', async () => {
    await open('nothing', 520, 'light', '/api/thread?id=thread%3Atl-impl2');
    assert.equal(await read('p.dataset.thisThread'), 'empty');
    assert.match((await notes())[0], /^Could not read the title of thread …-impl2 \(GET \/api\/thread\?id=thread%3Atl-impl2\)/);
  });

  await open('planned-proposed');
  await check('a thread that planned a mission still proposed: the plan waits on you, and every title of it opens its record page', async () => {
    assert.deepEqual(await sections(), ['waiting', 'holds', 'work']);
    assert.deepEqual(await rows('waiting'), ['mission:atelier-tpm1']);
    assert.match(await read(`text(p.querySelector('[data-section="waiting"]'))`), /Plan proposed/);
    assert.match(await read(`text(p.querySelector('[data-card="mission:atelier-tpm1"]'))`), /This thread plans this mission, a plan still proposed\./);
    await calls();
    await click('[data-section="waiting"] [data-row="mission:atelier-tpm1"]');
    await click('[data-card="mission:atelier-tpm1"] [data-title]');
    await click('[data-card="work:atelier-tpm1"] [data-title]');
    assert.deepEqual(await calls(), Array(3).fill(['mission', 'atelier', 'atelier-tpm1', true]));
  });

  await open('nothing');
  await check('a thread with nothing: it says so, and shows its project at a glance', async () => {
    assert.equal(await read('p.dataset.thisThread'), 'empty');
    assert.equal(await read(`text(p.querySelector('[data-empty]'))`), 'This thread holds nothing, started nothing and names no record. Here is atelier.');
    assert.deepEqual(await sections(), ['waiting', 'missions', 'motion', 'next']);
    assert.deepEqual(await rows('waiting'), ['item:atelier-tn01']);
    // Only the agreed mission: the proposed plan is not one.
    assert.deepEqual(await rows('missions'), ['mission:atelier-tlm1']);
    assert.match(await read(`text(p.querySelector('[data-section="motion"]'))`), /atelier-tl02.*building, Implement atelier-tl02/);
    assert.deepEqual(await rows('next'), ['item:atelier-tn02']);
    await calls();
    await click('[data-section="missions"] [data-row="mission:atelier-tlm1"]');
    await click('[data-section="next"] [data-row="item:atelier-tn02"]');
    assert.deepEqual(await calls(), [['mission', 'atelier', 'atelier-tlm1', false], ['item', 'atelier', 'atelier-tn02']]);
    await noFrame();
  });
  await check('with nothing but a failed inbox read, the page names it and does not say the thread holds nothing', async () => {
    await open('nothing', 520, 'light', '/api/inbox');
    assert.equal(await read('p.dataset.thisThread'), 'holds');
    assert.doesNotMatch(await read('text(p)'), /holds nothing/);
    assert.match((await notes())[0], /^Could not read the inbox, for what waits on you \(GET \/api\/inbox\)/);
  });
  await check('with nothing, a repository the inbox could not read is named, and the page does not say the thread holds nothing', async () => {
    await open('nothing', 520, 'light', null, '/api/inbox');
    assert.equal(await read('p.dataset.thisThread'), 'holds');
    assert.doesNotMatch(await read('text(p)'), /holds nothing/);
    assert.deepEqual(await sections(), []);
    assert.deepEqual(await notes(), ['Could not read the repository atelier, for what waits on you (GET /api/inbox): fixture repository unreadable']);
  });
  await check('the glance names a repository the Desk could not read for what is in motion, and does not say nothing is started or can start', async () => {
    await open('nothing', 520, 'light', null, '/api/motion');
    assert.equal(await read('p.dataset.thisThread'), 'empty');
    assert.deepEqual(await sections(), ['waiting', 'missions']);
    assert.doesNotMatch(await read('text(p)'), /Nothing is started|Nothing can start/);
    assert.deepEqual(await notes(), ['The Desk could not read what is in motion in atelier (GET /api/motion): fixture repository unreadable']);
  });

  for (const appearance of ['light', 'dark']) {
    for (const scenario of ['implements-item', 'leads-mission', 'children-and-mentions', 'planned-proposed', 'nothing']) {
      await open(scenario, 520, appearance);
      await shot(`${scenario}-${appearance}`);
    }
  }
  console.log(`Screenshots in ${path.join(root, 'artifacts')}/thread-*.png`)
} catch (error) {
  console.log(`FAIL: ${error.stack ?? error.message}`);
  failures.push('setup');
} finally {
  page?.close?.();
  browser?.kill();
  server?.close();
  await fs.rm(temp, { recursive: true, force: true }).catch(() => {});
}
process.exitCode = failures.length ? 1 : 0;

async function until(check, label, timeout = 20000) {
  const end = Date.now() + timeout; let lastError;
  while (Date.now() < end) {
    try { const result = await check(); if (result) return result; } catch (error) { lastError = error; }
    await delay(100);
  }
  throw new Error(`${label} timed out${lastError ? ': ' + lastError.message : ''}`);
}
