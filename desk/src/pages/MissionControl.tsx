import { Check, ChevronRight, CircleCheck, Clock, ExternalLink, GitMerge, GitPullRequest, Hand, MessageSquare, Play, Send, Square, Target, ThumbsDown, ThumbsUp, User } from "lucide-react";
import { useMemo, useState } from "react";
import { Link, useParams, useSearchParams } from "react-router";
import type { Agent } from "../api/Agent";
import type { ItemPage } from "../api/ItemPage";
import type { ItemRow } from "../api/ItemRow";
import type { MissionPage } from "../api/MissionPage";
import type { Checks, History, HistoryEvent } from "../api/spike";
import type { ThreadInfo } from "../api/ThreadInfo";
import { Crumbs } from "../components/Crumbs";
import { ErrorBanner, Markdown, ScopeBar, Skeleton, StatusGlyph } from "../components/ui";
import { api, to, useAll, useData } from "../lib/api";
import { openThread, useConnected } from "../lib/host";
import { roleWords, stateWords } from "../lib/words";
import { since, who } from "./Home";

const STALE_MS = 30 * 60 * 1000;
/** Stages that should move on their own; sitting in one is worth a look. */
const MOVING = new Set(["building", "in_review", "changes_requested", "approved"]);
const ENDED = new Set(["completed", "cancelled", "failed", "interrupted", "rolled_back"]);

/** Every identity's thread as T3 Code knows it, live or ended. */
export function useThreads(ids: string[]): Map<string, ThreadInfo> {
  const unique = [...new Set(ids)];
  const read = useAll<ThreadInfo>(unique.map(api.thread), 60_000);
  const map = new Map<string, ThreadInfo>();
  read.data.forEach((info, at) => info && map.set(unique[at], info));
  return map;
}

/** An item's stage as the human reads it, with its review round. */
export function stage(row: ItemRow, events: HistoryEvent[]): string {
  const submits = events.filter((e) => e.id === row.id && e.kind === "submitted").length;
  switch (row.status) {
    case "in_review":
      return `in review, round ${Math.max(1, submits)}`;
    case "changes_requested":
      return "changes requested";
    case "done":
      return events.some((e) => e.id === row.id && e.kind === "merged") ? "merged" : "done";
    default:
      return row.status.replaceAll("_", " ");
  }
}

/** When an item entered its stage: its latest recorded step, else when it was readied. */
function stageAt(id: string, events: HistoryEvent[], page: ItemPage | null): string | null {
  const latest = events.find((e) => e.id === id);
  return latest?.at ?? page?.ready?.at ?? null;
}

function ThreadLine({ agent, info }: { agent: Agent; info: ThreadInfo | undefined }) {
  const hosted = useConnected();
  const title = agent.thread?.title ?? info?.title ?? who(null, agent.identity);
  const status = agent.thread?.status ?? info?.status ?? "unknown";
  const model = agent.thread?.model ?? info?.model ?? null;
  const ended = ENDED.has(status);
  return (
    <button type="button" className="mc-thread" onClick={() => openThread(agent.identity)} title={hosted ? "Open this thread in T3 Code" : agent.identity}>
      <span className={`pulse ${ended ? "none" : status}`} aria-hidden />
      <span className="grow">
        <span className="t">{title}</span>
        <span className="s">
          <span>{roleWords[agent.role]}</span>
          <span className={status === "waiting" ? "you-text" : undefined}>{status.replaceAll("_", " ")}</span>
          {model ? <span className="mono">{model}</span> : null}
        </span>
      </span>
      <ChevronRight size={14} aria-hidden />
    </button>
  );
}

