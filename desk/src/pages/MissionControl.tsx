/**
 * Mission control, after Factory's Mission Control: one terminal-style
 * screen of bordered panes that scroll inside themselves. Active item on
 * the left, items and the progress log on the right, the threads at work
 * along the bottom, and a line of keys.
 */
import { useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import type { Agent } from "../api/Agent";
import type { ItemPage } from "../api/ItemPage";
import type { ItemRow } from "../api/ItemRow";
import type { MissionPage } from "../api/MissionPage";
import type { Checks, History, HistoryEvent } from "../api/spike";
import type { ThreadInfo } from "../api/ThreadInfo";
import { Crumbs } from "../components/Crumbs";
import { api, to, useAll, useData } from "../lib/api";
import { openThread } from "../lib/host";
import { roleWords } from "../lib/words";
import { since, who } from "./Home";

const STALE_MS = 30 * 60 * 1000;
/** Stages that should move on their own; sitting in one is worth a look. */
const MOVING = new Set(["building", "in_review", "changes_requested", "approved"]);
const LIVE = new Set(["preparing", "queued", "starting", "running", "waiting"]);

/** Every identity's thread as T3 Code knows it, live or ended. */
export function useThreads(ids: string[], full = false): Map<string, ThreadInfo> {
  const unique = [...new Set(ids)];
  const read = useAll<ThreadInfo>(
    unique.map((id) => api.thread(id) + (full ? "&full=1" : "")),
    full ? 15_000 : 60_000,
  );
  const map = new Map<string, ThreadInfo>();
  read.data.forEach((info, at) => info && map.set(unique[at], info));
  return map;
}

/** An item's stage in a word or two, with its review round. */
export function stage(row: ItemRow, events: HistoryEvent[]): string {
  const submits = events.filter((e) => e.id === row.id && e.kind === "submitted").length;
  switch (row.status) {
    case "in_review":
      return `in review ${Math.max(1, submits)}`;
    case "changes_requested":
      return "changes req";
    case "done":
      return events.some((e) => e.id === row.id && e.kind === "merged") ? "merged" : "done";
    default:
      return row.status.replaceAll("_", " ");
  }
}

/** When an item entered its stage: its latest recorded step, else when it was readied. */
function stageAt(id: string, events: HistoryEvent[], page: ItemPage | null | undefined): string | null {
  return events.find((e) => e.id === id)?.at ?? page?.ready?.at ?? null;
}

const ago = (iso: string | null | undefined) => {
  const s = since(iso);
  return s === null ? "" : s === "now" ? "now" : `${s} ago`;
};

/** `2h 14m`, `36m`, `48s`: a duration, short. */
function span(ms: number): string {
  const s = Math.max(0, Math.round(ms / 1000));
  if (s < 60) return `${s}s`;
  const m = Math.round(s / 60);
  if (m < 60) return `${m}m`;
  const h = Math.floor(m / 60);
  if (h < 48) return `${h}h ${m % 60}m`;
  return `${Math.round(h / 24)}d`;
}

function Pane({ title, right, className, children, bodyRef }: { title: ReactNode; right?: ReactNode; className?: string; children: ReactNode; bodyRef?: React.Ref<HTMLDivElement> }) {
  return (
    <section className={`mcx-pane${className ? ` ${className}` : ""}`}>
      <header>
        <span className="ptitle">{title}</span>
        {right ? <span className="pright">{right}</span> : null}
      </header>
      <div className="pbody" ref={bodyRef}>
        {children}
      </div>
    </section>
  );
}

function ActiveItem({ repo, row, page, events, holder }: { repo: string; row: ItemRow; page: ItemPage | null; events: HistoryEvent[]; holder: string | null }) {
  const [all, setAll] = useState(false);
  useEffect(() => setAll(false), [row.id]);
  const pr = page?.pull_request ?? null;
  const checks = useData<Checks>(pr ? api.checks(repo, pr.number) : null, 60_000);
  const blockers = row.blockers.filter((b) => b.status !== "done");
  const doneWhen = page?.done_when ?? [];
  const shown = all ? doneWhen : doneWhen.slice(0, 6);
  const ticked = doneWhen.filter((d) => d.ticked).length;
  const at = stageAt(row.id, events, page);
  const reviewer = page?.threads.find((a) => a.identity === row.reviewer);
  return (
    <>
      <p className="mcx-title">{row.title}</p>
      <dl className="mcx-kv">
        <dt>stage</dt>
        <dd>
          <span className={`st ${row.status}`}>{stage(row, events)}</span>
          {at ? <span className="dim"> for {span(Date.now() - Date.parse(at))}</span> : null}
        </dd>
        <dt>holder</dt>
        <dd>{holder ?? <span className="dim">nobody</span>}</dd>
        {row.status === "in_review" ? (
          <>
            <dt>reviewer</dt>
            <dd>{reviewer?.thread?.title ?? (row.reviewer ? who(null, row.reviewer) : <span className="dim">none yet</span>)}</dd>
          </>
        ) : null}
        {row.needs_human ? (
          <>
            <dt>waits on</dt>
            <dd className="you-text">you (needs-human)</dd>
          </>
        ) : null}
      </dl>
      {blockers.length > 0 ? (
        <>
          <h4>Blocked by</h4>
          <ul className="mcx-bullets">
            {blockers.map((b) => (
              <li key={b.id}>
                {b.id} <span className="dim">({b.status ? b.status.replaceAll("_", " ") : "not found"})</span>
              </li>
            ))}
          </ul>
        </>
      ) : null}
      {doneWhen.length > 0 ? (
        <>
          <h4>
            Done when <span className="dim">{ticked}/{doneWhen.length}</span>
          </h4>
          <ul className="mcx-bullets ticks">
            {shown.map((d, at) => (
              <li key={at} className={d.ticked ? "on" : undefined}>
                <span className="mark">{d.ticked ? "✓" : "·"}</span>
                <span>{d.text}</span>
              </li>
            ))}
          </ul>
          {doneWhen.length > shown.length ? (
            <button type="button" className="mcx-more" onClick={() => setAll(true)}>
              +{doneWhen.length - shown.length} more
            </button>
          ) : null}
        </>
      ) : null}
      {page?.goal ? <blockquote className="mcx-quote">{page.goal.trim()}</blockquote> : null}
      <p className="mcx-pr">
        <span className="dim">PR </span>
        {pr ? (
          <>
            <a href={pr.url} target="_blank" rel="noreferrer">
              #{pr.number} {pr.state.toLowerCase()}
            </a>
            {checks.data ? (
              <span className={`ci ${checks.data.state}`}>
                {" "}
                · CI {checks.data.state}
                {checks.data.checks.length > 0
                  ? ` (${checks.data.checks.filter((c) => c.state === "SUCCESS").length} passed, ${checks.data.checks.filter((c) => c.state === "SKIPPED").length} skipped)`
                  : ""}
              </span>
            ) : null}
          </>
        ) : (
          <span className="dim">{row.branch ? "none found for its branch" : "no branch yet"}</span>
        )}
      </p>
    </>
  );
}

type Entry = { at: string; text: string; id: string; waited: boolean; tone: string };

function logEntries(missionId: string, items: ItemRow[], events: HistoryEvent[], pages: Map<string, ItemPage>, caller: string): Entry[] {
  const needsYou = new Set(items.filter((i) => i.needs_human).map((i) => i.id));
  const recorded = new Set([...pages].filter(([, p]) => p.ready?.by.toLowerCase().startsWith(caller)).map(([id]) => id));
  const entries: Entry[] = [];
  for (const e of events) {
    if (e.kind === "readied" && recorded.has(e.id)) continue;
    const plan = e.id === missionId;
    const waited = needsYou.has(e.id);
    const one = (text: string, tone: string) => entries.push({ at: e.at, text, id: e.id, waited, tone });
    switch (e.kind) {
      case "planned":
        one("Mission planned", "plain");
        break;
      case "plan_review":
        one(`Plan review ${e.round}: ${e.verdict}`, e.verdict === "approved" ? "ok" : "bad");
        break;
      case "started":
        one(`Started ${e.id}`, "run");
        break;
      case "submitted":
        one(`Submitted ${e.id} for review`, "review");
        break;
      case "review":
        one(`Review ${e.round} of ${e.id}: ${e.verdict}`, e.verdict === "approved" ? "ok" : "bad");
        break;
      case "merged":
        one(plan ? "Plan merged: mission agreed" : `Merged ${e.id}`, "ok");
        break;
      case "closed":
        one(plan ? "Mission closed with its result" : `Closed ${e.id}`, "ok");
        break;
      case "dropped":
        one(plan ? "Mission dropped" : `Dropped ${e.id}`, "dim");
        break;
      case "readied":
        one(`Readied ${plan ? "the plan" : e.id}`, "plain");
        break;
      case "led":
        one("Lead took the mission", "plain");
        break;
    }
  }
  const readied = new Map<string, { by: string; at: string; ids: string[] }>();
  for (const [id, page] of pages) {
    if (!page.ready || !page.ready.by.toLowerCase().startsWith(caller)) continue;
    const key = `${page.ready.by}\n${page.ready.at.slice(0, 16)}`;
    const group = readied.get(key) ?? { by: page.ready.by, at: page.ready.at, ids: [] };
    group.ids.push(id);
    readied.set(key, group);
  }
  for (const g of readied.values()) {
    const said = g.by.includes(":") ? `: "${g.by.split(":").slice(1).join(":").trim()}"` : "";
    entries.push({ at: g.at, text: `Sumeet readied ${g.ids.length === 1 ? g.ids[0] : `${g.ids.length} items`}${said}`, id: g.ids[0], waited: true, tone: "you" });
  }
  return entries.sort((a, b) => Date.parse(b.at) - Date.parse(a.at));
}

/** How many one-line rows fit in an element, kept current as it resizes. */
function useRows(ref: React.RefObject<HTMLDivElement | null>, ready: boolean, line = 20): number {
  const [rows, setRows] = useState(12);
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const watch = new ResizeObserver(() => setRows(Math.max(3, Math.floor((el.clientHeight - 4) / line))));
    watch.observe(el);
    return () => watch.disconnect();
  }, [ref, line, ready]);
  return rows;
}

