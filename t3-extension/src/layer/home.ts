// Project home: the project's name in T3's thread breadcrumb opens the project home in the layer
// instead of starting a new thread, for a project with a Desk repository. The extension draws the
// page itself (src/home/ProjectHome.tsx).
import type { Extension } from "../core/extension";
import { expectHostDom, hostSelectors } from "../host/adapter";
import { layerTargets, type Layer } from "./layer";

export function createProjectHome(ext: Extension, layer: Layer): void {
  ext.onDOM(() => expectHostDom("breadcrumbProject", Boolean(ext.current) && !location.hash.includes("/settings"), "T3 showed a thread"));
  ext.listen(
    document,
    "click",
    (event) => {
      const target = event.target instanceof Element ? event.target.closest(hostSelectors.breadcrumbProject) : null;
      if (!target || !ext.settings.enabled) return;
      const repo = ext.repositories.mapped(ext.current ?? ext.host?.draftContext?.() ?? null);
      if (!repo) return;
      // Before T3's own handler, which would start a new thread.
      event.preventDefault();
      event.stopPropagation();
      layer.toggle(layerTargets.home(repo));
    },
    { capture: true },
  );
}