function ItemList({
  items,
  pages,
  events,
  names,
  selected,
  onSelect,
}: {
  items: ItemRow[];
  pages: Map<string, ItemPage>;
  events: HistoryEvent[];
  names: (identity: string | null) => string | null;
  selected: string | null;
  onSelect: (id: string) => void;
}) {
  const now = Date.now();
  return (
    <div className="list mc-items" role="listbox" aria-label="Items">
      {items.map((row) => {
        const at = stageAt(row.id, events, pages.get(row.id) ?? null);
        const stuck = MOVING.has(row.status) && at !== null && now - Date.parse(at) > STALE_MS;
        const holder = row.status === "in_review" ? (row.reviewer ? `review: ${names(row.reviewer)}` : "no reviewer yet") : names(row.holder);
        const live = MOVING.has(row.status);
        return (
          <button
            key={row.id}
            type="button"
            role="option"
            aria-selected={row.id === selected}
            className={`mc-item${row.id === selected ? " on" : ""}${stuck ? " stuck" : ""}${live ? " live" : ""}`}
            onClick={() => onSelect(row.id)}
          >
            <StatusGlyph status={row.status} />
            <span className="grow">
              <span className="t">
                <span className="id">{row.id}</span> {row.title}
              </span>
              <span className="s">
                <span className={`stage-word ${row.status}`}>{stage(row, events)}</span>
                {holder ? <span className="who">{holder}</span> : null}
                {row.blockers.some((b) => row.waiting_on.includes(b.id)) ? <span>waits on {row.waiting_on.join(", ")}</span> : null}
                {row.needs_human ? <span className="you-text">needs you</span> : null}
              </span>
            </span>
            <span className="age" title={at ? `In this stage since ${at}` : undefined}>
              {stuck ? (
                <span className="tag bad">
                  <Clock size={11} aria-hidden /> {since(at)}
                </span>
              ) : (
                since(at)
              )}
            </span>
          </button>
        );
      })}
    </div>
  );
}

function SelectedItem({ repo, row, page, events, infos }: { repo: string; row: ItemRow; page: ItemPage | null; events: HistoryEvent[]; infos: Map<string, ThreadInfo> }) {
  const pr = page?.pull_request ?? null;
  const checks = useData<Checks>(pr ? api.checks(repo, pr.number) : null, 60_000);
  const ticked = page?.done_when.filter((d) => d.ticked).length ?? 0;
  const threads = (page?.threads ?? []).filter((a) => a.role === "implementer" || a.role === "reviewer");
  const reviews = page?.reviews ?? [];
  return (
    <section className="mc-spec">
      <header>
        <StatusGlyph status={row.status} />
        <div className="grow">
          <h2>
            <Link to={to.item(repo, row.id)}>{row.title}</Link>
          </h2>
          <p className="mc-sub">
            <span className="mono">{row.id}</span> · {stage(row, events)}
            {row.holder ? ` · held by ${infos.get(row.holder)?.title ?? who(null, row.holder)}` : ""}
          </p>
        </div>
      </header>
      {!page ? <Skeleton rows={3} /> : null}
      {page?.goal ? (
        <div className="mc-block">
          <h3>Goal</h3>
          <Markdown text={page.goal} small />
        </div>
      ) : null}
      {page && page.done_when.length > 0 ? (
        <div className="mc-block">
          <h3>
            Done when <span className="tally">{ticked}/{page.done_when.length}</span>
          </h3>
          <ul className="ticks">
            {page.done_when.map((d, at) => (
              <li key={at} className={d.ticked ? "on" : undefined}>
                {d.ticked ? <Check size={13} aria-label="Ticked" /> : <Square size={13} aria-label="Not ticked" />}
                <span>{d.text}</span>
              </li>
            ))}
          </ul>
        </div>
      ) : null}
      {page ? (
        <div className="mc-block">
          <h3>Pull request</h3>
          {pr ? (
            <p className="mc-pr">
              <a href={pr.url} target="_blank" rel="noreferrer">
                <GitPullRequest size={13} aria-hidden /> #{pr.number} {pr.state.toLowerCase()} <ExternalLink size={11} aria-hidden />
              </a>
              {checks.data ? (
                <span className={`ci ${checks.data.state}`} title={checks.data.checks.map((c) => `${c.name}: ${c.state}`).join("\n")}>
                  CI {checks.data.state}
                  {checks.data.checks.length > 0 ? ` (${checks.data.checks.filter((c) => c.state === "SUCCESS").length} passed, ${checks.data.checks.filter((c) => c.state === "SKIPPED").length} skipped)` : ""}
                </span>
              ) : checks.error ? (
                <span className="ci none">CI unknown</span>
              ) : null}
            </p>
          ) : (
            <p className="mc-muted">{row.branch ? "No pull request found for its branch." : "No branch yet."}</p>
          )}
          {reviews.length > 0 ? (
            <p className="mc-muted">
              Reviews:{" "}
              {reviews.map((r, at) => (
                <span key={r.name}>
                  {at > 0 ? ", " : ""}round {r.round} {r.verdict ? r.verdict.replaceAll("_", " ") : "open"}
                  {r.model ? ` (${r.model})` : ""}
                </span>
              ))}
            </p>
          ) : null}
        </div>
      ) : null}
      {threads.length > 0 ? (
        <div className="mc-block">
          <h3>
            Threads <span className="tally">{threads.length}</span>
          </h3>
          <div className="list">
            {threads.map((a, at) => (
              <ThreadLine key={`${a.identity}${a.role}${at}`} agent={a} info={infos.get(a.identity)} />
            ))}
          </div>
        </div>
      ) : null}
    </section>
  );
}

