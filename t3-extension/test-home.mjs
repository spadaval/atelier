// Checks the project home (src/home) against fixture repositories (check/home-fixtures.ts): which
// missions and items it puts under Now, Next and Later, what a mission folds open to and who it
// names as holding each item, the strip of the last seven days, what its rows open, a repository
// with nothing open, a read that fails, and its layout at 500, 728, 1,100 and 1,400 pixels. It
// builds check/home.tsx with the extension's own Vite config and draws it in a page that carries
// T3 Code's stylesheet, as last copied, in light and dark. Screenshots go to artifacts/home-*.png.
//
//   node test-home.mjs        # BROWSER=<chromium> to choose the browser
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
const temp = await fs.mkdtemp(path.join(os.tmpdir(), 'atelier-home-test-'));
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
  await build({ ...quiet, configFile: path.join(root, 'vite.config.ts'), build: { outDir: path.join(temp, 'page'), emptyOutDir: true, lib: { entry: path.join(root, 'check/home.tsx'), formats: ['iife'], name: 'HomeCheck', fileName: () => 'home.js' } } });
  const warn = console.warn;
  console.warn = (message, ...rest) => { if (!/while optimizing generated CSS/.test(message)) warn(message, ...rest); };
  try {
    await build({ ...quiet, configFile: false, plugins: [tailwindcss()], build: { outDir: path.join(temp, 'reference'), emptyOutDir: true, lib: { entry: path.join(root, 'check/reference.ts'), formats: ['es'], fileName: () => 'reference.js', cssFileName: 'reference' } } });
  } finally { console.warn = warn; }
  const script = await fs.readFile(path.join(temp, 'page/home.js'), 'utf8');
  const t3Css = await fs.readFile(path.join(temp, 'reference/reference.css'), 'utf8');
  server = http.createServer((request, response) => {
    const url = new URL(request.url, 'http://localhost');
    if (url.pathname === '/home.js') { response.setHeader('content-type', 'text/javascript'); response.end(script); return; }
    response.setHeader('content-type', 'text/html');
    response.end(`<!doctype html><html class="${url.searchParams.get('appearance') === 'dark' ? 'dark' : ''}"><head><meta charset="utf-8"><style>${t3Css} body { margin: 0; padding: 8px; background: var(--background); }</style></head><body><script src="/home.js"></script></body></html>`);
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = `http://127.0.0.1:${server.address().port}`;
  // The week's days are local days: the fixture's clock is in UTC, so the browser is too.
  const env = { ...process.env, TZ: 'UTC' }; delete env.ELECTRON_RUN_AS_NODE; delete env.NODE_OPTIONS;
  const browserLog = await fs.open(path.join(temp, 'browser.log'), 'w');
  browser = spawn(browserPath, ['--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking', '--lang=en-US', `--user-data-dir=${path.join(temp, 'profile')}`, `--remote-debugging-port=${debugPort}`, '--remote-debugging-address=127.0.0.1', '--window-size=1500,1200', `${base}/`], { env, stdio: ['ignore', browserLog.fd, browserLog.fd] });
  await browserLog.close();
  page = await until(() => connectPage(debugPort, base), 'Browser startup', 120000);
  await fs.mkdir(path.join(root, 'artifacts'), { recursive: true });

  // Opens a scenario at a width and waits until the page has drawn from its reads.
  // `fail` names a path the stand-in injector answers `after` times and then fails (check/home.tsx).
  const open = async (scenario, width = 1000, appearance = 'light', fail = null, after = 0) => {
    const url = `${base}/?${new URLSearchParams({ scenario, width, appearance, ...(fail ? { fail, after } : {}) })}`;
    await page.call('Page.navigate', { url });
    await until(() => page.evaluate(`location.href === ${JSON.stringify(url)} && document.readyState === 'complete' && !['loading', undefined].includes(document.querySelector('#home')?.shadowRoot?.querySelector('[data-project-home]')?.dataset.projectHome)`), `${scenario} drawn`);
    // The fixture answers at once; the reads that follow the missions' (each open mission's items) are in by then.
    await delay(300);
  };
  const shot = async name => {
    const shot = await page.call('Page.captureScreenshot', { format: 'png', captureBeyondViewport: false });
    await fs.writeFile(path.join(root, `artifacts/home-${name}.png`), Buffer.from(shot.data, 'base64'));
  };
  const read = expression => page.evaluate(`(() => { const p = document.querySelector('#home').shadowRoot.querySelector('[data-project-home]'); const text = el => el?.textContent.replace(/\\s+/g, ' ').trim() ?? null; return ${expression}; })()`);
  const calls = () => page.evaluate('homeCalls.splice(0)');
  const click = selector => read(`(p.querySelector(${JSON.stringify(selector)}).click(), true)`);
  const lanes = () => read(`[...p.querySelectorAll('[data-lane]')].map(lane => [lane.dataset.lane, [...lane.querySelectorAll('[data-mission-row]')].map(r => r.dataset.missionRow), [...lane.querySelectorAll('[data-loose] [data-item-row]')].map(r => r.dataset.itemRow)])`);

  await open('full');
  await check('Now, Next and Later: the begun mission, the agreed one with nothing started and the proposed plan; loose items started and open; a done mission in none', async () => {
    assert.equal(await read(`p.dataset.projectHome`), 'shown');
    assert.equal(await read(`text(p.querySelector('h1'))`), 'atelier');
    assert.deepEqual(await lanes(), [
      ['now', ['atelier-nm01'], ['atelier-l001']],
      ['next', ['atelier-xm01'], ['atelier-l004', 'atelier-l002', 'atelier-l005', 'atelier-l003']],
      ['later', ['atelier-lm01'], []],
    ]);
    assert.match(await read(`text(p.querySelector('[data-lane="now"]'))`), /^Now2missions in progress, and started work in no mission/);
    assert.match(await read(`text(p.querySelector('[data-mission-row="atelier-nm01"]'))`), /^Draw the project home nativelyatelier-nm011 need you1\/5/);
    // The open items in no mission count in their lane.
    assert.match(await read(`text(p.querySelector('[data-lane="next"]'))`), /^Next5agreed, nothing started, and open work/);
    // Each item in no mission shows its priority, as the Desk's page does; a mission's items do not.
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-loose] [data-item-row]')].map(r => [r.dataset.itemRow, r.querySelector('[data-priority]')?.dataset.priority, r.querySelectorAll('[data-priority] i.bg-border').length])`), [['atelier-l001', 'critical', 0], ['atelier-l004', 'critical', 0], ['atelier-l002', 'high', 0], ['atelier-l005', 'high', 0], ['atelier-l003', 'low', 2]]);
    assert.equal(await read(`p.querySelector('[data-items] [data-priority]')`), null);
    assert.deepEqual(await page.evaluate('homeErrors'), []);
  });
  await check('a mission folds open to its items, open work first, each with who holds it; Now\'s and, with nothing under Now, Next\'s start open, Later\'s folded', async () => {
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-mission-row]')].map(r => [r.dataset.missionRow, r.hasAttribute('data-open')])`), [['atelier-nm01', true], ['atelier-xm01', false], ['atelier-lm01', false]]);
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-items="atelier-nm01"] [data-item-row]')].map(r => r.dataset.itemRow)`), ['atelier-n002', 'atelier-n003', 'atelier-n004', 'atelier-n005', 'atelier-n001']);
    // The holder by the title T3 Code gives its thread; a reviewer T3 Code does not list by the end of its identity.
    assert.equal(await read(`text(p.querySelector('[data-item-row="atelier-n002"] [data-holder]'))`), 'Implement the fold');
    assert.equal(await read(`text(p.querySelector('[data-item-row="atelier-n003"] [data-holder]'))`), 'review: thread …ewer01');
    assert.match(await read(`text(p.querySelector('[data-item-row="atelier-n004"]'))`), /blocked by atelier-n002 \(building\)/);
    assert.match(await read(`text(p.querySelector('[data-item-row="atelier-n005"]'))`), /Needs youopen$/);
    assert.equal(await read(`p.querySelector('[data-item-row="atelier-n001"] [data-holder]')`), null, 'a done item names a holder');
    await click('[data-mission-row="atelier-xm01"] [data-action="fold"]');
    await until(() => read(`p.querySelectorAll('[data-items="atelier-xm01"] [data-item-row]').length === 2`), 'Next\'s mission folded open');
    await click('[data-mission-row="atelier-nm01"] [data-action="fold"]');
    assert.equal(await read(`p.querySelector('[data-items="atelier-nm01"]')`), null);
  });
  await check('the week: seven days, today last, each with the items started and merged and plans agreed, one entry per record a day', async () => {
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-day]')].map(d => [d.dataset.day, [...d.querySelectorAll('[data-entry]')].map(text)])`), [
      ['Wed Nov 11 2026', []],
      ['Thu Nov 12 2026', []],
      ['Fri Nov 13 2026', []],
      ['Sat Nov 14 2026', ['Plan agreed Agreed, not started']],
      ['Sun Nov 15 2026', []],
      ['Mon Nov 16 2026', ['Started Fold a mission open']],
      ['Tue Nov 17 2026', ['Started and merged Draw the lanes']],
    ]);
    assert.equal(await read(`text(p.querySelector('[data-day]:last-child > div'))`), 'Today1');
    assert.match(await read(`text(p.querySelector('[data-week]'))`), /^This week3items started/);
  });
  await check('a mission\'s title opens it, saying whether it is a proposed plan; an item, loose or in a mission, and an entry of the week open what they name', async () => {
    // The check above folded Now's mission; it opens again.
    await click('[data-mission-row="atelier-nm01"] [data-action="fold"]');
    await until(() => read(`Boolean(p.querySelector('[data-item-row="atelier-n002"]'))`), 'Now\'s mission folded open again');
    await calls();
    await click('[data-lane="now"] [data-mission-row="atelier-nm01"] [data-action="mission"]');
    await click('[data-lane="later"] [data-mission-row="atelier-lm01"] [data-action="mission"]');
    await click('[data-item-row="atelier-n002"]');
    await click('[data-loose] [data-item-row="atelier-l003"]');
    await click('[data-day="Sat Nov 14 2026"] [data-entry="atelier-xm01"]');
    await click('[data-day="Tue Nov 17 2026"] [data-entry="atelier-n001"]');
    assert.deepEqual(await calls(), [['mission', 'atelier-nm01', false], ['mission', 'atelier-lm01', true], ['item', 'atelier-n002'], ['item', 'atelier-l003'], ['mission', 'atelier-xm01', false], ['item', 'atelier-n001']]);
    await read(`(p.focus(), true)`);
    for (const type of ['keyDown', 'keyUp']) await page.call('Input.dispatchKeyEvent', { type, key: 'Escape', code: 'Escape', windowsVirtualKeyCode: 27 });
    assert.deepEqual(await calls(), [['close']]);
  });
  await check('the page reads the missions, the roadmap, the week and each open mission, through the gate\'s forms', async () => {
    assert.deepEqual([...new Set(await page.evaluate('homeReads'))].sort(), [
      '/api/history?repo=atelier&days=7',
      '/api/repos/atelier/missions',
      '/api/repos/atelier/missions/atelier-nm01',
      '/api/repos/atelier/missions/atelier-xm01',
      '/api/roadmap?repo=atelier',
    ]);
  });

  const layout = () => read(`(() => {
    const box = el => el.getBoundingClientRect();
    const days = [...p.querySelectorAll('[data-day]')].map(d => Math.round(box(d).top));
    return { overflow: p.scrollWidth - p.clientWidth, perRow: days.filter(top => top === days[0]).length, bar: getComputedStyle(p.querySelector('[data-mission-row] [role="img"]').parentElement).display !== 'none', holder: getComputedStyle(p.querySelector('[data-holder]')).display !== 'none' };
  })()`);
  await check('layout: no sideways scroll from 500 to 1,400 pixels; the week in 7 columns from 1,000, 4 below, 2 below 560; the bar and holders dropped below 560', async () => {
    for (const [width, expected] of [[500, { perRow: 2, bar: false, holder: false }], [728, { perRow: 4, bar: true, holder: true }], [1100, { perRow: 7, bar: true, holder: true }], [1400, { perRow: 7, bar: true, holder: true }]]) {
      await open('full', width);
      const got = await layout();
      assert.equal(got.overflow, 0, `${width}px: the page scrolls sideways by ${got.overflow}px`);
      assert.deepEqual({ perRow: got.perRow, bar: got.bar, holder: got.holder }, expected, `${width}px`);
      await shot(`full-${width}`);
    }
  });

  await open('empty');
  await check('a repository with nothing open: each lane says so, and the week is seven empty days', async () => {
    assert.equal(await read(`p.dataset.projectHome`), 'empty');
    assert.deepEqual(await lanes(), [['now', [], []], ['next', [], []], ['later', [], []]]);
    assert.deepEqual(await read(`[...p.querySelectorAll('[data-lane] p')].map(text)`), ['Nothing here.', 'Nothing here.', 'Nothing here.']);
    assert.equal(await read(`p.querySelectorAll('[data-day]').length`), 7);
    assert.equal(await read(`p.querySelectorAll('[data-entry]').length`), 0);
    await shot('empty');
  });

  await open('full', 1000, 'light', '/api/roadmap?repo=atelier');
  await check('the items in no mission cannot be read: the missions still show, with a note naming the failure', async () => {
    assert.deepEqual(await lanes(), [['now', ['atelier-nm01'], []], ['next', ['atelier-xm01'], []], ['later', ['atelier-lm01'], []]]);
    assert.match(await read(`text(p.querySelector('[data-note]'))`), /items in no mission could not be read.*HTTP 502/);
  });
  // A read that answered and then fails keeps what it gave, said to be as of then, never silently.
  for (const [failing, label, where, kept] of [
    ['/api/roadmap?repo=atelier', 'the items in no mission', `p`, `p.querySelectorAll('[data-loose] [data-item-row]').length === 3`],
    ['/api/history?repo=atelier&days=7', 'the week', `p`, `p.querySelectorAll('[data-entry]').length === 3`],
    ['/api/repos/atelier/missions/atelier-nm01', 'an open mission\'s items', `p.querySelector('[data-mission-row="atelier-nm01"]')`, `p.querySelectorAll('[data-items="atelier-nm01"] [data-item-row]').length === 5`],
  ]) {
    await open('full', 1000, 'light', failing, 1);
    await check(`${label} answer and then fail: they stay, with a note saying they are as of the last read`, async () => {
      assert.equal(await read(`${where}.querySelector('[data-note]')`), null);
      await page.evaluate('document.dispatchEvent(new Event("visibilitychange"))');
      await until(() => read(`Boolean(${where}.querySelector('[data-note]'))`), `The failed read of ${failing} noted`);
      assert.match(await read(`text(${where}.querySelector('[data-note]'))`), /could not be read again, so .* as of .*HTTP 502/);
      assert.equal(await read(kept), true, 'what the read gave before is gone');
    });
  }
  await open('full', 1000, 'light', '/api/repos/atelier/missions');
  await check('the missions cannot be read: the page says so, naming the Desk and the failure, and draws no lanes', async () => {
    assert.equal(await read(`p.dataset.projectHome`), 'error');
    assert.match(await read(`text(p.querySelector('[data-home-error]'))`), /^Could not read the missions of atelier from http:\/\/desk\.invalid: .*HTTP 502/);
    assert.equal(await read(`p.querySelector('[data-lane]')`), null);
  });

  for (const appearance of ['light', 'dark']) { await open('full', 1100, appearance); await shot(`full-${appearance}`); }
  console.log(`Screenshots in ${path.join(root, 'artifacts')}/home-*.png`);
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
