import { ChevronDown, ChevronRight, CircleCheck, GitMerge, Play, Target } from "lucide-react";
import { useState, type ReactNode } from "react";
import { Link, useParams } from "react-router";
import type { ItemRow } from "../api/ItemRow";
import type { MissionPage } from "../api/MissionPage";
import type { MissionRow } from "../api/MissionRow";
import type { MissionsResponse } from "../api/MissionsResponse";
import type { HistoryEvent } from "../api/HistoryEvent";
import type { HistoryResponse } from "../api/HistoryResponse";
import type { RoadmapResponse } from "../api/RoadmapResponse";
import { Crumbs } from "../components/Crumbs";
import { sinceAgo, who } from "../components/Glance";
import { ErrorBanner, Priority, ScopeBar, Skeleton, StatusGlyph } from "../components/ui";
import { api, to, useData, useThreads } from "../lib/api";
import { statusName } from "../lib/words";

const STARTED = new Set(["building", "in_review", "changes_requested", "approved"]);

function blockedLine(row: ItemRow): string | null {
  const open = row.blockers.filter((b) => row.waiting_on.includes(b.id));
  if (open.length === 0) return null;
  return `blocked by ${open.map((b) => `${b.id} (${b.status ? statusName(b.status) : "not found"})`).join(", ")}`;
}

function ItemLineSmall({ repo, row, names }: { repo: string; row: ItemRow; names: (id: string | null) => string | null }) {
  const blocked = blockedLine(row);
  const finished = row.status === "done" || row.status === "dropped";
  const holder = finished ? null : row.status === "in_review" ? (row.reviewer ? `review: ${names(row.reviewer)}` : null) : names(row.holder);
  return (
    <Link to={to.item(repo, row.id)} className={`rm-item${blocked ? " blocked" : ""}`}>
      <StatusGlyph status={row.status} />
      <span className="id">{row.id}</span>
      <span className="title">{row.title}</span>
      <span className="note">
        {blocked ? <span className="blk">{blocked}</span> : null}
        {holder ? <span className="who">{holder}</span> : null}
        {row.needs_human ? <span className="tag you">Needs you</span> : null}
        <span className={`stage-word ${row.status}`}>{statusName(row.status)}</span>
      </span>
    </Link>
  );
}

function MissionItems({ repo, id }: { repo: string; id: string }) {
  const page = useData<MissionPage>(api.mission(repo, id));
  const items = page.data?.items ?? [];
  const infos = useThreads(items.flatMap((i) => [i.holder, i.reviewer]).filter((x): x is string => !!x));
  const names = (identity: string | null) => (identity ? (infos.get(identity)?.title ?? who(null, identity)) : null);
  if (!page.data) return <Skeleton rows={2} />;
  if (items.length === 0) return <p className="rm-empty">No items yet.</p>;
  // Open work first, in lifecycle order; finished items last.
  const order = ["building", "in_review", "changes_requested", "approved", "ready", "draft", "done", "dropped"];
  const sorted = [...items].sort((a, b) => order.indexOf(a.status) - order.indexOf(b.status));
  return (
    <div className="rm-items">
      {sorted.map((row) => (
        <ItemLineSmall key={row.id} repo={repo} row={row} names={names} />
      ))}
    </div>
  );
}

function MissionLine({ repo, m, open: startOpen }: { repo: string; m: MissionRow; open: boolean }) {
  const [open, setOpen] = useState(startOpen);
  const total = m.counts.reduce((n, c) => n + c.count, 0);
  const done = m.counts.filter((c) => c.status === "done").reduce((n, c) => n + c.count, 0);
  return (
    <div className={`rm-mission${open ? " open" : ""}`}>
      <div className="rm-mrow">
        <button type="button" className="rm-toggle" aria-expanded={open} aria-label={open ? "Fold its items" : "Show its items"} onClick={() => setOpen(!open)}>
          {open ? <ChevronDown size={15} /> : <ChevronRight size={15} />}
        </button>
        <span className={`glyph ${m.in_progress ? "run" : m.state === "proposed" ? "you" : "plain"}`}>
          <Target size={15} aria-hidden />
        </span>
        <Link to={to.control(repo, m.id)} className="rm-title" title="Open mission control">
          <span className="t">{m.title}</span>
          <span className="id">{m.id}</span>
        </Link>
        {m.waiting.length > 0 ? <span className="tag you">{m.waiting.length} need you</span> : null}
        <span className="rm-bar">
          <ScopeBar counts={m.counts} />
        </span>
        <span className="num">
          {done}/{total}
        </span>
      </div>
      {open ? <MissionItems repo={repo} id={m.id} /> : null}
    </div>
  );
}