type LogEntry = { at: string; icon: typeof Play; text: string; id: string; waited: boolean; tone: string };

function logEntries(mission: MissionPage, events: HistoryEvent[], pages: Map<string, ItemPage>, caller: string): LogEntry[] {
  const title = (id: string) => (id === mission.mission.id ? "the plan" : id);
  const needsYou = new Set(mission.items.filter((i) => i.needs_human).map((i) => i.id));
  // An item the record says the human readied gets one entry from the
  // record below; the commit that readied it adds nothing.
  const recorded = new Set([...pages].filter(([, p]) => p.ready?.by.toLowerCase().startsWith(caller)).map(([id]) => id));
  const entries: LogEntry[] = events.filter((e) => !(e.kind === "readied" && recorded.has(e.id))).map((e) => {
    const plan = e.id === mission.mission.id;
    const waited = needsYou.has(e.id);
    switch (e.kind) {
      case "planned":
        return { at: e.at, icon: Target, text: "Mission planned", id: e.id, waited, tone: "plain" };
      case "plan_review":
        return { at: e.at, icon: e.verdict === "approved" ? ThumbsUp : ThumbsDown, text: `Plan review, round ${e.round}: ${e.verdict}`, id: e.id, waited, tone: e.verdict === "approved" ? "ok" : "bad" };
      case "started":
        return { at: e.at, icon: Play, text: `Started ${title(e.id)}`, id: e.id, waited, tone: "run" };
      case "submitted":
        return { at: e.at, icon: Send, text: `Submitted ${title(e.id)} for review`, id: e.id, waited, tone: "review" };
      case "review":
        return { at: e.at, icon: e.verdict === "approved" ? ThumbsUp : ThumbsDown, text: `Review of ${e.id}, round ${e.round}: ${e.verdict}`, id: e.id, waited, tone: e.verdict === "approved" ? "ok" : "bad" };
      case "merged":
        return { at: e.at, icon: GitMerge, text: plan ? "Plan merged: the mission is agreed" : `Merged ${e.id}`, id: e.id, waited, tone: "ok" };
      case "closed":
        return { at: e.at, icon: CircleCheck, text: plan ? "Mission closed with its result" : `Closed ${e.id}`, id: e.id, waited, tone: "ok" };
      case "readied":
        return { at: e.at, icon: Hand, text: `Readied ${title(e.id)}`, id: e.id, waited, tone: "plain" };
      case "dropped":
        return { at: e.at, icon: CircleCheck, text: plan ? "Mission dropped" : `Dropped ${e.id}`, id: e.id, waited, tone: "dim" };
      case "led":
        return { at: e.at, icon: User, text: "A lead took the mission", id: e.id, waited, tone: "plain" };
      default:
        return { at: e.at, icon: MessageSquare, text: e.subject, id: e.id, waited, tone: "plain" };
    }
  });
  // An item readied by the human, as its record says, is a step that waited on him.
  const readied = new Map<string, { by: string; at: string; ids: string[] }>();
  for (const [id, page] of pages) {
    if (!page.ready || !page.ready.by.toLowerCase().startsWith(caller)) continue;
    const key = `${page.ready.by}\n${page.ready.at.slice(0, 16)}`;
    const group = readied.get(key) ?? { by: page.ready.by, at: page.ready.at, ids: [] };
    group.ids.push(id);
    readied.set(key, group);
  }
  for (const group of readied.values()) {
    const said = group.by.includes(":") ? ` ("${group.by.split(":").slice(1).join(":").trim()}")` : "";
    entries.push({ at: group.at, icon: Hand, text: `Sumeet readied ${group.ids.length === 1 ? group.ids[0] : `${group.ids.length} items`}${said}`, id: group.ids[0], waited: true, tone: "you" });
  }
  return entries.sort((a, b) => Date.parse(b.at) - Date.parse(a.at));
}

