// Missions in T3's own thread list. T3's boring sidebar is one flex column of li: a header per
// project folder, the folder's thread rows after it, then other rows (pinned, shelves). The
// extension never moves or changes T3's nodes: it appends its own rows to that list and gives
// every child a CSS order, so each open folder reads: its header, each mission with threads in it
// (a header, then those threads, indented, with their role), "N more missions" for the
// repository's missions with none there, then the folder's other threads in T3's order. A
// generated stylesheet keyed on data-thread-item indents, labels and folds the mission threads.
// The rows sit inside T3's list among its own, so they are DOM nodes, not React.
import type { Role } from "../../../desk/src/api/Role";
import type { SideMission } from "../../../desk/src/api/SideMission";
import type { SideRepo } from "../../../desk/src/api/SideRepo";
import type { SidebarResponse } from "../../../desk/src/api/SidebarResponse";
import { element, icon } from "../core/dom";
import type { Extension } from "../core/extension";
import { deskGet, messageOf } from "../host/api";
import { expectHostDom, hostListProblem, hostSelectors, setHostProbe, type ThreadEntry } from "../host/adapter";
import { layerTargets, type Layer } from "../layer/layer";
import { repositoryKey, type Repository } from "../repositories";
import { viewOn } from "../views";

/** An open mission, with the Desk and repository that serve it; `key` is `<desk>/<repo>/<id>`. */
type MissionEntry = { key: string; desk: string; repo: string; mission: SideMission };
/** The mission a T3 row belongs to, and its role there; a thread its threads started has none. */
type Claim = { entry: MissionEntry; role: Role | null };

export type SidebarMissions = {
  /** The notes under the Inbox row: how to turn T3's boring sidebar on, and the Desks' read errors. */
  notes: HTMLElement[];
  /** Reads GET /api/sidebar from each Desk. */
  read(): Promise<void>;
  /** Matches T3's threads to missions again and lays the list out. */
  refresh(): void;
  /** Lays the missions out in T3's list. */
  sync(): void;
};

// The Desk's role names, as the list labels a mission thread.
const roleLabels: Record<Role, string> = { lead: "lead", planner: "plan", implementer: "build", reviewer: "review", decider: "decide" };
const chevron = icon('<path d="m6 9 6 6 6-6"/>');
// Which missions are folded, in localStorage; a mission starts open.
const foldedKey = "t3-custom:atelier:missions:v1";

