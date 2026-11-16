// Checks every T3 hook host-adapter.js relies on against a running T3 build.
// Usage: node test-contract.mjs [--port 9333] [--thread <environmentId>/<threadId>] [--draft <draftId>]
// It shows a thread, the workspace panel, a new-thread screen (the first draft
// with a project, or --draft) and Settings in that T3 window, then restores the
// route. Exits non-zero naming each hook that is missing, or if
// discovery imported anything but the renderer's own modules (never a worker).
import fs from 'node:fs/promises';
import { connectPage } from './devtools.mjs';

const option = (name, fallback) => { const index = process.argv.indexOf(name); return index < 0 ? fallback : process.argv[index + 1]; };
const port = Number(option('--port', 9333));
const thread = option('--thread', null);
const draft = option('--draft', null);
const adapter = await fs.readFile(new URL('./host-adapter.js', import.meta.url), 'utf8');
const page = await connectPage(port);
const { probes, imported, declared } = await page.evaluate(`(async api => {
  ${adapter}
  const wait = async (check, ms = 10000) => { const end = Date.now() + ms; while (!check() && Date.now() < end) await new Promise(r => setTimeout(r, 100)); observeHostDom(); };
  const original = location.hash;
  const host = await createHostAdapter(api);
  try {
    if (${JSON.stringify(thread)}) location.hash = '/' + ${JSON.stringify(thread)}.split('/').map(encodeURIComponent).join('/');
    const context = host.context && await host.context().catch(error => { hostProbes['route:thread'] ??= { ok: false, detail: error.message }; return null; });
    if (!context) hostProbes['route:thread'] ??= { ok: false, detail: location.hash + ' is not a thread with a project; pass --thread <environmentId>/<threadId>' };
    await wait(() => document.querySelector(hostSelectors.conversation));
    readHostTheme(document.body);
    if (context) {
      // The sidebar views, the layer and the project home need these on a thread's screen.
      const shown = ['sidebarSearch', 'mainInset', 'threadItem', 'sidebarRow', 'breadcrumbProject'];
      await wait(() => shown.every(name => document.querySelector(hostSelectors[name])));
      // Missions read who started each thread from the thread shells.
      if (!host.threadList?.().length) hostProbes['state:thread-origin'] ??= { ok: false, detail: host.threadList ? 'no thread in any connected environment' : 'a thread module hook is missing' };
      const found = host.findThread?.(context.threadId);
      hostProbes['state:find-thread'] = found === context.environmentId ? { ok: true, detail: 'thread found in environment ' + found + ' among the connected environments' } : { ok: false, detail: host.findThread ? 'found ' + found + ' for a thread of ' + context.environmentId : 'module:environment-atoms is missing' };
    }
    if (context && host.panel) {
      await host.panel.show(context);
      observeHostDom();
      // A native tab only exists once a surface is open; open the first one T3 offers.
      if (!document.querySelector(hostSelectors.panelTab)) document.querySelector(hostSelectors.panelContent + ' button')?.click();
      await wait(() => document.querySelector(hostSelectors.panelTab));
    }
    // The project strip needs a new-thread screen of a project: T3's route, draft store and composer.
    const draftId = ${JSON.stringify(draft)} ?? host.draftIds?.()[0];
    if (draftId) {
      location.hash = '/draft/' + encodeURIComponent(draftId);
      await wait(() => ['composerOverlay', 'composerStack', 'composerEditor'].every(name => document.querySelector(hostSelectors[name])));
      if (!host.draftContext?.()) hostProbes['route:draft'] ??= { ok: false, detail: '#/draft/' + draftId + ' named no project' };
    } else hostProbes['route:draft'] = { ok: false, detail: host.draftIds ? 'T3 holds no new-thread draft with a project; open a new thread in a project, or pass --draft <id>' : 'module:composer-draft-store is missing' };
    location.hash = '/settings/general';
    await wait(() => document.querySelector(hostSelectors.settingsPage));
    const declared = [...document.querySelectorAll('script[type="module"][src], link[rel="modulepreload"][href]')].map(node => node.src || node.href);
    return { probes: hostProbes, imported: hostImportedModules, declared };
  } finally { location.hash = original; }
})({ signal: new AbortController().signal })`, 90000);
page.close();

// Worker chunks must never be imported into the window: their top-level code would run there.
const stray = imported.filter(url => !declared.includes(url) || /worker/i.test(url.split('/').pop()));
probes['discovery:imports'] = { ok: stray.length === 0, detail: stray.length ? `imported non-module or worker scripts: ${stray.join(', ')}` : `${imported.length} of ${declared.length} renderer modules imported: ${imported.map(url => url.split('/').pop()).join(', ')}` };
const broken = Object.entries(probes).filter(([, probe]) => probe.ok !== true);
for (const [name, probe] of Object.entries(probes)) console.log(`${probe.ok === true ? 'ok     ' : 'BROKEN '} ${name}: ${probe.detail}`);
if (broken.length) {
  console.error(`\n${broken.length} T3 hook(s) broken: ${broken.map(([name]) => name).join(', ')}`);
  process.exitCode = 1;
}