const LOG_SHOWN = 40;

function Log({ entries: all, onSelect }: { entries: LogEntry[]; onSelect: (id: string) => void }) {
  const [more, setMore] = useState(false);
  const entries = more ? all : all.slice(0, LOG_SHOWN);
  let day = "";
  return (
    <section className="mc-log" aria-label="Progress log">
      <h3>
        Progress log <span className="tally">{all.length}</span>
      </h3>
      {entries.length === 0 ? <p className="mc-muted">No recorded steps yet.</p> : null}
      <ol>
        {entries.map((e, at) => {
          const d = new Date(e.at);
          const thisDay = d.toLocaleDateString(undefined, { weekday: "short", month: "short", day: "numeric" });
          const head = thisDay !== day ? thisDay : null;
          day = thisDay;
          const Icon = e.icon;
          return (
            <li key={at} className={e.waited ? "waited" : undefined}>
              {head ? <div className="mc-day">{head}</div> : null}
              <button type="button" className="mc-entry" onClick={() => onSelect(e.id)}>
                <span className={`glyph ${e.tone}`}>
                  <Icon size={13} aria-hidden />
                </span>
                <span className="txt">
                  {e.text}
                  {e.waited ? <span className="tag you">waited on you</span> : null}
                </span>
                <time dateTime={e.at}>{d.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" })}</time>
              </button>
            </li>
          );
        })}
      </ol>
      {all.length > entries.length ? (
        <button type="button" className="fold" onClick={() => setMore(true)}>
          Show {all.length - entries.length} older steps
        </button>
      ) : null}
    </section>
  );
}

/** `/mission/:repo/:id`: one mission as a control room: its items, the selected item, its log. */
export function MissionControlPage() {
  const { repo = "", id = "" } = useParams();
  const [params, setParams] = useSearchParams();
  const loaded = useData<MissionPage>(api.mission(repo, id));
  const mission = loaded.data;
  const items = mission?.items ?? [];
  const ids = [id, ...items.map((i) => i.id)];
  const history = useData<History>(mission ? api.history(repo, { ids }) : null);
  const pagesRead = useAll<ItemPage>(items.map((i) => api.item(repo, i.id)));
  const pages = useMemo(() => {
    const map = new Map<string, ItemPage>();
    pagesRead.data.forEach((p, at) => p && items[at] && map.set(items[at].id, p));
    return map;
  }, [pagesRead.data, items]);
  const identities = [
    ...(mission?.threads ?? []).map((a) => a.identity),
    ...items.flatMap((i) => [i.holder, i.reviewer]).filter((x): x is string => !!x),
    ...[...pages.values()].flatMap((p) => p.threads.map((a) => a.identity)),
  ];
  const infos = useThreads(identities);
  const events = history.data?.events ?? [];

  if (!mission) {
    return (
      <div className="page mc-page">
        <Crumbs trail={[{ label: repo, to: to.home(repo) }, { label: id }]} />
        <ErrorBanner loaded={loaded} />
        {loaded.loading ? <Skeleton rows={8} /> : null}
      </div>
    );
  }

  const names = (identity: string | null) => {
    if (!identity) return null;
    const live = [...pages.values()].flatMap((p) => p.threads).find((a) => a.identity === identity && a.thread);
    return live?.thread?.title ?? infos.get(identity)?.title ?? who(null, identity);
  };
  const live = items.filter((i) => MOVING.has(i.status));
  const asked = params.get("item");
  const selected = items.find((i) => i.id === asked) ?? live[0] ?? items.find((i) => i.status !== "done" && i.status !== "dropped") ?? items[0] ?? null;
  const select = (item: string) => {
    if (!items.some((i) => i.id === item)) return;
    const next = new URLSearchParams(params);
    next.set("item", item);
    setParams(next, { replace: true });
  };
  const lead = mission.threads.find((a) => a.role === "lead") ?? mission.threads.find((a) => a.role === "planner") ?? null;
  const counts = mission.mission.counts;
  const total = counts.reduce((n, c) => n + c.count, 0);
  const done = counts.filter((c) => c.status === "done").reduce((n, c) => n + c.count, 0);
  const merged = events.find((e) => e.id === id && e.kind === "merged");
  const state = mission.mission.status === "done" ? "Done" : mission.mission.in_progress ? "In progress" : stateWords[mission.mission.state];
  const entries = logEntries(mission, events, pages, "sumeet");
  const stuck = live.filter((i) => {
    const at = stageAt(i.id, events, pages.get(i.id) ?? null);
    return at !== null && Date.now() - Date.parse(at) > STALE_MS;
  });

  return (
    <div className="page mc-page">
      <Crumbs trail={[{ label: repo, to: to.home(repo) }, { label: mission.mission.title }]} />
      <header className="mc-head">
        <div className="grow">
          <div className="mc-kicker">
            <span className={`pill-state ${mission.mission.in_progress ? "run" : mission.mission.status === "done" ? "ok" : "plain"}`}>{state}</span>
            <span className="mono">{id}</span>
            {merged ? <span>plan merged {since(merged.at)} ago</span> : mission.mission.state === "proposed" ? <span>plan not merged yet</span> : null}
            {live.length > 0 ? <span>{live.length} live</span> : null}
            {stuck.length > 0 ? <span className="bad-text">{stuck.length} quiet over 30 minutes</span> : null}
          </div>
          <h1>{mission.mission.title}</h1>
          <div className="mc-progress">
            <ScopeBar counts={counts} tall />
            <span className="num">
              {done}/{total} done
            </span>
            <Link to={to.mission(repo, id)} className="mc-link">
              Plan and documents <ChevronRight size={13} aria-hidden />
            </Link>
          </div>
        </div>
        {lead ? (
          <button type="button" className="mc-talk" onClick={() => openThread(lead.identity)} title={lead.identity}>
            <MessageSquare size={15} aria-hidden />
            <span>
              Talk to the {lead.role === "lead" ? "lead" : "planner"}
              <small>{lead.thread?.title ?? infos.get(lead.identity)?.title ?? who(null, lead.identity)}</small>
            </span>
          </button>
        ) : null}
      </header>

      <div className="mc-grid">
        <section className="mc-col">
          <h3 className="mc-h">
            Items <span className="tally">{items.length}</span>
          </h3>
          <ItemList items={items} pages={pages} events={events} names={names} selected={selected?.id ?? null} onSelect={select} />
        </section>
        <section className="mc-col">{selected ? <SelectedItem repo={repo} row={selected} page={pages.get(selected.id) ?? null} events={events} infos={infos} /> : <p className="mc-muted">This mission has no items yet.</p>}</section>
        <Log entries={entries} onSelect={select} />
      </div>
    </div>
  );
}
