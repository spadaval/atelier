// Places the Customizations section at the end of T3's Settings → General page, drawn by React in
// a shadow root of its own, and redraws it on each DOM pass while it is there.
import type { Extension } from "../core/extension";
import { element } from "../core/dom";
import { hostSelectors } from "../host/adapter";
import { mountReact } from "../react/mount";
import { projectKey } from "../repositories";
import { Customizations } from "./Customizations";

export type SettingsSection = {
  /** Goes to T3's Settings → General page and scrolls to the section. */
  open(): void;
};

/**
 * `changed` runs after a setting the section edits is saved: the views follow it, and a record
 * link's repository override is dropped.
 */
export function createSettingsSection(ext: Extension, hooks: { showDesk(): void; changed(): void; reloadRepositories(): void }): SettingsSection {
  const { settings } = ext;
  const host = element("section", { id: "t3-custom-settings", "data-t3-custom-settings": "" });
  host.style.cssText = "display:block;margin:24px 0;padding:20px 0;border-top:1px solid var(--border)";
  const view = mountReact(host, ext.api);
  let placed = 0;
  let focus = false;

  const save = () => {
    void ext.saveSettings();
    ext.scheduleDOM();
  };
  const actions = {
    toggle(key: "enabled" | "references" | "followProject" | "autoUpdate", value: boolean) {
      settings[key] = value;
      ext.repositories.override = null;
      save();
      hooks.changed();
    },
    // Automatic follows the Desk's repository identity; anything else is a manual override.
    map(value: string) {
      if (!ext.current) return;
      if (value === "@automatic") delete settings.mappings[projectKey(ext.current)];
      else settings.mappings[projectKey(ext.current)] = value;
      ext.repositories.override = null;
      save();
    },
    pin(value: string) {
      settings.pinnedRepo = value;
      ext.repositories.override = null;
      save();
    },
    removeMapping(project: string) {
      delete settings.mappings[project];
      save();
    },
    showDesk: hooks.showDesk,
    reloadRepositories: hooks.reloadRepositories,
  };

  ext.onDOM(() => {
    const target = location.hash.startsWith("#/settings/general") ? document.querySelector(hostSelectors.settingsPage) : null;
    if (!target) {
      host.remove();
      return;
    }
    const container = target.firstElementChild ?? target;
    if (host.parentElement !== container) {
      container.append(host);
      placed++;
    }
    view.render(<Customizations ext={ext} actions={actions} placed={placed} />);
    if (focus) {
      focus = false;
      host.scrollIntoView({ block: "start" });
    }
  });
  ext.onDispose(() => host.remove());

  return {
    open() {
      focus = true;
      location.hash = "/settings/general";
      ext.scheduleDOM();
    },
  };
}