type Worker = { agent: Agent; info: ThreadInfo | undefined; live: boolean };

function WorkerLine({ w, now }: { w: Worker; now: number }) {
  const info = w.info;
  const status = w.agent.thread?.status ?? info?.status ?? "unknown";
  const model = (w.agent.thread?.model ?? info?.model ?? "").replace(/^claude-/, "");
  const ran = info?.started_at ? (w.live ? now - Date.parse(info.started_at) : Date.parse(info.updated_at) - Date.parse(info.started_at)) : null;
  return (
    <button type="button" className={`mcx-worker${w.live ? " live" : ""}`} onClick={() => openThread(w.agent.identity)} title={`${info?.title ?? w.agent.identity}: open in T3 Code`}>
      <span className={`dot ${w.live ? status : "done"}`}>{w.live ? "●" : "✓"}</span>
      <span className="role">{roleWords[w.agent.role].toLowerCase()}</span>
      <span className="on">{w.agent.item ?? "mission"}</span>
      <span className="model">{model}</span>
      <span className="time">{w.live ? (ran !== null ? span(ran) : "") : ran !== null ? `ran ${span(ran)}` : ""}</span>
      <span className="what">
        {w.live ? null : <span className="dim">finished · </span>}
        {status === "waiting" ? <span className="you-text">waiting on a person · </span> : null}
        {info?.last_line ?? info?.title ?? w.agent.identity}
      </span>
    </button>
  );
}

