/**
 * The pieces of the views a host shows beside a thread: rows, blocks, short
 * times, and the project at a glance.
 */
import { GitPullRequestArrow, Target } from "lucide-react";
import type { ReactNode } from "react";
import { Link } from "react-router";
import type { Agent } from "../api/Agent";
import type { InboxResponse } from "../api/InboxResponse";
import type { MissionsResponse } from "../api/MissionsResponse";
import type { MotionResponse } from "../api/MotionResponse";
import type { ItemRow } from "../api/ItemRow";
import type { MotionItem } from "../api/MotionItem";
import { LiveThreadRow } from "./Agents";
import { ErrorBanner, Priority, ScopeBar, Skeleton, StatusGlyph } from "./ui";
import { api, settled, to, useData, useThreads } from "../lib/api";
import { ago, countsLine, finished, statusName, total } from "../lib/words";

/** A thread as a short name: its live title, else the end of its identity. */
export function who(agent: Agent | null | undefined, identity?: string | null): string {
  if (agent?.thread) return agent.thread.title;
  const id = agent?.identity ?? identity;
  if (!id) return "nobody";
  const tail = id.split(/%3A|:/).at(-1) ?? id;
  return `thread …${tail.slice(-6)}`;
}

/** `12m`, `3h`, `2d`, or `now` within a minute: how long ago, short. */
export function since(iso: string | null | undefined, now = Date.now()): string | null {
  if (!iso) return null;
  const at = Date.parse(iso);
  if (Number.isNaN(at)) return null;
  const s = Math.max(0, (now - at) / 1000);
  if (s < 60) return "now";
  if (s < 3600) return `${Math.round(s / 60)}m`;
  if (s < 172_800) return `${Math.round(s / 3600)}h`;
  return `${Math.round(s / 86_400)}d`;
}

/** `12m ago`, or `now`: how long ago, as a phrase; empty without a time. */
export function sinceAgo(iso: string | null | undefined, now = Date.now()): string {
  const s = since(iso, now);
  return s === null ? "" : s === "now" ? "now" : `${s} ago`;
}

/** A titled block of rows, with a tally and a link to more. */
export function Block({ title, tally, more, children }: { title: string; tally?: ReactNode; more?: ReactNode; children: ReactNode }) {
  return (
    <section className="glance-block">
      <div className="glance-head">
        <h3>{title}</h3>
        {tally !== undefined ? <span className="tally">{tally}</span> : null}
        {more ? <span className="more">{more}</span> : null}
      </div>
      <div className="list">{children}</div>
    </section>
  );
}

/** One item as a compact row: glyph, ID, title, and a short right-hand note. */
export function Row({ repo, id, title, status, note, hot, extra }: { repo: string; id: string; title: string; status: string; note?: ReactNode; hot?: boolean; extra?: ReactNode }) {
  return (
    <Link to={to.item(repo, id)} className={`glance-row${hot ? " hot" : ""}`}>
      <StatusGlyph status={status} />
      <span className="id">{id}</span>
      <span className="title">{title}</span>
      <span className="note">
        {extra}
        {note}
      </span>
    </Link>
  );
}

function MotionRow({ repo, item, names }: { repo: string; item: MotionItem; names: Map<string, string> }) {
  const age = since(item.changed_at);
  const name = (agent: Agent | null) => (agent?.thread ? agent.thread.title : agent ? (names.get(agent.identity) ?? who(agent)) : "nobody");
  const reviewing = item.status === "in_review" && item.reviewer;
  return (
    <Link to={to.item(repo, item.id)} className="glance-row two">
      <StatusGlyph status={item.status} />
      <span className="id">{item.id}</span>
      <span className="title">{item.title}</span>
      {age ? (
        <span className="age" title={`Last commit on ${item.branch ?? "its branch"} ${item.changed_at}`}>
          {age}
        </span>
      ) : (
        <span />
      )}
      <span className="sub">
        <span className="stage">{statusName(item.status)}</span>
        <span className="who" title={(reviewing ? item.reviewer : item.holder)?.identity ?? undefined}>
          {reviewing ? `Reviewed by ${name(item.reviewer)}` : `Held by ${name(item.holder)}`}
        </span>
      </span>
    </Link>
  );
}

function StartableRow({ repo, row }: { repo: string; row: ItemRow }) {
  return <Row repo={repo} id={row.id} title={row.title} status={row.status} extra={<Priority priority={row.priority} />} />;
}

