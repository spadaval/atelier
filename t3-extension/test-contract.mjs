// Checks every T3 hook host-adapter.js relies on against a running T3 build.
// Usage: node test-contract.mjs [--port 9333] [--thread <environmentId>/<threadId>]
// It shows a thread, the workspace panel and Settings in that T3 window, then
// restores the route. Exits non-zero naming each hook that is missing.
import fs from 'node:fs/promises';
import { connectPage } from './devtools.mjs';

const option = (name, fallback) => { const index = process.argv.indexOf(name); return index < 0 ? fallback : process.argv[index + 1]; };
const port = Number(option('--port', 9333));
const thread = option('--thread', null);
const adapter = await fs.readFile(new URL('./host-adapter.js', import.meta.url), 'utf8');
const page = await connectPage(port);
const probes = await page.evaluate(`(async api => {
  ${adapter}
  const wait = async (check, ms = 10000) => { const end = Date.now() + ms; while (!check() && Date.now() < end) await new Promise(r => setTimeout(r, 100)); observeHostDom(); };
  const original = location.hash;
  const host = await createHostAdapter(api);
  try {
    if (${JSON.stringify(thread)}) location.hash = '/' + ${JSON.stringify(thread)}.split('/').map(encodeURIComponent).join('/');
    const context = host.context && await host.context().catch(error => { hostProbes['route:thread'] ??= { ok: false, detail: error.message }; return null; });
    if (!context) hostProbes['route:thread'] ??= { ok: false, detail: location.hash + ' is not a thread with a project; pass --thread <environmentId>/<threadId>' };
    await wait(() => document.querySelector(hostSelectors.conversation));
    if (context && host.panel) {
      host.panel.show(context);
      await wait(() => document.querySelector(hostSelectors.panelContent));
      // A native tab only exists once a surface is open; open the first one T3 offers.
      if (!document.querySelector(hostSelectors.panelTab)) document.querySelector(hostSelectors.panelContent + ' button')?.click();
      await wait(() => document.querySelector(hostSelectors.panelTab));
    }
    location.hash = '/settings/general';
    await wait(() => document.querySelector(hostSelectors.settingsPage));
    return hostProbes;
  } finally { location.hash = original; }
})({ signal: new AbortController().signal })`, 90000);
page.close();

const broken = Object.entries(probes).filter(([, probe]) => probe.ok !== true);
for (const [name, probe] of Object.entries(probes)) console.log(`${probe.ok === true ? 'ok     ' : 'BROKEN '} ${name}: ${probe.detail}`);
if (broken.length) {
  console.error(`\n${broken.length} T3 hook(s) broken: ${broken.map(([name]) => name).join(', ')}`);
  process.exitCode = 1;
}