function Lane({ title, hint, count, children }: { title: string; hint: string; count: number; children: ReactNode }) {
  return (
    <section className="rm-lane">
      <div className="rm-lane-head">
        <h2>{title}</h2>
        <span className="tally">{count}</span>
        <span className="hint">{hint}</span>
      </div>
      {count === 0 ? <p className="rm-empty">Nothing here.</p> : <div className="list">{children}</div>}
    </section>
  );
}

function LooseItems({ repo, rows, limit = 8 }: { repo: string; rows: ItemRow[]; limit?: number }) {
  const [all, setAll] = useState(false);
  const shown = all ? rows : rows.slice(0, limit);
  if (rows.length === 0) return null;
  return (
    <div className="rm-loose">
      <div className="rm-loose-head">In no mission</div>
      {shown.map((row) => (
        <Link key={row.id} to={to.item(repo, row.id)} className="rm-item">
          <StatusGlyph status={row.status} />
          <span className="id">{row.id}</span>
          <span className="title">{row.title}</span>
          <span className="note">
            {blockedLine(row) ? <span className="blk">{blockedLine(row)}</span> : null}
            {row.needs_human ? <span className="tag you">Needs you</span> : null}
            {STARTED.has(row.status) ? <span className={`stage-word ${row.status}`}>{statusName(row.status)}</span> : null}
            <Priority priority={row.priority} />
          </span>
        </Link>
      ))}
      {rows.length > shown.length ? (
        <button type="button" className="fold" onClick={() => setAll(true)}>
          Show {rows.length - shown.length} more
        </button>
      ) : null}
    </div>
  );
}

const WEEK_KINDS: Record<string, { icon: typeof Play; tone: string; word: (e: HistoryEvent) => string | null }> = {
  started: { icon: Play, tone: "run", word: (e) => (e.record === "item" ? "Started" : null) },
  merged: { icon: GitMerge, tone: "ok", word: (e) => (e.record === "mission" ? "Plan agreed" : e.record === "item" ? "Merged" : null) },
  closed: { icon: CircleCheck, tone: "ok", word: (e) => (e.record === "mission" ? "Mission done" : null) },
};

type DayEntry = { id: string; title: string; words: string[]; tone: string; icon: typeof Play; record: HistoryEvent["record"]; at: string };

const DAY_SHOWN = 6;

function Day({ repo, label, entries }: { repo: string; label: string; entries: DayEntry[] }) {
  const [all, setAll] = useState(false);
  const shown = all ? entries : entries.slice(0, DAY_SHOWN);
  return (
    <div className={`rm-day${entries.length === 0 ? " quiet" : ""}`}>
      <div className="rm-day-head">
        {label} <span className="tally">{entries.length || ""}</span>
      </div>
      {shown.map((e) => {
        const Icon = e.icon;
        const href = e.record === "mission" ? to.control(repo, e.id) : to.item(repo, e.id);
        return (
          <Link key={e.id} to={href} className="rm-ev" title={`${e.words.join(" and ")} ${e.id}: ${e.title}`}>
            <span className={`glyph ${e.tone}`}>
              <Icon size={12} aria-hidden />
            </span>
            <span className="txt">
              <b>{e.words.map((w, at) => (at === 0 ? w : w.toLowerCase())).join(" and ")}</b> {e.title}
            </span>
          </Link>
        );
      })}
      {entries.length > shown.length ? (
        <button type="button" className="fold" onClick={() => setAll(true)}>
          +{entries.length - shown.length} more
        </button>
      ) : null}
    </div>
  );
}