/**
 * The project at a glance, for "This thread" when the thread holds, started
 * and names nothing: what waits on the human, missions agreed and open,
 * items in motion and what can start next. No heading of its own, and no
 * claim on the window's height.
 */
export function ProjectGlance({ repo }: { repo: string }) {
  const missions = useData<MissionsResponse>(api.missions(repo));
  const motion = useData<MotionResponse>(api.motion());
  const inbox = useData<InboxResponse>(api.inbox());
  const moving = motion.data?.repos.find((r) => r.repo === repo);
  const waiting = inbox.data?.repos.find((r) => r.repo === repo);
  const moved = moving?.items ?? [];
  const quiet = [
    ...new Set(
      moved
        .flatMap((i) => [i.holder, i.reviewer])
        .filter((a): a is Agent => !!a && !a.thread)
        .map((a) => a.identity),
    ),
  ];
  const infos = useThreads(quiet);
  const names = new Map([...infos].map(([id, info]) => [id, info.title]));
  if (missions.status === 404) return <p className="glance-empty">The Desk does not serve a repository named {repo}.</p>;
  // A read that fails leaves its blocks out, and the rest still draw.
  if (!settled(missions) || !settled(motion) || !settled(inbox)) return <Skeleton rows={4} />;

  // Agreed missions, those with work under way first.
  const active = (missions.data?.missions ?? []).filter((m) => m.state === "open").sort((a, b) => Number(b.in_progress) - Number(a.in_progress));
  const items = moved;
  const needs = (waiting?.needs_human ?? []).flatMap((g) => g.items);
  const startable = (moving?.ready ?? []).filter((r) => !r.needs_human);
  const plans = waiting?.proposed ?? [];
  const threads = waiting?.threads ?? [];
  const waits = needs.length + plans.length + threads.length;

  return (
    <div className="glance">
      {[missions, motion, inbox].some((read) => read.error) ? (
        <div className="glance-errors">
          <ErrorBanner loaded={missions} />
          <ErrorBanner loaded={motion} />
          <ErrorBanner loaded={inbox} />
        </div>
      ) : null}
      {waits > 0 ? (
        <Block title="Waiting on you" tally={waits}>
          {plans.map((m) => (
            <Link key={m.id} to={to.mission(repo, m.id)} className="glance-row hot">
              <span className="glyph you">
                <GitPullRequestArrow size={15} aria-hidden />
              </span>
              <span className="id">{m.id}</span>
              <span className="title">{m.title}</span>
              <span className="note">
                <span className="tag you">Plan proposed</span>
              </span>
            </Link>
          ))}
          {threads.map((t) => (
            <LiveThreadRow key={t.id} thread={t} holds={t.holds} />
          ))}
          {needs.map((row) => (
            <Row key={row.id} repo={repo} id={row.id} title={row.title} status={row.status} note={<span className="tag you">Needs you</span>} />
          ))}
        </Block>
      ) : null}

      {missions.data ? (
        <Block title="Missions" tally={active.length} more={<Link to={to.repo(repo)}>All missions</Link>}>
          {active.length === 0 ? <p className="card-empty">No mission is agreed and open.</p> : null}
          {active.map((m) => (
            <Link key={m.id} to={to.mission(repo, m.id)} className="glance-mission" title={m.in_progress ? "Work under way" : "Agreed, nothing started"}>
              <span className={`glyph ${m.in_progress ? "run" : "plain"}`}>
                <Target size={15} aria-hidden />
              </span>
              <span className="title">{m.title}</span>
              <span className="bar" title={countsLine(m.counts)}>
                <ScopeBar counts={m.counts} />
              </span>
              <span className="num">
                {finished(m.counts)}/{total(m.counts)}
              </span>
            </Link>
          ))}
        </Block>
      ) : null}

      {motion.data ? (
        <>
          <Block title="In motion" tally={items.length} more={<Link to="/motion">In motion</Link>}>
            {items.length === 0 ? <p className="card-empty">Nothing is started.</p> : null}
            {items.map((item) => (
              <MotionRow key={item.id} repo={repo} item={item} names={names} />
            ))}
          </Block>

          <Block title="Next up" tally={startable.length}>
            {startable.length === 0 ? <p className="card-empty">Nothing can start now.</p> : null}
            {startable.map((row) => (
              <StartableRow key={row.id} repo={repo} row={row} />
            ))}
          </Block>
        </>
      ) : null}
      <p className="glance-foot">{moving?.fetched.at ? `${repo}, fetched ${ago((Date.now() - Date.parse(moving.fetched.at)) / 1000)}` : repo}</p>
    </div>
  );
}
