// The project at a glance, which This thread shows for a thread that holds, started and names
// nothing (desk/src/components/Glance.tsx, ProjectGlance): what waits on the human, the agreed
// missions, the items in motion and what can start next. A read that fails leaves its blocks out
// and is named.
import { GitPullRequestArrow, Target } from "lucide-react";
import type { ReactNode } from "react";
import type { Agent } from "../../../desk/src/api/Agent";
import type { InboxResponse } from "../../../desk/src/api/InboxResponse";
import type { MissionsResponse } from "../../../desk/src/api/MissionsResponse";
import type { MotionResponse } from "../../../desk/src/api/MotionResponse";
import type { ThreadInfo } from "../../../desk/src/api/ThreadInfo";
import type { RequestError } from "../host/api";
import { useDeskReads } from "../react/desk";
import { Badge } from "../t3/ui/badge";
import * as m from "./model";
import { Heading, Row, StatusDot } from "./parts";
import type { ThreadActions } from "./ThisThread";

const READ_MS = 30_000;

export function ProjectGlance({ desk, repo, now, actions }: { desk: string; repo: string; now: number; actions: ThreadActions }) {
  const missionsRead = useDeskReads<MissionsResponse>(desk, [m.paths.missions(repo)], READ_MS).get(m.paths.missions(repo));
  const motionRead = useDeskReads<MotionResponse>(desk, [m.paths.motion()], READ_MS).get(m.paths.motion());
  const inboxRead = useDeskReads<InboxResponse>(desk, [m.paths.inbox()], READ_MS).get(m.paths.inbox());
  const moving = motionRead?.data?.repos.find((r) => r.repo === repo);
  const waiting = inboxRead?.data?.repos.find((r) => r.repo === repo);
  const moved = moving?.items ?? [];
  // The holders T3 Code does not list as live, named by their title where T3 Code still knows them.
  const unlisted = [
    ...new Set(
      moved
        .flatMap((i) => [i.holder, i.reviewer])
        .filter((a): a is Agent => !!a && !a.thread)
        .map((a) => a.identity),
    ),
  ];
  const infos = useDeskReads<ThreadInfo>(desk, unlisted.map(m.paths.thread), READ_MS);
  const name = (agent: Agent | null) => (agent ? (agent.thread?.title ?? infos.get(m.paths.thread(agent.identity))?.data?.title ?? m.shortName(agent.identity)) : "nobody");

  if ((missionsRead?.error as RequestError | undefined)?.status === 404) return <p className="text-muted-foreground text-xs">The Desk does not serve a repository named {repo}.</p>;
  if (!missionsRead || !motionRead || !inboxRead) return <p className="text-muted-foreground text-xs">Reading {repo}…</p>;
  const failed: (readonly [string, string, { error: Error | null }])[] = [
    ["the missions", m.paths.missions(repo), missionsRead],
    ["what is in motion", m.paths.motion(), motionRead],
    ["the inbox", m.paths.inbox(), inboxRead],
    // A holder's title: a thread T3 Code does not know (404) keeps its short name and is not a failure.
    ...unlisted.flatMap((identity) => {
      const read = infos.get(m.paths.thread(identity));
      return read?.error && (read.error as RequestError).status !== 404 ? [[`the title of ${m.shortName(identity)}`, m.paths.thread(identity), read] as const] : [];
    }),
  ];

  const active = (missionsRead.data?.missions ?? []).filter((mission) => mission.state === "open").sort((a, b) => Number(b.in_progress) - Number(a.in_progress));
  const needs = (waiting?.needs_human ?? []).flatMap((g) => g.items);
  const ready = (moving?.ready ?? []).filter((r) => !r.needs_human);
  const plans = waiting?.proposed ?? [];
  const threads = waiting?.threads ?? [];
  const waits = needs.length + plans.length + threads.length;
  const fetched = moving?.fetched.at ? m.since(moving.fetched.at, now) : null;

  return (
    <div className="flex flex-col" data-glance={repo}>
      {failed.map(([what, path, read]) =>
        read.error ? (
          <p key={path} className="text-destructive-foreground text-xs" data-note="">
            Could not read {what} (GET {path}): {read.error.message}
          </p>
        ) : null,
      )}
      {inboxRead.data && waits > 0 ? (
        <Block name="waiting" title="Waiting on you" count={waits} warn>
          {plans.map((mission) => (
            <Row key={mission.id} id={mission.id} row={`mission:${mission.id}`} title={mission.title} glyph={<GitPullRequestArrow className="size-3.5 text-warning-foreground" aria-hidden />} note={<Badge variant="warning">Plan proposed</Badge>} onClick={() => actions.openMission(repo, mission.id, true)} />
          ))}
          {threads.map((t) => (
            <Row key={t.id} id={m.threadStatus(t.status)} row={`thread:${t.id}`} title={t.title} glyph={<span className="size-1.5 justify-self-center rounded-full bg-warning" aria-hidden />} note={<span>{t.holds.map((h) => h.id).join(", ")}</span>} onClick={() => actions.openThread(t.id)} />
          ))}
          {needs.map((row) => (
            <Row key={row.id} id={row.id} row={`item:${row.id}`} title={row.title} glyph={<StatusDot status={row.status} />} note={<Badge variant="warning">Needs you</Badge>} onClick={() => actions.openItem(repo, row.id)} />
          ))}
        </Block>
      ) : null}
      {missionsRead.data ? (
        <Block name="missions" title="Missions" count={active.length}>
          {active.length === 0 ? <p className="px-1.5 text-muted-foreground text-xs">No mission is agreed and open.</p> : null}
          {active.map((mission) => (
            <Row
              key={mission.id}
              id={mission.id}
              row={`mission:${mission.id}`}
              title={mission.title}
              glyph={<Target className={`size-3.5 ${mission.in_progress ? "text-info-foreground" : "text-muted-foreground"}`} aria-hidden />}
              note={
                <span className="tabular-nums" title={m.countsLine(mission.counts)}>
                  {m.finished(mission.counts)}/{m.total(mission.counts)}
                </span>
              }
              onClick={() => actions.openMission(repo, mission.id, false)}
            />
          ))}
        </Block>
      ) : null}
      {motionRead.data ? (
        <>
          <Block name="motion" title="In motion" count={moved.length}>
            {moved.length === 0 ? <p className="px-1.5 text-muted-foreground text-xs">Nothing is started.</p> : null}
            {moved.map((item) => {
              const reviewing = item.status === "in_review" && item.reviewer;
              return (
                <Row
                  key={item.id}
                  id={item.id}
                  row={`item:${item.id}`}
                  title={item.title}
                  glyph={<StatusDot status={item.status} />}
                  note={<span className="max-w-40 truncate">{reviewing ? `Reviewed by ${name(item.reviewer)}` : `${m.statusName(item.status)}, ${name(item.holder)}`}</span>}
                  onClick={() => actions.openItem(repo, item.id)}
                />
              );
            })}
          </Block>
          <Block name="ready" title="Next up" count={ready.length}>
            {ready.length === 0 ? <p className="px-1.5 text-muted-foreground text-xs">Nothing is ready to start.</p> : null}
            {ready.map((row) => (
              <Row key={row.id} id={row.id} row={`item:${row.id}`} title={row.title} glyph={<StatusDot status={row.status} />} note={<span>{row.priority}</span>} onClick={() => actions.openItem(repo, row.id)} />
            ))}
          </Block>
        </>
      ) : null}
      <p className="mt-3 text-muted-foreground text-xs">{fetched ? `${repo}, fetched ${fetched === "now" ? "just now" : `${fetched} ago`}` : repo}</p>
    </div>
  );
}

function Block({ name, title, count, warn, children }: { name: string; title: string; count: number; warn?: boolean; children: ReactNode }) {
  return (
    <section data-section={name}>
      <Heading warn={warn} count={count}>
        {title}
      </Heading>
      {children}
    </section>
  );
}
