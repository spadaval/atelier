// Checks the mission page (src/mission) against fixture missions (check/mission-fixtures.ts): what
// it says, what it lists under Needs you and At work, what it folds, what its buttons and keys
// open, the done page's lanes and merged items, and its layout at 500, 728, 1,100 and 1,400 pixels.
// It builds check/mission.tsx with the extension's own Vite config and draws it in a page that
// carries T3 Code's stylesheet, as last copied, for T3's colours, in light and dark. Screenshots go
// to artifacts/mission-*.png.
//
//   node test-mission.mjs        # BROWSER=<chromium> to choose the browser
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
const temp = await fs.mkdtemp(path.join(os.tmpdir(), 'atelier-mission-test-'));
const debugPort = 19224;
const browserPath = process.env.BROWSER ?? ({ win32: 'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe', darwin: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' }[process.platform] ?? '/usr/bin/google-chrome');

let browser, server, page;
const failures = [];
async function check(name, body) {
  try { await body(); console.log(`PASS: ${name}`); }
  catch (error) { console.log(`FAIL: ${name}: ${error.message}`); failures.push(name); }
}
try {
  const quiet = { logLevel: 'warn', root };
  await build({ ...quiet, configFile: path.join(root, 'vite.config.ts'), build: { outDir: path.join(temp, 'page'), emptyOutDir: true, lib: { entry: path.join(root, 'check/mission.tsx'), formats: ['iife'], name: 'MissionCheck', fileName: () => 'mission.js' } } });
  const warn = console.warn;
  console.warn = (message, ...rest) => { if (!/while optimizing generated CSS/.test(message)) warn(message, ...rest); };
  try {
    await build({ ...quiet, configFile: false, plugins: [tailwindcss()], build: { outDir: path.join(temp, 'reference'), emptyOutDir: true, lib: { entry: path.join(root, 'check/reference.ts'), formats: ['es'], fileName: () => 'reference.js', cssFileName: 'reference' } } });
  } finally { console.warn = warn; }
  const script = await fs.readFile(path.join(temp, 'page/mission.js'), 'utf8');
  const t3Css = await fs.readFile(path.join(temp, 'reference/reference.css'), 'utf8');
  server = http.createServer((request, response) => {
    const url = new URL(request.url, 'http://localhost');
    if (url.pathname === '/mission.js') { response.setHeader('content-type', 'text/javascript'); response.end(script); return; }
    response.setHeader('content-type', 'text/html');
    response.end(`<!doctype html><html class="${url.searchParams.get('appearance') === 'dark' ? 'dark' : ''}"><head><meta charset="utf-8"><style>${t3Css} body { margin: 0; padding: 8px; background: var(--background); }</style></head><body><script src="/mission.js"></script></body></html>`);
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  const env = { ...process.env }; delete env.ELECTRON_RUN_AS_NODE; delete env.NODE_OPTIONS;
  const browserLog = await fs.open(path.join(temp, 'browser.log'), 'w');
  browser = spawn(browserPath, ['--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking', `--user-data-dir=${path.join(temp, 'profile')}`, `--remote-debugging-port=${debugPort}`, '--remote-debugging-address=127.0.0.1', '--window-size=1500,1000', `${base}/`], { env, stdio: ['ignore', browserLog.fd, browserLog.fd] });
  await browserLog.close();
  page = await until(() => connectPage(debugPort, base), 'Browser startup', 120000);
  await fs.mkdir(path.join(root, 'artifacts'), { recursive: true });

  // Opens a scenario at a width and waits until its page has drawn from its reads.
  // `fail` names a path the stand-in injector answers `after` times and then fails (check/mission.tsx).
  const open = async (scenario, width = 1000, appearance = 'light', fail = null) => {
    const url = `${base}/?${new URLSearchParams({ scenario, width, appearance, ...(fail ?? {}) })}`;
    await page.call('Page.navigate', { url });
    await until(() => page.evaluate(`location.href === ${JSON.stringify(url)} && document.readyState === 'complete' && document.querySelector('#mission')?.shadowRoot?.querySelector('[data-mission-page]')?.dataset.missionPage !== 'loading'`), `${scenario} drawn`);
    // The fixture answers at once; the reads that follow the mission's (history, threads, items) are in by then.
    await delay(300);
  };
  const shot = async name => {
    const shot = await page.call('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false });
    await fs.writeFile(path.join(root, `artifacts/mission-${name}.png`), Buffer.from(shot.data, 'base64'));
  };
  const read = expression => page.evaluate(`(() => { const p = document.querySelector('#mission').shadowRoot.querySelector('[data-mission-page]'); const text = el => el?.textContent.replace(/\\s+/g, ' ').trim() ?? null; return ${expression}; })()`);
  const rows = section => read(`[...p.querySelectorAll('[data-section="${section}"] [data-row]')].map(r => r.dataset.row)`);
  const calls = () => page.evaluate('missionCalls.splice(0)');
  const click = selector => read(`(p.querySelector(${JSON.stringify(selector)}).click(), true)`);
  const key = async (keyName, code = keyName) => {
    for (const type of ['keyDown', 'keyUp']) await page.call('Input.dispatchKeyEvent', { type, key: keyName, code, windowsVirtualKeyCode: { Enter: 13, Escape: 27, l: 76 }[keyName] ?? 0, text: type === 'keyDown' ? ({ Enter: '\r', l: 'l' }[keyName]) : undefined });
  };

  await open('needs-you');
  await check('running, with a needs-human item, a quiet thread and two live threads: the sentence, Needs you and At work', async () => {
    assert.equal(await read(`p.dataset.missionPage`), 'running');
    assert.equal(await read(`text(p.querySelector('h1'))`), 'Make the mission page say what needs you');
    assert.equal(await read(`text(p.querySelector('[data-state]'))`), 'Running');
    assert.match(await read(`text(p)`), /1 of 8 merged/);
    assert.equal(await read(`text(p.querySelector('[data-summary]'))`), 'Two items are being built, one is in review and one is merging. One item waits on you and one thread has been quiet for 47m.');
    assert.deepEqual(await rows('needs-you'), ['item:atelier-r001', 'thread:thread:quiet']);
    assert.match(await read(`text(p.querySelector('[data-row="item:atelier-r001"]'))`), /atelier-r001 is labelled needs-human\. One item waits behind it\.Open item$/);
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:quiet"]'))`), /^Draw the lanes from commit subjectsReviewer has been quiet for 47m\. Last: \$ cargo nextest runOpen reviewer$/);
    // The lead, still changing, and the two implementers are at work; the ended thread is not listed.
    assert.deepEqual(await rows('at-work'), ['thread:thread:lead', 'thread:thread:busy', 'thread:thread:merging']);
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:busy"]'))`), /^Read every Desk through the loader's gate\$ pnpm vitest run app\/src\/modules\/route38m$/);
    assert.deepEqual(await page.evaluate('missionErrors'), []);
  });
  await check('running: Not started, Merged and Outcome and success criteria are each one folded line that opens in place; no list of the mission\'s steps', async () => {
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-fold]')].map(f => [f.dataset.fold, text(f), f.children.length])`), [
      ['Not started', 'Not started1 can start, 1 wait on others, 1 draft', 1],
      ['Merged', 'Merged1', 1],
      ['Outcome and success criteria', 'Outcome and success criteria', 1],
    ]);
    await click('[data-fold="Merged"] button');
    assert.match(await read(`text(p.querySelector('[data-fold="Merged"]'))`), /Show the Result as writtenmerged 2h 0m ago.*commit subjects/);
    await click('[data-fold="Not started"] button');
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-fold="Not started"] [data-row]')].map(r => text(r))`), ['Stack the page below 560 pixelscan start', 'Name each key on its buttondraft', 'Wait for the decisionwaits on atelier-r001']);
    await click('[data-fold="Outcome and success criteria"] button');
    assert.match(await read(`text(p.querySelector('[data-fold="Outcome and success criteria"]'))`), /says what needs you\..*no progress log.*needs-you-first/);
    assert.doesNotMatch(await read('text(p)'), /Progress Log|Plan merged: mission agreed|Started atelier-|Submitted atelier-/);
  });
  await check('running: each row\'s button opens the item or the thread; L opens the lead, Enter the focused row, Esc closes', async () => {
    await calls();
    await click('[data-row="item:atelier-r001"] button');
    await click('[data-row="thread:thread:quiet"] button');
    await click('[data-row="thread:thread:busy"]');
    await click('[data-fold="Merged"] [data-row="item:atelier-r005"]');
    assert.deepEqual(await calls(), [['item', 'atelier-r001'], ['thread', 'thread:quiet'], ['thread', 'thread:busy'], ['item', 'atelier-r005']]);
    // The keys, each shown on its button.
    assert.equal(await read(`text(p.querySelector('[data-action="lead"] kbd'))`), 'L');
    assert.equal(await read(`text(p.querySelector('[data-action="close"] kbd'))`), 'Esc');
    assert.equal(await read(`[...p.querySelectorAll('kbd')].length`), 2, 'a key is listed somewhere other than its button');
    await read(`(p.focus(), true)`);
    await key('l');
    await read(`(p.querySelector('[data-row="thread:thread:merging"]').focus(), true)`);
    await key('Enter');
    await key('Escape');
    assert.deepEqual(await calls(), [['thread', 'thread:lead'], ['thread', 'thread:merging'], ['close']]);
  });
  await check('running: the page reads the mission, its history and each listed thread in full, through the gate\'s forms', async () => {
    const reads = [...new Set(await page.evaluate('missionReads'))].sort();
    assert.deepEqual(reads, [
      '/api/history?repo=atelier&ids=atelier-rm01,atelier-r001,atelier-r002,atelier-r003,atelier-r004,atelier-r005,atelier-r006,atelier-r007,atelier-r008',
      '/api/repos/atelier/missions/atelier-rm01',
      '/api/thread?id=thread%3Abusy&full=1', '/api/thread?id=thread%3Alead&full=1', '/api/thread?id=thread%3Amerging&full=1', '/api/thread?id=thread%3Aquiet&full=1',
    ]);
  });

  // Layout, on the running page with the most in it.
  const layout = () => read(`(() => {
    const box = el => el.getBoundingClientRect();
    const needs = p.querySelector('[data-section="needs-you"]'), work = p.querySelector('[data-section="at-work"]');
    const ask = p.querySelector('[data-row="item:atelier-r001"]');
    // The widest line of text, in widths of its font's 0 (CSS's ch): each element with text of its own.
    const canvas = document.createElement('canvas').getContext('2d');
    let widest = { chars: 0, text: '' };
    for (const el of p.querySelectorAll('*')) {
      if (![...el.childNodes].some(n => n.nodeType === 3 && n.textContent.trim())) continue;
      const style = getComputedStyle(el); canvas.font = style.font;
      // The line as drawn: its text's width, clipped to its element where it is truncated.
      const range = document.createRange(); range.selectNodeContents(el);
      const chars = Math.min(box(el).width, Math.max(0, ...[...range.getClientRects()].map(r => r.width))) / canvas.measureText('0').width;
      if (chars > widest.chars) widest = { chars: Math.round(chars), text: el.textContent.slice(0, 40) };
    }
    return { overflow: p.scrollWidth - p.clientWidth, sideBySide: Math.abs(box(needs).top - box(work).top) < 2 && box(work).left > box(needs).right, stacked: box(ask.querySelector('button')).top >= box(ask.querySelector('.min-w-0')).bottom - 1, lastLineShown: getComputedStyle(p.querySelector('[data-row="thread:thread:busy"] .font-mono')).display !== 'none', widest };
  })()`);
  await check('layout: holds at 728 pixels; stacks below 560; Needs you and At work side by side from 1,100; no line over 75 characters', async () => {
    for (const [width, expected] of [[500, { sideBySide: false, stacked: true, lastLineShown: false }], [728, { sideBySide: false, stacked: false, lastLineShown: true }], [1100, { sideBySide: true, stacked: false, lastLineShown: true }], [1400, { sideBySide: true, stacked: false, lastLineShown: true }]]) {
      await open('needs-you', width);
      const got = await layout();
      assert.equal(got.overflow, 0, `${width}px: the page scrolls sideways by ${got.overflow}px`);
      assert.deepEqual({ sideBySide: got.sideBySide, stacked: got.stacked, lastLineShown: got.lastLineShown }, expected, `${width}px`);
      assert(got.widest.chars <= 75, `${width}px: "${got.widest.text}" is ${got.widest.chars} characters wide`);
      await shot(`running-${width}`);
    }
  });

  await open('waiting');
  await check('running, with no needs-human item and a thread that began waiting a minute ago: it is listed under Needs you', async () => {
    assert.deepEqual(await rows('needs-you'), ['thread:thread:asks']);
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:asks"]'))`), /Implementer is waiting on you\. Last: Which Desk should the page read\?Open implementer$/);
    assert.equal(await read(`text(p.querySelector('[data-summary]'))`), 'One item is being built. One thread waits on you.');
    assert.deepEqual(await rows('at-work'), []);
    // No lead: L opens the planner, whose thread T3 Code does not list.
    assert.equal(await read(`text(p.querySelector('[data-action="lead"]'))`), 'PlannerL');
  });

  await open('calm');
  await check('running, with nothing needing the human: no Needs you section, and the sentence says so', async () => {
    assert.equal(await read(`p.querySelector('[data-section="needs-you"]')`), null);
    assert.equal(await read(`text(p.querySelector('[data-summary]'))`), 'One item is being built. Nothing needs you.');
    assert.deepEqual(await rows('at-work'), ['thread:thread:calm']);
  });

  // A thread's full read can fail on its own while T3 Code's list is read: the page must not then
  // move a quiet thread to At work, or say nothing needs the human.
  const threadPath = id => `/api/thread?id=${encodeURIComponent(id)}&full=1`;
  await open('needs-you', 1000, 'light', { fail: threadPath('thread:quiet'), after: 1 });
  await check('running: a quiet thread whose read answers and then fails stays under Needs you, from its last answer, with a note', async () => {
    assert.deepEqual(await rows('needs-you'), ['item:atelier-r001', 'thread:thread:quiet']);
    await page.evaluate('document.dispatchEvent(new Event("visibilitychange"))');
    await until(() => read(`text(p.querySelector('[data-note]'))?.startsWith('One thread could not be read again')`), 'The failed read noted');
    assert.deepEqual(await rows('needs-you'), ['item:atelier-r001', 'thread:thread:quiet']);
    assert.deepEqual(await rows('at-work'), ['thread:thread:lead', 'thread:thread:busy', 'thread:thread:merging']);
    assert.match(await read(`text(p.querySelector('[data-note]'))`), /HTTP 502/);
  });
  await open('calm', 1000, 'light', { fail: threadPath('thread:calm'), after: 0 });
  await check('running: a thread whose read never answers is listed as not read, and the sentence does not say nothing needs you', async () => {
    assert.equal(await read(`text(p.querySelector('[data-summary]'))`), 'One item is being built. Whether one thread needs you is not known yet.');
    assert.deepEqual(await rows('at-work'), ['thread:thread:calm']);
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:calm"]'))`), /Not read yet/);
    assert.equal(await read(`Boolean(p.querySelector('[data-row="thread:thread:calm"] [data-unknown]'))`), true);
  });

  await open('turns-waiting', 1000, 'light', { fail: threadPath('thread:turns'), after: 1 });
  await check('running: a thread T3 Code\'s list now says is waiting is listed under Needs you at once, though its last full answer says running', async () => {
    assert.deepEqual(await rows('at-work'), ['thread:thread:turns']);
    await page.evaluate('document.dispatchEvent(new Event("visibilitychange"))');
    await until(async () => (await rows('needs-you'))?.join() === 'thread:thread:turns', 'The waiting thread under Needs you');
    assert.equal(await read(`text(p.querySelector('[data-summary]'))`), 'One item is being built. One thread waits on you.');
    assert.deepEqual(await rows('at-work'), []);
    assert.match(await read(`text(p.querySelector('[data-row="thread:thread:turns"]'))`), /Implementer is waiting on you\. Last: Reading src\/desk\.ts/);
  });

  await open('done', 1000);
  await check('done: the state and when it closed, the Result as written, a lane per item with its spans, rounds and duration, and a legend', async () => {
    assert.equal(await read(`p.dataset.missionPage`), 'done');
    assert.equal(await read(`text(p.querySelector('[data-state]'))`), 'Done');
    assert.equal(await read(`text(p.querySelector('[data-closed]'))`), '2 of 2 merged. Closed 3h 0m ago, 7h 0m after the plan merged.');
    assert.equal(await read(`[...p.querySelectorAll('[data-section="result"] p')].map(text).join(' ')`), 'The Desk has an embedded mode under /embed and a host contract. All 7 success criteria are checked by a test.');
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-legend]')].map(l => l.dataset.legend)`), ['building', 'review', 'merging']);
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-lane]')].map(l => [l.dataset.lane, [...l.querySelectorAll('[data-span]')].map(s => s.dataset.span).join(' '), text(l.lastElementChild)])`), [
      ['atelier-d002', 'building review merging', '1 round · 50m'],
      ['atelier-d001', 'building review building review building review merging', '3 rounds · 3h 30m'],
      ['atelier-d003', 'building review building', 'dropped · 1 round · 3h 0m'],
    ]);
    // A dropped item's lane ends where it was dropped, 300 of the 420 minutes in, not at the close.
    const droppedEnd = await read(`(() => { const lane = p.querySelector('[data-lane="atelier-d003"]'); const track = lane.children[1].getBoundingClientRect(); const spans = [...lane.querySelectorAll('[data-span]')]; return (spans.at(-1).getBoundingClientRect().right - track.left) / track.width; })()`);
    assert(Math.abs(droppedEnd - 300 / 420) < 0.01, `the dropped item's lane ends at ${droppedEnd} of the lane`);
    // The lanes run from the plan's merge to the close: the first span starts 10 minutes in.
    const first = await read(`(() => { const lane = p.querySelector('[data-lane="atelier-d002"]'); const track = lane.children[1].getBoundingClientRect(); const span = lane.querySelector('[data-span]').getBoundingClientRect(); return (span.left - track.left) / track.width; })()`);
    assert(Math.abs(first - 10 / 420) < 0.01, `the first span starts at ${first} of the lane`);
    assert.match(await read('text(p)'), /The lanes are read from commit subjects/);
  });
  await check('done: merged items most review rounds first; a pull request the Desk gives opens on the forge, one it does not is "not known" and opens the item\'s page', async () => {
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-section="merged"] [data-row]')].map(r => text(r))`), ['#172Let a host theme the embedded Desk3 rounds · 3h 30m', 'not knownLay the Desk out for a panel1 round · 50m']);
    await calls();
    await click('[data-row="item:atelier-d001"]');
    await click('[data-row="item:atelier-d002"]');
    assert.deepEqual(await calls(), [['url', 'https://example.invalid/pull/172'], ['item', 'atelier-d002']]);
    // A finished mission reads no thread, and only its merged items' pages.
    const reads = [...new Set(await page.evaluate('missionReads'))];
    assert(!reads.some(r => r.startsWith('/api/thread')), `a done mission read its threads: ${reads.join(', ')}`);
    assert(reads.includes('/api/repos/atelier/items/atelier-d002'));
    assert.equal(await read(`p.querySelector('[data-section="needs-you"], [data-section="at-work"]')`), null);
  });
  await check('done: the lanes take the full width at 1,400 pixels', async () => {
    await open('done', 1400);
    const share = await read(`p.querySelector('[data-lane] > div').getBoundingClientRect().width / p.getBoundingClientRect().width`);
    assert(share > 0.6, `a lane's track is ${Math.round(share * 100)}% of the page`);
    const got = await layout().catch(() => null);
    assert.equal(got, null, 'the done page drew a running page\'s sections');
    await shot('done-1400');
  });
  for (const appearance of ['light', 'dark']) {
    await open('needs-you', 1100, appearance); await shot(`running-${appearance}`);
    await open('done', 1100, appearance); await shot(`done-${appearance}`);
  }
  console.log(`Screenshots in ${path.join(root, 'artifacts')}/mission-*.png`);
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
