// The extension's entry. Vite builds it into dist/custom.js, one classic script that defines
// T3Custom; the loader injects it and calls T3Custom.start(api) (t3-custom.mjs, buildBundle) and
// reports what it throws.
import { Extension } from "./core/extension";
import { startViews } from "./custom.js";
import type { ExtensionApi } from "./host/api";
import { createHostAdapter, hostImportedModules, hostProbes, useHostApi } from "./host/adapter";
import { createDeskPanel } from "./panel/panel";
import { createReferences } from "./references/references";
import { createSettingsSection } from "./settings/mount";

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
  // The DOM pass runs the parts' syncs in this order: the panel, the settings section, then the
  // views custom.js draws (the project strip, the project home, the sidebar and the layer).
  const panel = createDeskPanel(ext, () => settings.open());
  const settings = createSettingsSection(ext, {
    showDesk: () => void panel.show(),
    changed: () => references.queueScan(),
    reloadRepositories: () => void loadRepositories(),
  });
  const references = createReferences(ext, panel);
  const views = startViews(ext, { panel });

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

  // A covered window throttles timers: when it is shown again, every view reads afresh.
  ext.listen(document, "visibilitychange", () => {
    ext.scheduleDOM();
    if (document.hidden) return;
    views.refresh();
    panel.tellThread();
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
