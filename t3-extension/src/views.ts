// Which of the extension's views are on, by the T3 hooks each needs. A missing hook turns off only
// the views that need it, and Customizations names it.
import { hostProbes, type HostAdapter, type ProbeName } from "./host/adapter";

// The Desk panel needs T3's panel store, project context and a slot to mount in;
// references need only the conversation DOM and otherwise open Desk externally.
export const deskHooks: ProbeName[] = ["module:workspace-panel-store", "module:atom-registry", "module:thread-snapshot-atom", "dom:panel", "dom:panelContent"];
export const deskAvailable = (host: HostAdapter | null): boolean => Boolean(host?.panel && host?.context) && !deskHooks.some((name) => hostProbes[name]?.ok === false);

// Views outside the panel: the Inbox row in T3's sidebar, missions in T3's thread list, the layer
// over T3's main area, the project strip under the new-thread composer and the project home from
// the breadcrumb. Each needs only its own hooks; a missing one turns off that view alone.
export type ViewName = "sidebar" | "missions" | "layer" | "missionThreads" | "strip" | "composer" | "home";
export const views: Record<ViewName, { label: string; hooks: ProbeName[]; off?: string }> = {
  sidebar: { label: "Inbox row in the sidebar", hooks: ["dom:sidebarSearch"] },
  missions: { label: "Missions in T3's thread list", hooks: ["layout:threadList", "dom:settleButton"], off: "T3's list is left as T3 drew it" },
  layer: { label: "Desk layer over T3's main area", hooks: ["dom:mainInset"], off: "its pages open in the browser" },
  missionThreads: { label: "Mission threads read from T3", hooks: ["module:atom-registry", "module:thread-snapshot-atom", "module:environment-atoms", "state:thread-origin"] },
  strip: { label: "Project strip on the new-thread screen", hooks: ["module:composer-draft-store", "route:draft", "dom:composerOverlay", "dom:composerStack"] },
  composer: { label: "Filling T3's composer from the strip", hooks: ["dom:composerEditor"] },
  home: { label: "Project home from the breadcrumb", hooks: ["dom:breadcrumbProject"] },
};
export const viewMissing = (name: ViewName): ProbeName[] => views[name].hooks.filter((hook) => hostProbes[hook]?.ok === false);
export const viewOn = (name: ViewName): boolean => viewMissing(name).length === 0;