function ThisWeek({ repo, events }: { repo: string; events: HistoryEvent[] }) {
  const days: { key: string; label: string; entries: DayEntry[] }[] = [];
  for (let back = 6; back >= 0; back--) {
    const d = new Date();
    d.setHours(0, 0, 0, 0);
    d.setDate(d.getDate() - back);
    days.push({ key: d.toDateString(), label: back === 0 ? "Today" : d.toLocaleDateString(undefined, { weekday: "short", day: "numeric" }), entries: [] });
  }
  // Oldest first, so a day reads in order and an item started and merged
  // the same day is one line.
  for (const e of [...events].reverse()) {
    const kind = WEEK_KINDS[e.kind];
    const word = kind?.word(e);
    if (!kind || !word) continue;
    const day = days.find((d) => d.key === new Date(e.at).toDateString());
    if (!day) continue;
    const same = day.entries.find((x) => x.id === e.id);
    if (same) {
      if (!same.words.includes(word)) same.words.push(word);
      same.tone = kind.tone;
      same.icon = kind.icon;
      continue;
    }
    day.entries.push({ id: e.id, title: e.title ?? e.id, words: [word], tone: kind.tone, icon: kind.icon, record: e.record, at: e.at });
  }
  // Finished work first: merges and agreed plans are what the week produced.
  for (const d of days) d.entries.sort((a, b) => Number(b.tone === "ok") - Number(a.tone === "ok"));
  const count = days.reduce((n, d) => n + d.entries.length, 0);
  return (
    <section className="rm-week">
      <div className="rm-lane-head">
        <h2>This week</h2>
        <span className="tally">{count}</span>
        <span className="hint">items started and merged, plans agreed, from the record history</span>
      </div>
      <div className="rm-days">
        {days.map((d) => (
          <Day key={d.key} repo={repo} label={d.label} entries={d.entries} />
        ))}
      </div>
    </section>
  );
}

/** `/home/:repo`: the project home, a roadmap without dates: now, next, later, and what happened this week. */
export function ProjectHomePage() {
  const { repo = "" } = useParams();
  const missions = useData<MissionsResponse>(api.missions(repo));
  const roadmap = useData<RoadmapResponse>(api.roadmap(repo));
  const week = useData<HistoryResponse>(api.history(repo, { days: 7 }));
  const rows = missions.data?.missions ?? [];
  const loose = roadmap.data?.loose ?? [];
  // A mission with any item started or done is under way.
  const begun = (m: MissionRow) => m.in_progress || m.counts.some((c) => c.status !== "draft" && c.status !== "ready" && c.status !== "dropped" && c.count > 0);
  const now = rows.filter((m) => m.state === "open" && begun(m));
  const next = rows.filter((m) => m.state === "open" && !begun(m));
  const later = rows.filter((m) => m.state === "proposed");
  const looseNow = loose.filter((i) => STARTED.has(i.status));
  const looseNext = loose.filter((i) => i.status === "ready");
  const looseLater = loose.filter((i) => i.status === "draft");
  return (
    <div className="page rm-page">
      <Crumbs trail={[{ label: repo }]} />
      <ErrorBanner loaded={missions} />
      {!missions.data || !roadmap.data ? (
        <Skeleton rows={8} />
      ) : (
        <>
          <div className="rm-lanes">
            <Lane title="Now" hint="missions in progress, and started work in no mission" count={now.length + looseNow.length}>
              {now.map((m) => (
                <MissionLine key={m.id} repo={repo} m={m} open />
              ))}
              <LooseItems repo={repo} rows={looseNow} />
            </Lane>
            <Lane title="Next" hint="agreed, nothing started, and ready work" count={next.length + looseNext.length}>
              {next.map((m) => (
                <MissionLine key={m.id} repo={repo} m={m} open={now.length === 0 && next.length <= 2} />
              ))}
              <LooseItems repo={repo} rows={looseNext} />
            </Lane>
            <Lane title="Later" hint="proposed plans and drafts" count={later.length + looseLater.length}>
              {later.map((m) => (
                <MissionLine key={m.id} repo={repo} m={m} open={false} />
              ))}
              <LooseItems repo={repo} rows={looseLater} limit={5} />
            </Lane>
          </div>
          {week.data ? <ThisWeek repo={repo} events={week.data.events} /> : null}
          <p className="glance-foot">{missions.data.fetched.at ? `Fetched ${sinceAgo(missions.data.fetched.at)}.` : null}</p>
        </>
      )}
    </div>
  );
}
