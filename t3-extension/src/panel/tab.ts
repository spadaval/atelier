// The Desk tab in T3's panel tab strip, its controls, and the launch button in T3's layout
// controls. They sit inside DOM T3 owns, so they are DOM nodes placed among T3's own, not React.
import { element, icons } from "../core/dom";
import type { Extension } from "../core/extension";
import { hostSelectors } from "../host/adapter";

export type TabActions = { toggle(): void; close(): void; refresh(): void; settings(): void; launch(): void };
/** How a DOM pass leaves the tab: shown, selected, and what it is labelled and links to. */
export type TabState = { allowed: boolean; visible: boolean; label: string; title: string; external: string | null; repository: boolean };

export type DeskTab = { sync(tabbar: Element | null | undefined, state: TabState): void };

export function createDeskTab(ext: Extension, actions: TabActions): DeskTab {
  // The Desk tab and its controls copy T3's own panel tab and icon-button metrics and colours.
  const style = element("style", {
    text: `
  [data-t3-desk-tab] { display: flex; align-items: center; gap: 2px; height: 24px; max-width: 9rem; flex-shrink: 0; padding: 0 8px 0 6px; border-radius: 6px; font-size: 12px; color: var(--muted-foreground); -webkit-app-region: no-drag; }
  [data-t3-desk-tab][data-selected] { background: var(--accent); color: var(--foreground); }
  [data-t3-desk-tab] > [data-t3-desk-label] { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; background: none; border: 0; padding: 0; color: inherit; font: inherit; cursor: pointer; }
  [data-t3-desk-icon], [data-t3-desk-launch] { display: inline-flex; align-items: center; justify-content: center; gap: 4px; flex-shrink: 0; min-width: 24px; height: 24px; padding: 0 4px; border: 0; border-radius: 6px; background: transparent; color: var(--muted-foreground); font: inherit; font-size: 12px; cursor: pointer; text-decoration: none; -webkit-app-region: no-drag; }
  [data-t3-desk-tab] > [data-t3-desk-icon] { min-width: 16px; height: 16px; padding: 0; }
  [data-t3-desk-launch] { width: 28px; height: 28px; padding: 0; color: var(--foreground); }
  [data-t3-desk-icon]:hover, [data-t3-desk-launch]:hover { background: var(--accent); color: var(--foreground); }
  [data-t3-desk-controls] { display: flex; align-items: center; gap: 2px; flex-shrink: 0; }
  :is([data-t3-desk-tab], [data-t3-desk-controls]) :focus-visible, [data-t3-desk-launch]:focus-visible { outline: 2px solid var(--ring); outline-offset: -2px; }
  [data-t3-desk-tabbar] ${hostSelectors.panelTab} { background: transparent !important; color: var(--muted-foreground) !important; }
`,
  });
  document.head.append(style);

  // The tab sits in T3's tab strip like a native one: close icon, then the Desk's page title.
  // Its controls (Refresh, Open externally, Customizations) follow it while the Desk is shown.
  const close = element("button", { "data-t3-desk-icon": "", "aria-label": "Close Desk", title: "Close Desk", html: icons.close });
  const label = element("button", { "data-t3-desk-label": "", "aria-pressed": "false", text: "Desk" });
  const tab = element("div", { "data-t3-desk-tab": "" }, close, label);
  const refresh = element("button", { "data-t3-desk-icon": "", "aria-label": "Refresh Desk", title: "Refresh Desk", html: icons.refresh });
  const external = element("a", { "data-t3-desk-icon": "", "data-t3-desk-external": "", "aria-label": "Open Desk externally", title: "Open externally", target: "_blank", rel: "noopener noreferrer", html: icons.external });
  const settings = element("button", { "data-t3-desk-icon": "", "aria-label": "Atelier customizations", title: "Customizations", html: icons.settings });
  const controls = element("div", { "data-t3-desk-controls": "", hidden: "" }, refresh, external, settings);
  const launch = element("button", { "data-t3-desk-launch": "", "aria-label": "Open Atelier Desk", title: "Atelier Desk", html: icons.desk });
  ext.listen(label, "click", actions.toggle);
  ext.listen(close, "click", actions.close);
  ext.listen(refresh, "click", actions.refresh);
  ext.listen(settings, "click", actions.settings);
  ext.listen(launch, "click", actions.launch);
  ext.onDispose(() => {
    style.remove();
    tab.remove();
    controls.remove();
    launch.remove();
  });

  return {
    sync(tabbar, state) {
      const add = tabbar?.querySelector(hostSelectors.panelAddButton);
      if (add && tab.parentElement !== add.parentElement) add.before(tab);
      // The controls go after T3's scrolling tab list, so a narrow panel never clips them.
      const tabList = tabbar?.querySelector(hostSelectors.panelTabList);
      if (tabList && controls.previousElementSibling !== tabList) tabList.after(controls);
      // Only a layout toolbar on screen, one with a width, takes the launch button.
      const toolbar = [...document.querySelectorAll(hostSelectors.layoutControls)].find((node) => node.getBoundingClientRect().width > 0);
      if (toolbar && launch.parentElement !== toolbar) toolbar.prepend(launch);
      tab.hidden = launch.hidden = !state.allowed;
      label.setAttribute("aria-pressed", String(state.visible));
      tab.toggleAttribute("data-selected", state.visible);
      controls.hidden = !state.visible;
      if (!state.visible) return;
      refresh.hidden = external.hidden = !state.repository;
      label.textContent = state.label;
      tab.title = state.title;
      if (state.external) external.href = state.external;
    },
  };
}