/** `/mission/:repo/:id`: mission control. */
export function MissionControlPage() {
  const { repo = "", id = "" } = useParams();
  const [params, setParams] = useSearchParams();
  const navigate = useNavigate();
  const loaded = useData<MissionPage>(api.mission(repo, id), 15_000);
  const mission = loaded.data;
  const items = useMemo(() => mission?.items ?? [], [mission]);
  const history = useData<History>(mission ? api.history(repo, { ids: [id, ...items.map((i) => i.id)] }) : null, 30_000);
  const pagesRead = useAll<ItemPage>(items.map((i) => api.item(repo, i.id)), 30_000);
  const pages = useMemo(() => {
    const map = new Map<string, ItemPage>();
    pagesRead.data.forEach((p, at) => p && items[at] && map.set(items[at].id, p));
    return map;
  }, [pagesRead.data, items]);
  const events = useMemo(() => history.data?.events ?? [], [history.data]);
  const now = Date.now();

  // Every agent on the mission and its items, with the live ones read in full for their latest line.
  const agents = useMemo(() => {
    const all = [...(mission?.threads ?? [])];
    for (const p of pages.values()) for (const a of p.threads) if (!all.some((b) => b.identity === a.identity && b.role === a.role)) all.push({ ...a, item: p.row.id });
    return all;
  }, [mission, pages]);
  // A done mission has no work left, so a thread of it still running is about something else.
  const finishedMission = mission?.mission.status === "done" || mission?.mission.status === "dropped";
  const liveAgents = finishedMission ? [] : agents.filter((a) => a.thread && LIVE.has(a.thread.status));
  const infos = useThreads(agents.map((a) => a.identity));

  const live = items.filter((i) => MOVING.has(i.status));
  const merged = (i: ItemRow) => events.find((e) => e.id === i.id && e.kind === "merged")?.at ?? "";
  const fallback =
    live[0] ??
    items.find((i) => i.status === "ready") ??
    [...items].filter((i) => i.status === "done").sort((a, b) => merged(b).localeCompare(merged(a)))[0] ??
    items.find((i) => i.status !== "draft") ??
    items[0] ??
    null;
  const selected = items.find((i) => i.id === params.get("item")) ?? fallback;
  const select = (item: string) => {
    const next = new URLSearchParams(params);
    next.set("item", item);
    setParams(next, { replace: true });
  };

  const onItem = selected ? agents.filter((a) => a.item === selected.id) : [];
  const recentOnItem = [...onItem]
    .filter((a) => a.role === "implementer" || a.role === "reviewer")
    .sort((a, b) => (infos.get(b.identity)?.updated_at ?? "").localeCompare(infos.get(a.identity)?.updated_at ?? ""))
    .slice(0, 3);
  const shownWorkers = liveAgents.length > 0 ? liveAgents : recentOnItem;
  const full = useThreads(
    shownWorkers.map((a) => a.identity),
    true,
  );
  const workers: Worker[] = shownWorkers.map((a) => ({ agent: a, info: full.get(a.identity) ?? infos.get(a.identity), live: liveAgents.includes(a) }));

  const name = (identity: string | null) => {
    if (!identity) return null;
    const a = agents.find((x) => x.identity === identity && x.thread);
    return a?.thread?.title ?? infos.get(identity)?.title ?? who(null, identity);
  };
  const lead = mission?.threads.find((a) => a.role === "lead") ?? mission?.threads.find((a) => a.role === "planner") ?? null;
  const currentThread = (row: ItemRow | null): string | null => {
    if (!row) return null;
    if (row.status === "in_review" && row.reviewer) return row.reviewer;
    if (row.holder) return row.holder;
    const last = agents.filter((a) => a.item === row.id).at(-1);
    return last?.identity ?? null;
  };

  // The log, paged to the rows its pane holds.
  const logRef = useRef<HTMLDivElement>(null);
  const rows = useRows(logRef, !!mission);
  const [page, setPage] = useState(0);
  const entries = mission ? logEntries(id, items, events, pages, "sumeet") : [];
  const pagesCount = Math.max(1, Math.ceil(entries.length / rows));
  const pageAt = Math.min(page, pagesCount - 1);
  const shownLog = entries.slice(pageAt * rows, pageAt * rows + rows);

  // The keys in the footer.
  const keys = useRef<(e: KeyboardEvent) => void>(() => undefined);
  keys.current = (e: KeyboardEvent) => {
    const target = e.target as HTMLElement | null;
    if (e.defaultPrevented || e.metaKey || e.ctrlKey || e.altKey || target?.closest("input, textarea, [contenteditable]")) return;
    const at = selected ? items.findIndex((i) => i.id === selected.id) : -1;
    const key = e.key;
    if ((key === "ArrowDown" || key === "j") && items.length > 0) {
      select(items[Math.min(items.length - 1, at + 1)].id);
      e.preventDefault();
    } else if ((key === "ArrowUp" || key === "k") && items.length > 0) {
      select(items[Math.max(0, at - 1)].id);
      e.preventDefault();
    } else if (key === "Enter") {
      const thread = currentThread(selected);
      if (thread) openThread(thread);
      e.preventDefault();
    } else if (key === "l" || key === "L") {
      if (lead) openThread(lead.identity);
    } else if ((key === "i" || key === "I") && selected) {
      navigate(to.item(repo, selected.id));
    } else if (key === "]" || key === "PageDown") {
      setPage(Math.min(pagesCount - 1, pageAt + 1));
    } else if (key === "[" || key === "PageUp") {
      setPage(Math.max(0, pageAt - 1));
    }
  };
  useEffect(() => {
    const listen = (e: KeyboardEvent) => keys.current(e);
    window.addEventListener("keydown", listen);
    return () => window.removeEventListener("keydown", listen);
  }, []);
  // Keep the selected row in view as the keys move it.
  useEffect(() => {
    document.querySelector(".mcx-items .on")?.scrollIntoView({ block: "nearest" });
  }, [selected?.id]);

  if (!mission) {
    return (
      <div className="mcx">
        <Crumbs trail={[{ label: id }]} />
        <div className="mcx-top">
          <b>Mission Control</b>
          <span className="dim">
            {repo} / {id}
          </span>
        </div>
        <p className="mcx-loading">{loaded.error ?? "Reading the mission…"}</p>
      </div>
    );
  }

  const counts = mission.mission.counts;
  const total = counts.reduce((n, c) => n + c.count, 0);
  const done = counts.filter((c) => c.status === "done").reduce((n, c) => n + c.count, 0);
  const planMerged = events.find((e) => e.id === id && e.kind === "merged");
  const state = mission.mission.status === "done" ? "DONE" : mission.mission.state === "proposed" ? "PROPOSED" : live.length > 0 ? "RUNNING" : "AGREED";
  const quiet = live.filter((i) => {
    const at = stageAt(i.id, events, pages.get(i.id));
    return at !== null && now - Date.parse(at) > STALE_MS;
  });

  return (
    <div className="mcx">
      <Crumbs trail={[{ label: mission.mission.title }]} />
      <div className="mcx-top">
        <b>Mission Control</b>
        <span className="dim">
          {repo} / {id}
        </span>
        <span className="ttl">{mission.mission.title}</span>
        <span className="right">
          {planMerged ? `Plan merged ${ago(planMerged.at)}` : state === "PROPOSED" ? "Plan on its branch" : "Plan merged"}
          <span className="sep">·</span>Live {live.length}
          <span className="sep">·</span>
          <span className={quiet.length > 0 ? "bad-text" : undefined}>Quiet {quiet.length}</span>
        </span>
      </div>
      <div className="mcx-state">
        <span className={`sdot ${state.toLowerCase()}`}>●</span>
        <b>{state}</b>
        <span className="bar" role="img" aria-label={`${done} of ${total} done`}>
          <i style={{ width: total ? `${(done / total) * 100}%` : 0 }} />
        </span>
        <span className="num">
          {done}/{total}
        </span>
      </div>

      <div className="mcx-main">
        <Pane title={`Active Item ${selected?.id ?? ""}`} className="active">
          {selected ? <ActiveItem repo={repo} row={selected} page={pages.get(selected.id) ?? null} events={events} holder={name(selected.holder)} /> : <p className="dim">This mission has no items yet.</p>}
        </Pane>
        <div className="mcx-right">
          <Pane title={`Items ${done}/${total}`} className="items">
            <div className="mcx-items" role="listbox" aria-label="Items">
              {items.map((row) => {
                const at = stageAt(row.id, events, pages.get(row.id));
                const stuck = MOVING.has(row.status) && at !== null && now - Date.parse(at) > STALE_MS;
                const mark = row.status === "done" ? "✓" : row.status === "dropped" ? "×" : MOVING.has(row.status) ? "●" : "○";
                return (
                  <button
                    key={row.id}
                    type="button"
                    role="option"
                    aria-selected={row.id === selected?.id}
                    className={`mcx-row${row.id === selected?.id ? " on" : ""} ${row.status}`}
                    onClick={() => select(row.id)}
                  >
                    <span className="mark">{mark}</span>
                    <span className="id">{row.id}</span>
                    <span className="t">{row.title}</span>
                    <span className={`st ${row.status}${stuck ? " stuck" : ""}`}>{stuck ? `! ${stage(row, events)} ${since(at)}` : stage(row, events)}</span>
                  </button>
                );
              })}
            </div>
          </Pane>
          <Pane
            title={`Progress Log ${entries.length === 0 ? 0 : pageAt * rows + 1}–${pageAt * rows + shownLog.length} of ${entries.length}`}
            right={
              <span className="pager">
                <button type="button" disabled={pageAt === 0} onClick={() => setPage(pageAt - 1)} aria-label="Newer">
                  ‹
                </button>
                <button type="button" disabled={pageAt >= pagesCount - 1} onClick={() => setPage(pageAt + 1)} aria-label="Older">
                  ›
                </button>
              </span>
            }
            className="log"
            bodyRef={logRef}
          >
            {shownLog.map((e, at) => (
              <button key={at} type="button" className={`mcx-log${e.waited ? " waited" : ""}`} onClick={() => items.some((i) => i.id === e.id) && select(e.id)} title={e.at}>
                <span className="when">{ago(e.at)}</span>
                <span className={`what ${e.tone}`}>{e.text}</span>
                {e.waited ? <span className="you-text">waited on you</span> : null}
              </button>
            ))}
          </Pane>
        </div>
      </div>

      <Pane
        title={liveAgents.length > 0 ? `Active Threads ${liveAgents.length}` : `Active Threads 0 · last on ${selected?.id ?? "this mission"}`}
        right={liveAgents.length > 0 ? `Time ${span(now - Math.min(...workers.map((w) => (w.info?.started_at ? Date.parse(w.info.started_at) : now))))}` : null}
        className="workers"
      >
        {workers.length === 0 ? <p className="dim">No thread has worked on this item yet.</p> : null}
        {workers.map((w) => (
          <WorkerLine key={`${w.agent.identity}${w.agent.role}`} w={w} now={now} />
        ))}
      </Pane>

      <div className="mcx-keys">
        <span>
          <kbd>↑↓</kbd> <kbd>j</kbd>
          <kbd>k</kbd> Select
        </span>
        <span>
          <kbd>Enter</kbd> Open thread
        </span>
        <span>
          <kbd>L</kbd> Talk to {lead?.role === "lead" ? "lead" : "planner"}
        </span>
        <span>
          <kbd>I</kbd> Item page
        </span>
        <span>
          <kbd>[</kbd>
          <kbd>]</kbd> Log page
        </span>
        <span>
          <kbd>Esc</kbd> Close
        </span>
      </div>
    </div>
  );
}