export function createSidebarMissions(ext: Extension, layer: Layer): SidebarMissions {
  const { api, deskUrls, settings } = ext;
  // The rows copy the metrics of T3's folder header: 32px high, 14px text, a 12px chevron, the
  // count and dot in 12px muted text, in T3's --sidebar-* colours.
  const style = element("style", {
    text: `
  [data-t3-missions-note] { font-size: 12px; color: var(--sidebar-muted-foreground); padding: 4px var(--sidebar-row-content-inset, 10px); overflow-wrap: anywhere; white-space: pre-line; }
  [data-t3-missions-note][data-error] { color: var(--destructive); }
  li[data-t3-list-row] { list-style: none; }
  [data-t3-list-row] > div { display: flex; align-items: center; gap: 10px; height: 32px; padding: 0 var(--sidebar-row-content-inset, 10px) 0 12px; border-radius: var(--control-radius, 6px); min-width: 0; font-size: 14px; color: var(--sidebar-foreground); user-select: none; -webkit-app-region: no-drag; }
  [data-t3-list-row] > div:hover { background: var(--sidebar-row-hover); }
  [data-t3-list-row][aria-pressed="true"] > div { background: var(--sidebar-row-active); }
  [data-t3-list-row] button:focus-visible { box-shadow: inset 0 0 0 2px var(--ring); outline: none; }
  [data-t3-list-row] button { font: inherit; color: inherit; background: none; border: 0; padding: 0; cursor: pointer; text-align: start; height: 100%; }
  [data-t3-list-row] [data-t3-side-label] { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  [data-t3-mission-title] { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-weight: 500; }
  [data-t3-ring] { display: inline-flex; flex-shrink: 0; color: var(--sidebar-muted-foreground); }
  [data-t3-count] { flex-shrink: 0; font-size: 12px; font-variant-numeric: tabular-nums; color: var(--sidebar-muted-foreground); }
  [data-t3-dot] { flex-shrink: 0; width: 6px; height: 6px; border-radius: 50%; background: var(--warning, #f5a524); }
  [data-t3-fold] { display: inline-flex; align-items: center; flex-shrink: 0; color: var(--sidebar-muted-foreground) !important; }
  [data-t3-fold] svg { width: 12px; height: 12px; transition: transform .1s; } [data-t3-fold][aria-expanded="false"] svg { transform: rotate(-90deg); }
  [data-t3-more-row] > div { height: 28px; padding-left: 36px; font-size: 12px; color: var(--sidebar-muted-foreground); }
`,
  });
  const layoutStyle = element("style", { "data-t3-mission-layout": "" });
  document.head.append(style, layoutStyle);

  const boringNote = element("div", { "data-t3-missions-note": "", "data-t3-boring-note": "", hidden: "", text: "Missions show in T3's thread list with Settings → Labs → Boring sidebar on." });
  const errorNote = element("div", { "data-t3-missions-note": "", "data-error": "", hidden: "" });

  let folded: Record<string, boolean>;
  try {
    folded = (JSON.parse(localStorage.getItem(foldedKey) ?? "{}") as { folded?: Record<string, boolean> }).folded ?? {};
  } catch {
    folded = {};
  }
  const saveFolded = () => localStorage.setItem(foldedKey, JSON.stringify({ folded }));

  // GET /api/sidebar from each Desk; each mission keeps the Desk that served it.
  let sidebarRepos: (SideRepo & { desk: string })[] | null = null;
  async function read() {
    const repos: (SideRepo & { desk: string })[] = [];
    const errors: string[] = [];
    await Promise.all(
      deskUrls.map(async (desk) => {
        try {
          for (const repo of (await deskGet<SidebarResponse>(api, desk, "/api/sidebar")).repos) repos.push({ ...repo, desk });
        } catch (error) {
          errors.push(`GET ${desk}/api/sidebar: ${messageOf(error)}`);
        }
      }),
    );
    if (api.signal.aborted) return;
    sidebarRepos = repos;
    errorNote.hidden = !errors.length;
    errorNote.textContent = errors.join("\n");
    refresh();
  }

  const threadList = (): ThreadEntry[] => (viewOn("missionThreads") ? (ext.host?.threadList?.() ?? []) : []);
  // What T3's list needs, read on each Desk read and every 3 seconds: the open missions in the
  // Desks' order; which T3 row (by its exact "<environment>:<thread>" key) belongs to which mission,
  // with its role; and which repository each row's project maps to. A mission's "other threads" are
  // T3 threads whose creator (lineage.parentThreadId, or delegatedFrom.threadId for a thread
  // delegated from a linked environment) is one of its threads; they get no role.
  let missions: MissionEntry[] = [];
  let claims = new Map<string, Claim>();
  let rowRepos = new Map<string, Repository>();
  function refresh() {
    const list = threadList();
    const rowKey = (thread: ThreadEntry) => `${thread.environmentId}:${thread.id}`;
    missions = (sidebarRepos ?? []).flatMap((repo) => repo.missions.map((mission) => ({ key: `${repo.desk}/${repo.repo}/${mission.id}`, repo: repo.repo, desk: repo.desk, mission })));
    rowRepos = new Map();
    for (const thread of list) {
      const repo = ext.repositories.mapped(thread);
      if (repo) rowRepos.set(rowKey(thread), repo);
    }
    claims = new Map();
    for (const entry of missions) {
      const own = new Map<string, Role>();
      for (const thread of [...entry.mission.threads, ...entry.mission.items.flatMap((item) => item.threads)]) if (!own.has(thread.identity)) own.set(thread.identity, thread.role);
      for (const thread of list) {
        const key = rowKey(thread);
        if (claims.has(key)) continue;
        if (own.has(thread.id)) claims.set(key, { entry, role: own.get(thread.id) ?? null });
        else if ((thread.parent && own.has(thread.parent)) || (thread.delegatedFrom && own.has(thread.delegatedFrom))) claims.set(key, { entry, role: null });
      }
    }
    sync();
  }
  const timer = setInterval(refresh, 3000);

  const listRows = new Map<string, HTMLLIElement>();
  const rowSignatures = new WeakMap<HTMLElement, string>();
  let ordered = new Set<HTMLElement>();
  let layoutList: Element | null = null;
  let layoutActive = false;
  // T3 fades a removed row by appending a copy of it, the extension's rows included, so every row
  // carrying the extension's mark goes, copies too.
  const removeListRows = (keep?: Set<Element>) => {
    for (const node of document.querySelectorAll("[data-t3-list-row]")) if (!keep?.has(node)) node.remove();
  };
  function clearLayout() {
    for (const node of ordered) node.style.order = "";
    removeListRows();
    listRows.clear();
    ordered = new Set();
    layoutList = null;
    layoutActive = false;
    layoutStyle.textContent = "";
  }
  // One of the extension's rows in T3's list, kept across syncs and rebuilt when its data changes.
  function listRow(key: string, signature: string, build: (node: HTMLLIElement) => void) {
    let node = listRows.get(key);
    if (!node) {
      node = element("li", { "data-t3-list-row": "" });
      listRows.set(key, node);
    }
    if (rowSignatures.get(node) !== signature) {
      rowSignatures.set(node, signature);
      build(node);
    }
    return node;
  }
  const ring = (done: number, total: number) => {
    const length = 2 * Math.PI * 5;
    const part = total > 0 ? (length * done) / total : 0;
    return element("span", {
      "data-t3-ring": "",
      html: `<svg viewBox="0 0 14 14" width="14" height="14" aria-hidden="true"><circle cx="7" cy="7" r="5" fill="none" stroke="currentColor" stroke-opacity=".25" stroke-width="2"/><circle cx="7" cy="7" r="5" fill="none" stroke="var(--success, #10b981)" stroke-width="2" stroke-dasharray="${part} ${length}" transform="rotate(-90 7 7)"/></svg>`,
    });
  };
  // A mission's header, like T3's folder header: open, it shows items done of total; folded, the
  // number of its threads here and a dot when something in it waits on the human.
  function missionHeader(key: string, entry: MissionEntry, count: number, isFolded: boolean) {
    const { mission } = entry;
    return listRow(key, JSON.stringify([mission, count, isFolded]), (node) => {
      const proposed = mission.state === "proposed";
      const title = element("button", { "data-t3-mission-title": "", title: `${mission.id}: ${mission.title} (${mission.state}). Open ${proposed ? "its plan" : "the mission page"}`, text: mission.title });
      // An agreed or running mission opens the mission page; a proposed plan, which that page does
      // not serve, its record page, framed.
      ext.listen(title, "click", () => layer.toggle(layerTargets.mission(entry.desk, entry.repo, mission.id, !proposed)));
      const fold = element("button", { "data-t3-fold": "", "aria-expanded": String(!isFolded), "aria-label": `${isFolded ? "Unfold" : "Fold"} ${mission.title}`, html: chevron });
      ext.listen(fold, "click", () => {
        if (isFolded) delete folded[entry.key];
        else folded[entry.key] = true;
        saveFolded();
        sync();
      });
      const marks = isFolded
        ? [...(mission.waiting > 0 ? [element("span", { "data-t3-dot": "", title: `${mission.waiting} waiting on you` })] : []), element("span", { "data-t3-count": "", title: `${count} of its threads here`, text: String(count) })]
        : [element("span", { "data-t3-count": "", title: `${mission.done} of ${mission.total} items done`, text: `${mission.done}/${mission.total}` })];
      node.setAttribute("data-t3-mission-row", mission.id);
      node.setAttribute("data-t3-layer-key", layerTargets.mission(entry.desk, entry.repo, mission.id).key);
      node.replaceChildren(element("div", {}, ring(mission.done, mission.total), title, ...marks, fold));
    });
  }
  // The repository's missions with no thread in this folder, in one row that opens the project home.
  function moreRow(key: string, repo: Repository, rest: MissionEntry[]) {
    const waiting = rest.filter((entry) => entry.mission.waiting > 0).length;
    return listRow(key, JSON.stringify([repositoryKey(repo), rest.map((entry) => [entry.mission.title, entry.mission.waiting])]), (node) => {
      const target = layerTargets.home(repo);
      const open = element("button", { "data-t3-side-label": "", title: `${rest.map((entry) => `${entry.mission.id}: ${entry.mission.title}`).join("\n")}\nOpen the ${repo.repo} project home`, text: `${rest.length} more mission${rest.length === 1 ? "" : "s"}` });
      ext.listen(open, "click", () => layer.toggle(target));
      node.setAttribute("data-t3-more-row", repo.repo);
      node.setAttribute("data-t3-layer-key", target.key);
      node.replaceChildren(element("div", {}, open, ...(waiting ? [element("span", { "data-t3-dot": "", title: `${waiting} of them wait on you` })] : [])));
    });
  }

  function sync() {
    const on = settings.enabled && viewOn("missions");
    const list = on ? (document.querySelector(hostSelectors.projectFolder)?.parentElement ?? null) : null;
    // T3 lists threads but no folders, and is not searching: the boring sidebar is off.
    boringNote.hidden = !on || Boolean(list) || Boolean(document.querySelector<HTMLInputElement>(hostSelectors.sidebarSearch)?.value) || !document.querySelector(hostSelectors.threadItem);
    const problem = list && hostListProblem(list);
    if (list) setHostProbe("layout:threadList", !problem, problem ? `T3's thread list: ${problem}` : "project folders and their thread rows in one flex column");
    if (!list || problem) {
      if (layoutList || ordered.size || document.querySelector("[data-t3-list-row]")) clearLayout();
      return;
    }
    // T3 fades a row it removes by appending an inert, absolutely placed copy (Sidebar.motion.ts);
    // being out of flow, those copies, like the extension's own rows, are left out of the walk.
    const children = [...list.children].filter((node): node is HTMLElement => node instanceof HTMLElement && !node.hasAttribute("data-t3-list-row") && !(node.inert && node.style.position === "absolute"));
    const sequence: HTMLElement[] = [];
    const rules: string[] = [];
    const used = new Set<string>();
    let folderThreads = 0;
    const add = (key: string, node: HTMLElement) => {
      used.add(key);
      sequence.push(node);
    };
    for (let index = 0; index < children.length; ) {
      const node = children[index++];
      sequence.push(node);
      const button = node.matches(hostSelectors.projectFolder) ? node.querySelector<HTMLElement>(hostSelectors.folderButton) : null;
      if (!button) continue;
      // A folder's threads are the thread rows after its header. A closed folder keeps only the
      // open thread's row, which stays a plain row under it, with no mission rows. T3 can reopen
      // such a folder by its button's aria-expanded alone, which Extension.onMutation watches.
      const threads: HTMLElement[] = [];
      while (index < children.length && children[index].matches(hostSelectors.threadItem)) threads.push(children[index++]);
      folderThreads += threads.length;
      if (button.getAttribute("aria-expanded") !== "true") {
        sequence.push(...threads);
        continue;
      }
      const groups = new Map<MissionEntry, [HTMLElement, Claim][]>();
      const loose: HTMLElement[] = [];
      for (const item of threads) {
        const claim = claims.get(item.dataset.threadItem ?? "");
        if (!claim) {
          loose.push(item);
          continue;
        }
        const group = groups.get(claim.entry) ?? [];
        groups.set(claim.entry, group);
        group.push([item, claim]);
      }
      const folderKey = button.dataset.sidebarRow;
      for (const entry of missions) {
        const items = groups.get(entry);
        if (!items) continue;
        const isFolded = Boolean(folded[entry.key]);
        const key = `${folderKey}|${entry.key}`;
        add(key, missionHeader(key, entry, items.length, isFolded));
        for (const [item, claim] of items) {
          const selector = `li[data-thread-item="${CSS.escape(item.dataset.threadItem ?? "")}"]`;
          rules.push(isFolded ? `${selector} { display: none !important; }` : `${selector} { margin-left: 18px; box-shadow: inset 1px 0 var(--border); }`);
          if (claim.role && !isFolded) rules.push(`${selector} [data-sidebar-row]::after { content: "${CSS.escape(roleLabels[claim.role] ?? claim.role)}"; flex-shrink: 0; font-size: 11px; color: var(--sidebar-muted-foreground, var(--muted-foreground)); }`);
          sequence.push(item);
        }
      }
      const repo = threads.map((item) => rowRepos.get(item.dataset.threadItem ?? "")).find(Boolean);
      const rest = repo ? missions.filter((entry) => entry.desk === repo.desk && entry.repo === repo.repo && !groups.has(entry)) : [];
      if (repo && rest.length) add(`${folderKey}|more`, moreRow(`${folderKey}|more`, repo, rest));
      sequence.push(...loose);
    }
    for (const key of listRows.keys()) if (!used.has(key)) listRows.delete(key);
    removeListRows(new Set(listRows.values()));
    for (const node of listRows.values()) if (node.parentElement !== list) list.append(node);
    const next = new Set(sequence);
    for (const node of ordered) if (!next.has(node)) node.style.order = "";
    sequence.forEach((node, position) => {
      if (node.style.order !== String(position)) node.style.order = String(position);
    });
    ordered = next;
    layoutList = list;
    layoutActive = [...used].some((key) => !key.endsWith("|more"));
    const css = rules.join("\n");
    if (layoutStyle.textContent !== css) layoutStyle.textContent = css;
    for (const node of listRows.values()) node.setAttribute("aria-pressed", String(layer.pressed(node.dataset.t3LayerKey)));
    expectHostDom("settleButton", folderThreads > 0, "T3 listed threads in a project folder");
  }

  // Two of T3's gestures act on a run of rows taken from T3's own order, which differs from what is
  // drawn once a mission is placed: dragging from a row's Settle button settles every row between,
  // and Shift-click selects every row between the anchor and the row clicked, which bulk Settle,
  // Snooze, Delete and a drag to the composer then act on. While a mission is placed, both are
  // stopped before T3 sees them: a press on Settle, so no drag starts (a plain click still settles
  // that thread), and every click with Shift held on a thread row, whatever other key is held, as
  // T3's own rule for which of those is a range depends on the platform (a click and Ctrl- or
  // Cmd-click without Shift still work).
  ext.listen(
    document,
    "pointerdown",
    (event) => {
      if (layoutActive && event.target instanceof Element && event.target.closest(hostSelectors.settleButton)) event.stopPropagation();
    },
    { capture: true },
  );
  ext.listen(
    document,
    "click",
    (event) => {
      if (layoutActive && event.shiftKey && event.target instanceof Element && event.target.closest(hostSelectors.threadItem)) {
        event.preventDefault();
        event.stopPropagation();
      }
    },
    { capture: true },
  );

  // Missions are placed before the browser paints what T3 changed in its list, and on the new
  // list when T3 replaces it (a search, a route change).
  ext.onMutation((records) => {
    const list = document.querySelector(hostSelectors.projectFolder)?.parentElement ?? null;
    const sidebar = document.querySelector('[data-workspace-area="sidebar"]');
    if (list !== layoutList || records.some((record) => sidebar?.contains(record.target))) sync();
  });
  ext.onDispose(() => {
    clearInterval(timer);
    clearLayout();
    style.remove();
    layoutStyle.remove();
  });

  return { notes: [boringNote, errorNote], read, refresh, sync };
}
