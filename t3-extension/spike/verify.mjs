// Injects dist/custom.js as one script into a page that carries T3 Code's own stylesheet, and
// compares each element drawn in a shadow root with the same element drawn by T3's stylesheet.
//   pnpm build && node verify.mjs
import fs from 'node:fs/promises';
import { chromium } from '@playwright/test';

const script = await fs.readFile('dist/custom.js', 'utf8');
const hostCss = await fs.readFile('dist-reference/reference.css', 'utf8');
const props = ['color', 'backgroundColor', 'borderTopColor', 'borderTopWidth', 'borderTopStyle', 'borderBottomWidth', 'borderRadius', 'fontSize', 'fontFamily', 'fontWeight', 'lineHeight', 'height', 'paddingLeft', 'boxShadow'];
await fs.mkdir('shots', { recursive: true });
const browser = await chromium.launch(process.env.BROWSER ? { executablePath: process.env.BROWSER } : {});
let failed = 0;
for (const appearance of ['light', 'dark']) {
  for (const registerProperties of [false, true]) {
    const page = await browser.newPage({ viewport: { width: 760, height: 260 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.setContent(`<!doctype html><html class="${appearance === 'dark' ? 'dark' : ''}"><head><style>${hostCss}</style></head><body></body></html>`);
    await page.evaluate(options => { globalThis.__spike = options; }, { shadow: true, reference: true, registerProperties });
    await page.evaluate(script);
    await page.waitForSelector('#reference [data-probe="kbd"]');
    for (const host of ['#reference', '#extension']) await page.locator(`${host} [data-probe="trigger"]`).hover().then(() => page.waitForTimeout(250));
    await page.locator('#extension [data-probe="trigger"]').hover();
    await page.waitForTimeout(250);
    const differences = await page.evaluate(props => {
      const out = [];
      const shadow = document.querySelector('#extension').shadowRoot;
      for (const reference of document.querySelectorAll('#reference [data-probe]')) {
        const probe = reference.dataset.probe;
        const ours = shadow.querySelector(`[data-probe="${probe}"]`);
        if (!ours) { out.push(`${probe}: not drawn in the shadow root`); continue; }
        const a = getComputedStyle(reference), b = getComputedStyle(ours);
        for (const prop of props) if (a[prop] !== b[prop]) out.push(`${probe}.${prop}: T3 ${a[prop]} | ours ${b[prop]}`);
      }
      const tip = shadow.querySelector('[data-probe="tooltip"]');
      if (!tip) out.push('tooltip: not drawn inside the shadow root');
      else {
        // The reference tooltip closed when the pointer left it, so check ours against the theme.
        const style = getComputedStyle(tip), root = getComputedStyle(document.documentElement);
        const want = document.body.appendChild(document.createElement('i'));
        want.style.cssText = 'background: var(--popover); border: 1px solid var(--border)';
        const expected = getComputedStyle(want);
        if (style.backgroundColor !== expected.backgroundColor) out.push(`tooltip.backgroundColor: theme ${expected.backgroundColor} | ours ${style.backgroundColor}`);
        if (style.borderTopWidth !== '1px' || style.borderTopStyle !== 'solid') out.push(`tooltip.border: ours ${style.borderTopWidth} ${style.borderTopStyle}`);
        if (style.boxShadow === 'none') out.push('tooltip.boxShadow: none');
        want.remove(); void root;
      }
      return out;
    }, props);
    const label = `${appearance}, @property ${registerProperties ? 'registered on the document' : 'left in the shadow root'}`;
    await page.screenshot({ path: `shots/${appearance}-${registerProperties ? 'registered' : 'unregistered'}.png` });
    console.log(`${differences.length + errors.length ? 'DIFFERS' : 'MATCHES'}: ${label}`);
    for (const line of [...errors, ...differences]) console.log(`  ${line}`);
    // Only the registered case is the one the extension would ship.
    if (registerProperties) failed += differences.length + errors.length;
    await page.close();
  }
}
// Without T3's stylesheet on the page, does Tailwind's @property need registering on the document?
for (const registerProperties of [false, true]) {
  const page = await browser.newPage();
  await page.setContent('<!doctype html><html><body></body></html>');
  await page.evaluate(options => { globalThis.__spike = options; }, { shadow: true, registerProperties });
  await page.evaluate(script);
  await page.waitForFunction(() => document.querySelector('#extension')?.shadowRoot.querySelector('[data-probe="trigger"]'));
  const style = await page.evaluate(() => { const s = getComputedStyle(document.querySelector('#extension').shadowRoot.querySelector('[data-probe="trigger"]')); return `border ${s.borderTopWidth} ${s.borderTopStyle}, shadow ${s.boxShadow}`; });
  console.log(`On a page without T3's stylesheet, @property ${registerProperties ? 'registered' : 'not registered'}: ${style}`);
  await page.close();
}
await browser.close();
process.exit(failed ? 1 : 0);
