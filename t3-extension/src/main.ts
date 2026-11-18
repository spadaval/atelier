// The extension's entry. Vite builds it into dist/custom.js, one classic script that defines
// T3Custom; the loader injects it and calls T3Custom.start(api) (t3-custom.mjs, buildBundle) and
// reports what it throws.
import { Extension } from "./core/extension";
import type { ExtensionApi } from "./host/api";
import { createHostAdapter, hostDomExpected, hostImportedModules, hostProbes, useHostApi } from "./host/adapter";
import { createProjectHome } from "./layer/home";
import { createLayer, layerTargets } from "./layer/layer";
import { createDeskPanel } from "./panel/panel";
import { createReferences } from "./references/references";
import { createSettingsSection } from "./settings/mount";
import { createSidebar } from "./sidebar/sidebar";
import { createProjectStrip } from "./strip/strip";
import { viewMissing, views, type ViewName } from "./views";

// test-contract.mjs runs the built adapter against a running T3 Code, without starting the extension.
export * as adapter from "./host/adapter";

/** What the loader leaves on the page for checks and the console (t3-custom.mjs, buildBundle). */
declare global {
  var __t3Custom: { diagnostics?: () => Record<string, unknown>; openMission?: (repo: string, id: string, desk?: string) => void; error: string | null } | undefined;
}

export async function start(api: ExtensionApi): Promise<void> {
  useHostApi(api);
  const ext = new Extension(api);
  const { repositories } = ext;
  // The DOM pass runs the parts' syncs in this order: the panel, the settings section, the project
  // strip, the project home, the sidebar with its missions, then the layer, which marks their rows.
  // The panel opens the mission page in the layer, which is made after it.
  const panel = createDeskPanel(ext, { openSettings: () => settings.open(), openMission: (desk, repo, id) => layer.open(layerTargets.mission(desk, repo, id)) });
  const settings = createSettingsSection(ext, {
    showDesk: () => void panel.show(),
    changed: () => references.queueScan(),
    reloadRepositories: () => void loadRepositories(),
  });
  const references = createReferences(ext, panel);
  // Views outside the panel each need only their own hooks; a missing one turns off that view alone (views.ts).
  const layer = createLayer(ext, (threadId) => panel.openThread(threadId));
  createProjectStrip(ext, layer);
  createProjectHome(ext, layer);
  const sidebar = createSidebar(ext, layer);
  ext.onDOM(layer.sync);
  ext.diagnostics.views = () => ({ views: Object.fromEntries((Object.keys(views) as ViewName[]).map((name) => [name, viewMissing(name)])) });
  // A DOM hook expected but not yet found is checked again each second until it is found or reported missing.
  const expectTimer = setInterval(() => {
    if (Object.keys(hostDomExpected).length) ext.scheduleDOM();
  }, 1000);
  ext.onDispose(() => clearInterval(expectTimer));

  async function loadRepositories() {
    await repositories.load();
    if (api.signal.aborted) return;
    references.buildPattern();
    ext.scheduleDOM();
    references.queueScan();
  }
  const state = globalThis.__t3Custom;
  if (state) {
    state.diagnostics = () =>
      Object.assign(
        {
          probes: hostProbes,
          modules: hostImportedModules,
          context: ext.current,
          repositories: repositories.list,
          mappedRepository: repositories.mapped(ext.current)?.repo ?? null,
          selectedRepository: repositories.selected(ext.current)?.repo ?? null,
          referenceOverride: repositories.override,
        },
        ...Object.values(ext.diagnostics).map((part) => part()),
      );
  }

  // A covered window throttles timers: when it is shown again, every view reads afresh. The strip's
  // and the mission page's reads follow visibility themselves (src/react/desk.ts).
  ext.listen(document, "visibilitychange", () => {
    ext.scheduleDOM();
    if (document.hidden) return;
    sidebar.read();
    panel.drawThread();
  });
  ext.listen(window, "resize", () => ext.scheduleDOM());
  ext.watchRoute();
  // The Customizations section and references do not wait for T3's hooks.
  const repositoriesLoaded = loadRepositories();
  ext.scheduleDOM();
  try {
    ext.host = await createHostAdapter(api);
    panel.watchPanel();
    await ext.updateContext();
    if (ext.hostError) throw ext.hostError;
  } catch (error) {
    if (!api.signal.aborted) {
      ext.hostError = error instanceof Error ? error : new Error(String(error));
      ext.displayError(error);
      throw error;
    }
  }
  await repositoriesLoaded;
  ext.scheduleDOM();
  references.queueScan();
  api.log("Atelier extension ready", { desks: ext.deskUrls, repositories: repositories.list.map((repo) => repo.repo) });
}
