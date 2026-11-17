// Checks that T3 Code's components, as the extension draws them, look as T3 draws them. It builds
// check/page.tsx, which draws check/Probe.tsx (every copied component in each variant), with the extension's own Vite config,
// and T3's stylesheet as last copied (src/t3/index.css) as T3 builds it. A page carrying T3's
// stylesheet draws the tree twice: in the page itself, as T3 would, and through mountReact in a
// shadow root, as the extension does. Every element's computed style must match, in light and in
// dark, with the font T3's Appearance settings put on <html>. A tooltip opened by hovering in the
// shadow root must be drawn inside it and styled.
//
//   node test-components.mjs        # BROWSER=<chromium> to choose the browser
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
const temp = await fs.mkdtemp(path.join(os.tmpdir(), 'atelier-components-test-'));
const debugPort = 19223;
const browserPath = process.env.BROWSER ?? ({ win32: 'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe', darwin: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' }[process.platform] ?? '/usr/bin/google-chrome');
// What a component's look is made of. Each is compared on the element and on its ::before and
// ::after, which T3's buttons and tooltips draw their edges with.
const properties = [
  'display', 'position', 'width', 'height', 'paddingTop', 'paddingRight', 'paddingBottom', 'paddingLeft', 'gap',
  'color', 'backgroundColor', 'backgroundClip', 'backgroundImage', 'opacity', 'boxShadow', 'transform', 'scale',
  'borderTopColor', 'borderRightColor', 'borderBottomColor', 'borderLeftColor', 'borderTopWidth', 'borderRightWidth', 'borderBottomWidth', 'borderLeftWidth',
  'borderTopStyle', 'borderBottomStyle', 'borderTopLeftRadius', 'borderBottomRightRadius',
  'outlineColor', 'outlineStyle', 'outlineWidth', 'outlineOffset',
  'fontFamily', 'fontSize', 'fontWeight', 'lineHeight', 'letterSpacing', 'textAlign', 'textDecorationLine', 'whiteSpace', 'cursor',
  'fill', 'stroke', 'strokeWidth', 'backdropFilter', 'content',
];
// T3's Appearance settings put a chosen font on <html> (apps/web/src/appearanceFonts.ts).
const appearanceFont = '"DejaVu Serif", Georgia, serif';

let browser, server;
try {
  // The extension's own build of the probe, so its stylesheet is built exactly as dist/custom.js's.
  const quiet = { logLevel: 'warn', root };
  await build({ ...quiet, configFile: path.join(root, 'vite.config.ts'), build: { outDir: path.join(temp, 'probe'), emptyOutDir: true, lib: { entry: path.join(root, 'check/page.tsx'), formats: ['iife'], name: 'Probe', fileName: () => 'probe.js' } } });
  // T3's stylesheet, optimised as T3's own build does it. Lightning CSS warns about T3's own
  // ::highlight() rules there, as it does in T3's build; those are counted, not printed.
  const warn = console.warn; let t3Warnings = 0;
  console.warn = (message, ...rest) => { if (/while optimizing generated CSS/.test(message)) t3Warnings++; else warn(message, ...rest); };
  try {
    await build({ ...quiet, configFile: false, plugins: [tailwindcss()], build: { outDir: path.join(temp, 'reference'), emptyOutDir: true, lib: { entry: path.join(root, 'check/reference.ts'), formats: ['es'], fileName: () => 'reference.js', cssFileName: 'reference' } } });
  } finally { console.warn = warn; }
  console.log(`Built the probe and T3 Code's stylesheet as of ${(await fs.readFile(path.join(root, 'src/t3/SOURCE'), 'utf8')).split(' ')[2].slice(0, 12)}${t3Warnings ? ' (Lightning CSS warned on T3\'s own rules, as in T3\'s build)' : ''}`);
  const script = await fs.readFile(path.join(temp, 'probe/probe.js'), 'utf8');
  const t3Css = await fs.readFile(path.join(temp, 'reference/reference.css'), 'utf8');

  server = http.createServer((request, response) => {
    const url = new URL(request.url, 'http://localhost');
    if (url.pathname === '/probe.js') { response.setHeader('content-type', 'text/javascript'); response.end(script); return; }
    const dark = url.searchParams.get('appearance') === 'dark';
    response.setHeader('content-type', 'text/html');
    response.end(`<!doctype html><html class="${dark ? 'dark' : ''}" style='--font-sans: ${appearanceFont}'><head><meta charset="utf-8"><style>${t3Css}</style></head><body><script src="/probe.js"></script></body></html>`);
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = `http://127.0.0.1:${server.address().port}`;

  const env = { ...process.env }; delete env.ELECTRON_RUN_AS_NODE; delete env.NODE_OPTIONS;
  const browserLog = await fs.open(path.join(temp, 'browser.log'), 'w');
  browser = spawn(browserPath, ['--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking', `--user-data-dir=${path.join(temp, 'profile')}`, `--remote-debugging-port=${debugPort}`, '--remote-debugging-address=127.0.0.1', '--window-size=1400,1400', `${base}/?appearance=light`], { env, stdio: ['ignore', browserLog.fd, browserLog.fd] });
  await browserLog.close();
  const page = await until(() => connectPage(debugPort, base), 'Browser startup', 120000);

  let failures = 0;
  for (const appearance of ['light', 'dark']) {
    await page.evaluate(`location.href = ${JSON.stringify(`${base}/?appearance=${appearance}`)}`).catch(() => {});
    await until(() => page.evaluate(`location.search === "?appearance=${appearance}" && document.readyState === "complete" && Boolean(document.querySelector('#extension')?.shadowRoot?.querySelector('[data-probe="kbd"]')) && Boolean(document.querySelector('#reference [data-probe="kbd"]'))`), `The probe drawn in ${appearance}`);
    // Tooltips open with a short scale and fade; compare them once it has run. One that is not in
    // the shadow root by then is named by the comparison.
    await delay(500);
    const differences = await page.evaluate(`(${compare})(${JSON.stringify(properties)})`);
    const errors = await page.evaluate('globalThis.probeErrors');
    assert(differences.compared > 50, `only ${differences.compared} elements were compared in ${appearance}: the probe did not draw`);

    // A tooltip opened by hovering its trigger in the shadow root.
    const trigger = await page.evaluate(`(() => { const rect = document.querySelector('#extension').shadowRoot.querySelector('[data-probe="tooltip-hover-trigger"]').getBoundingClientRect(); return { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 }; })()`);
    await page.call('Input.dispatchMouseEvent', { type: 'mouseMoved', x: trigger.x, y: trigger.y });
    const tooltip = await until(() => page.evaluate(`(${hovered})()`), `The hovered tooltip in ${appearance}`, 5000).catch(error => ({ problems: [error.message] }));
    await page.call('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });

    const problems = [...errors.map(error => `page error: ${error}`), ...differences.lines, ...tooltip.problems];
    console.log(problems.length
      ? `FAIL: in ${appearance}, ${problems.length} differences between ${differences.compared} elements drawn in a shadow root and T3's stylesheet:`
      : `PASS: ${differences.compared} elements drawn in a shadow root match T3's stylesheet in ${appearance}, and a hovered tooltip opens inside the shadow root, styled`);
    for (const line of problems) console.log(`  ${line}`);
    failures += problems.length;
  }

  // useDeskRead: data, then the read again on its interval, then a failure shown as an error with
  // the last data kept; a Desk that never answers shows an error and no data, never empty data.
  const shown = path => page.evaluate(`(() => { const output = document.querySelector('#desk-read').shadowRoot.querySelector('[data-path="${path}"]'); return { ...output.dataset }; })()`);
  const motion = await until(async () => { const read = await shown('/api/motion'); return read.error ? read : null; }, 'useDeskRead failing after two reads');
  const inbox = await shown('/api/inbox');
  try {
    assert.deepEqual(motion, { path: '/api/motion', loading: 'false', reads: '2', error: 'GET /api/motion: HTTP 503: down' });
    assert.deepEqual(inbox, { path: '/api/inbox', loading: 'false', reads: 'none', error: 'GET /api/inbox: HTTP 503: down' });
    console.log('PASS: useDeskRead reads again on its interval and gives a failed read as an error, keeping the last data');
  } catch (error) { console.log(`FAIL: useDeskRead: ${error.message}`); failures++; }
  process.exitCode = failures ? 1 : 0;
} finally {
  browser?.kill();
  server?.close();
  await fs.rm(temp, { recursive: true, force: true }).catch(() => {});
}

async function until(check, label, timeout = 20000) {
  const end = Date.now() + timeout; let lastError;
  while (Date.now() < end) {
    try { const result = await check(); if (result) return result; } catch (error) { lastError = error; }
    await delay(100);
  }
  throw new Error(`${label} timed out${lastError ? ': ' + lastError.message : ''}`);
}

// In the page: pairs each [data-probe] drawn by T3's stylesheet with the one in the shadow root,
// walks both subtrees together and names every element and property whose computed style differs.
function compare(properties) {
  const shadow = document.querySelector('#extension').shadowRoot;
  const lines = []; let compared = 0;
  const name = (element) => `${element.localName}${element.dataset.slot ? `[data-slot=${element.dataset.slot}]` : ''}`;
  const walk = (theirs, ours, label) => {
    compared++;
    for (const pseudo of [null, '::before', '::after']) {
      const a = getComputedStyle(theirs, pseudo), b = getComputedStyle(ours, pseudo);
      if (pseudo && a.content === 'none' && b.content === 'none') continue;
      for (const property of properties) {
        if (a[property] !== b[property]) lines.push(`${label}${pseudo ?? ''} ${property}: T3 draws ${a[property]}, the extension ${b[property]}`);
      }
    }
    if (theirs.children.length !== ours.children.length) { lines.push(`${label}: T3 draws ${theirs.children.length} children, the extension ${ours.children.length}`); return; }
    [...theirs.children].forEach((child, index) => walk(child, ours.children[index], `${label} > ${name(child)}`));
  };
  for (const theirs of document.querySelectorAll('[data-probe]')) {
    const probe = theirs.dataset.probe;
    if (theirs.parentElement?.closest('[data-probe]')) continue;
    const ours = shadow.querySelector(`[data-probe="${probe}"]`);
    if (!ours) { lines.push(`${probe}: not drawn in the shadow root`); continue; }
    walk(theirs, ours, `${probe} (${name(theirs)})`);
  }
  return { compared, lines };
}

// In the page: whether the hovered tooltip is open inside the shadow root and styled as T3's.
function hovered() {
  const shadow = document.querySelector('#extension').shadowRoot;
  const popup = shadow.querySelector('[data-probe="tooltip-hover"]');
  const outside = document.querySelector('[data-probe="tooltip-hover"]');
  if (!popup && !outside) return null;
  const problems = [];
  if (outside) problems.push(`tooltip-hover: drawn outside the shadow root, in ${outside.parentElement?.localName ?? 'nothing'}, where the extension's stylesheet does not reach`);
  if (popup) {
    const ours = getComputedStyle(popup);
    const theirs = getComputedStyle(document.querySelector('body > * [data-probe="tooltip-default"]'));
    for (const property of ['backgroundColor', 'borderTopStyle', 'borderTopColor', 'borderTopLeftRadius', 'boxShadow', 'color', 'fontSize']) {
      if (ours[property] !== theirs[property]) problems.push(`tooltip-hover ${property}: T3's open tooltip has ${theirs[property]}, the hovered one ${ours[property]}`);
    }
  }
  return { problems };
}
