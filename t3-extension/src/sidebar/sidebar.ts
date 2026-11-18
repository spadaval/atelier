// The Inbox row under T3's sidebar search, with its count; under it, a note when T3's list cannot
// hold missions and the errors of reading them (missions.ts). The row sits in T3's sidebar among
// its own nodes, so it is a DOM node, not React.
import type { InboxResponse } from "../../../desk/src/api/InboxResponse";
import { element, icon } from "../core/dom";
import type { Extension } from "../core/extension";
import { deskGet, messageOf } from "../host/api";
import { expectHostDom, hostSelectors } from "../host/adapter";
import { layerTargets, type Layer } from "../layer/layer";
import { viewOn } from "../views";
import { createSidebarMissions } from "./missions";

export type Sidebar = {
  /** Reads the inbox and the missions from every Desk again. */
  read(): void;
  /** Matches T3's threads to missions again. */
  refresh(): void;
};

const inboxIcon = icon('<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>');

export function createSidebar(ext: Extension, layer: Layer): Sidebar {
  const { api, deskUrls, settings } = ext;
  const missions = createSidebarMissions(ext, layer);
  // T3's own --sidebar-* variables, so the row follows its theme, with the metrics of T3's rows.
  const style = element("style", {
    text: `
  [data-t3-ext-sidebar] { flex-shrink: 0; display: flex; flex-direction: column; padding: 0 var(--sidebar-content-inset, 8px) 4px; }
  [data-t3-side-row] { display: flex; align-items: center; gap: 10px; height: 32px; padding: 0 var(--sidebar-row-content-inset, 10px); border-radius: var(--control-radius, 6px); cursor: pointer; font-size: 14px; font-weight: 500; color: var(--sidebar-foreground); user-select: none; outline: none; -webkit-app-region: no-drag; min-width: 0; }
  [data-t3-side-row]:hover { background: var(--sidebar-row-hover); }
  [data-t3-side-row][aria-pressed="true"] { background: var(--sidebar-row-active); }
  [data-t3-side-row]:focus-visible { box-shadow: inset 0 0 0 2px var(--ring); outline: none; }
  [data-t3-side-icon] { display: inline-flex; flex-shrink: 0; width: 16px; height: 16px; color: var(--sidebar-muted-foreground); } [data-t3-side-icon] svg { width: 16px; height: 16px; }
  [data-t3-side-row] [data-t3-side-label] { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  [data-t3-inbox-count] { font-size: 12px; font-variant-numeric: tabular-nums; min-width: 20px; padding: 1px 6px; border-radius: 999px; text-align: center; background: var(--sidebar-row-active); color: var(--sidebar-foreground); }
  [data-t3-inbox-count][data-zero] { background: transparent; color: var(--sidebar-muted-foreground); }
  [data-t3-inbox-count][data-error] { color: var(--destructive); }
`,
  });
  document.head.append(style);

  const count = element("span", { "data-t3-inbox-count": "", text: "…" });
  const inbox = layerTargets.inbox(deskUrls[0]);
  const row = element(
    "div",
    { "data-t3-side-row": "", "data-t3-inbox-row": "", "data-t3-layer-key": inbox.key, role: "button", tabindex: "0", "aria-pressed": "false", title: "Atelier inbox" },
    element("span", { "data-t3-side-icon": "", html: inboxIcon }),
    element("span", { "data-t3-side-label": "", text: "Inbox" }),
    count,
  );
  ext.listen(row, "click", () => layer.toggle(inbox));
  ext.listen(row, "keydown", (event) => {
    if (event.target === row && (event.key === "Enter" || event.key === " ")) {
      event.preventDefault();
      row.click();
    }
  });
  const container = element("div", { "data-t3-ext-sidebar": "" }, row, ...missions.notes);

  // The count: needs-human items, proposed plans and waiting threads, across repositories and Desks.
  // A Desk that cannot be read, or a repository it reports with an error (its lists are then empty),
  // shows "!" beside the count of what could be read, with each error in the row's tooltip.
  async function readInbox() {
    const errors: string[] = [];
    let total = 0;
    let read = 0;
    await Promise.all(
      deskUrls.map(async (desk) => {
        try {
          for (const repo of (await deskGet<InboxResponse>(api, desk, "/api/inbox")).repos) {
            if (repo.error) errors.push(`${desk} could not read ${repo.repo}: ${repo.error}`);
            total += (repo.needs_human ?? []).reduce((sum, group) => sum + group.items.length, 0) + (repo.proposed ?? []).length + (repo.threads ?? []).length;
          }
          read++;
        } catch (error) {
          errors.push(`GET ${desk}/api/inbox: ${messageOf(error)}`);
        }
      }),
    );
    if (api.signal.aborted) return;
    count.textContent = !errors.length ? String(total) : read ? `${total} !` : "!";
    count.toggleAttribute("data-error", errors.length > 0);
    count.toggleAttribute("data-zero", !errors.length && total === 0);
    row.title = errors.length ? `Atelier inbox: ${read ? `${total} waiting in what could be read; ` : ""}${errors.join("; ")}` : `Atelier inbox: ${total} waiting (read ${new Date().toLocaleTimeString()})`;
  }
  const read = () => {
    void readInbox();
    void missions.read();
  };
  read();
  const timer = setInterval(read, 60000);

  ext.onDOM(() => {
    const onMain = Boolean(ext.host) && !location.hash.includes("/settings");
    // The rows T3 lists are expected only where the missions read them.
    const t3Threads = onMain && viewOn("missionThreads") && (ext.host?.threadList?.().length ?? 0) > 0;
    expectHostDom("sidebarSearch", onMain, "T3 showed a thread or new-thread screen");
    expectHostDom("threadItem", t3Threads, "T3 listed its threads");
    expectHostDom("sidebarRow", t3Threads, "T3 listed its threads");
    const search = viewOn("sidebar") ? document.querySelector(hostSelectors.sidebarSearch) : null;
    const group = search?.closest('[data-slot="sidebar-group"]')?.parentElement;
    if (group && settings.enabled) {
      if (container.previousElementSibling !== group) group.after(container);
    } else container.remove();
    missions.sync();
  });
  ext.onDispose(() => {
    clearInterval(timer);
    container.remove();
    style.remove();
  });

  return { read, refresh: missions.refresh };
}
