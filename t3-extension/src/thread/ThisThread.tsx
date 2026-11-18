// This thread, drawn in T3's panel beside the thread T3 shows, in place of the Desk's framed page
// (desk/src/pages/ThisThread.tsx): what the thread holds, the work under it, the other threads it
// started, the records its conversation names, and what of that waits on the human; with none of
// these, its project at a glance. The thread, its children and the IDs it names come from the
// extension (src/panel/panel.ts); the records come from the Desk's reads. Its model is model.ts.
import { Check, ChevronRight, Clock, GitPullRequest, Inbox, Square } from "lucide-react";
import { useEffect, useMemo, useState, type ReactNode } from "react";
import type { Agent } from "../../../desk/src/api/Agent";
import type { InboxResponse } from "../../../desk/src/api/InboxResponse";
import type { ItemPage } from "../../../desk/src/api/ItemPage";
import type { LiveThread } from "../../../desk/src/api/LiveThread";
import type { MissionPage } from "../../../desk/src/api/MissionPage";
import type { RecordName } from "../../../desk/src/api/RecordName";
import type { StatusCount } from "../../../desk/src/api/StatusCount";
import type { ThreadInfo } from "../../../desk/src/api/ThreadInfo";
import type { Tie } from "../../../desk/src/api/Tie";
import type { TiesResponse } from "../../../desk/src/api/TiesResponse";
import type { RequestError } from "../host/api";
import { useDeskReads, type DeskAnswer } from "../react/desk";
import { Badge } from "../t3/ui/badge";
import { Button } from "../t3/ui/button";
import * as m from "./model";
import { dotClass, Heading, MissionGlyph, Note, Row, StatusDot } from "./parts";
import { ProjectGlance } from "./ProjectGlance";

/** What a row or a title of the page opens. The panel frames Desk pages and the layer draws the mission page. */
export type ThreadActions = {
  openItem(repo: string, id: string): void;
  /** The mission page, or the record page framed for a proposed plan. */
  openMission(repo: string, id: string, proposed: boolean): void;
  openInbox(): void;
  openThread(identity: string): void;
  openUrl(url: string): void;
};

export type ThisThreadProps = ThreadActions & {
  /** The Desk, as `desk.urls` names it, and the repository the thread's project maps to there. */
  desk: string;
  repository: string;
  /** T3's thread ID, whole, as records hold it; the threads it started; the IDs its conversation names. */
  thread: string;
  children: string[];
  mentioned: string[];
  /** The clock; a fixture gives a fixed one. */
  now?: () => number;
};

const READ_MS = 30_000;

/** The clock, read again every 30 seconds so ages move between reads. */
function useNow(clock: () => number): number {
  const [now, setNow] = useState(clock);
  useEffect(() => {
    const timer = setInterval(() => setNow(clock()), 30_000);
    return () => clearInterval(timer);
  }, [clock]);
  return now;
}

/** Whether a thread-info read was answered, by T3 Code's thread (a failed read keeps its last), by "no such thread" (404), or failed. */
type ThreadRead = { info: ThreadInfo | null; state: "read" | "pending" | "unknown" | "failed" };
function threadRead(answer: DeskAnswer<ThreadInfo> | undefined): ThreadRead {
  const info = answer?.data ?? null;
  if (!answer) return { info, state: "pending" };
  if (!answer.error) return { info, state: "read" };
  return { info, state: (answer.error as RequestError).status === 404 ? "unknown" : "failed" };
}
/** What the page says of a thread whose own read gave nothing to show. */
const unread = (state: ThreadRead["state"]) => ({ read: "", pending: "Not read yet", unknown: "T3 Code does not know it", failed: "Could not be read" })[state];

/** A read that failed, as the page names it, unless it is a 404 the page reads as "none". */
function failure(what: string, path: string, answer: DeskAnswer<unknown> | undefined, notFoundIsNone = false): string | null {
  const error = answer?.error;
  if (!error || (notFoundIsNone && (error as RequestError).status === 404)) return null;
  return `${what} (GET ${path}): ${error.message}`;
}

export function ThisThread(props: ThisThreadProps) {
  const { desk, repository, thread, children, mentioned, now: clock = Date.now } = props;
  const now = useNow(clock);
  const threadPaths = useMemo(() => [thread, ...children].map(m.paths.thread), [thread, children]);
  const tiePaths = useMemo(() => [thread, ...children].map(m.paths.ties), [thread, children]);
  const infos = useDeskReads<ThreadInfo>(desk, threadPaths, READ_MS);
  const ties = useDeskReads<TiesResponse>(desk, tiePaths, READ_MS);
  const inboxReads = useDeskReads<InboxResponse>(desk, [m.paths.inbox()], READ_MS);
  const named = useDeskReads<RecordName>(desk, mentioned.map(m.paths.record), READ_MS);

  const ownTies = ties.get(m.paths.ties(thread));
  const all = ownTies?.data?.ties ?? [];
  const current = all.filter((t) => t.current);
  const itemTies = m.unique(current.filter((t) => t.kind === "item"));
  const missionTies = m.unique(all.filter((t) => t.kind === "mission" && (t.current || t.role === "planner" || t.role === "lead")));
  const items = useDeskReads<ItemPage>(
    desk,
    itemTies.map((t) => m.paths.item(t.repo, t.id)),
    READ_MS,
  );
  const missionReads = useDeskReads<MissionPage>(
    desk,
    missionTies.map((t) => m.paths.mission(t.repo, t.id)),
    READ_MS,
  );
  const missionPages = missionTies.map((t) => missionReads.get(m.paths.mission(t.repo, t.id))?.data ?? null);
  // The implementer and reviewer threads on the items of the missions it planned or leads, for their chips.
  const agents = [...new Set(missionPages.flatMap((p) => (p?.threads ?? []).filter((a) => a.item !== null && !a.thread).map((a) => a.identity)))];
  const agentInfos = useDeskReads<ThreadInfo>(desk, agents.map(m.paths.thread), READ_MS);

  const inboxAnswer = inboxReads.get(m.paths.inbox());
  const inbox = inboxAnswer?.data;
  const childTies = new Map<string, Tie[]>();
  for (const child of children) {
    const answer = ties.get(m.paths.ties(child));
    if (answer?.data) childTies.set(child, answer.data.ties);
  }
  const namedRecords = mentioned.flatMap((id) => {
    const record = named.get(m.paths.record(id))?.data;
    return record ? [record] : [];
  });
  const arranged = m.arrange({ thread, children, ties: ownTies?.data?.ties ?? null, childTies, missions: missionPages, named: namedRecords, inbox });

  // The reads that say what the thread holds, started and names, and what waits. Until the thread's
  // ties and the inbox have answered the page draws nothing; a child or a record named since is
  // shown once its read answers. Until every one has answered, and if one failed, the page does not
  // say the thread holds nothing; a failed one is named. A record the Desk does not have (404) is
  // one it does not name.
  const essential: [string, string, DeskAnswer<unknown> | undefined, boolean][] = [
    ["This thread's records", m.paths.ties(thread), ownTies, false],
    ["The inbox, for what waits on you", m.paths.inbox(), inboxAnswer, false],
    ...children.map((child): [string, string, DeskAnswer<unknown> | undefined, boolean] => [`The records of ${m.shortName(child)}, a thread it started`, m.paths.ties(child), ties.get(m.paths.ties(child)), false]),
    ...mentioned.map((id): [string, string, DeskAnswer<unknown> | undefined, boolean] => [`${id}, which its conversation names`, m.paths.record(id), named.get(m.paths.record(id)), true]),
  ];
  const loading = !ownTies || !inboxAnswer;
  const pending = essential.some(([, , answer]) => !answer);
  const failed = essential.flatMap(([what, path, answer, notFound]) => failure(what, path, answer, notFound) ?? []);
  // The cards' own reads: a failed one leaves its details out, and is named too.
  const detailFailures = [
    ...itemTies.map((t) => failure(`The page of ${t.id}`, m.paths.item(t.repo, t.id), items.get(m.paths.item(t.repo, t.id)))),
    ...missionTies.map((t) => failure(`The page of ${t.id}`, m.paths.mission(t.repo, t.id), missionReads.get(m.paths.mission(t.repo, t.id)))),
    // T3 Code's word on this thread, the threads it started and those on its missions' items. A
    // thread T3 Code does not know (404) is said to be so on its row, not named here; one whose read
    // failed shows its last answer, if any, and is named.
    failure("This thread's title and status", m.paths.thread(thread), infos.get(m.paths.thread(thread)), true),
    ...children.map((child) => failure(`T3 Code's status of ${m.shortName(child)}, a thread it started`, m.paths.thread(child), infos.get(m.paths.thread(child)), true)),
    ...agents.map((agent) => failure(`T3 Code's status of ${m.shortName(agent)}, a thread on an item of its missions`, m.paths.thread(agent), agentInfos.get(m.paths.thread(agent)), true)),
  ].filter((line): line is string => line !== null);

  const self = infos.get(m.paths.thread(thread))?.data;
  const empty = !pending && m.holdsNothing(arranged, failed);
  const proposed = (repo: string, id: string, status: Tie["status"]) => m.proposed(repo, id, status, missionPages, inbox);
  const waits = m.waits(arranged);
  const holds = arranged.itemTies.length + arranged.missionTies.length;

  return (
    <div className="@container h-full overflow-y-auto bg-background text-foreground" data-this-thread={loading ? "loading" : empty ? "empty" : "holds"}>
      <div className="flex flex-col gap-1 px-4 py-3 text-sm">
        <header className="flex min-w-0 items-center gap-2">
          <h1 className="min-w-0 flex-1 truncate font-semibold text-sm" title={thread} data-thread-title="">
            {self?.title ?? "This thread"}
          </h1>
          <Button size="xs" variant="ghost-muted" onClick={props.openInbox} data-action="inbox" title="Open the inbox">
            <Inbox />
            Inbox
          </Button>
        </header>
        {arranged.earlier ? <p className="text-muted-foreground text-xs">Earlier it {arranged.earlier}.</p> : null}
        {[...failed, ...detailFailures].map((line) => (
          <Note key={line}>{`Could not read ${/^[A-Z][a-z]/.test(line) ? line.charAt(0).toLowerCase() + line.slice(1) : line}`}</Note>
        ))}
        {loading ? (
          <p className="text-muted-foreground text-xs">Reading what this thread holds…</p>
        ) : empty ? (
          <>
            <p className="mt-2 text-muted-foreground text-xs" data-empty="">
              This thread holds nothing, started nothing and names no record. Here is {repository}.
            </p>
            <ProjectGlance desk={desk} repo={repository} now={now} actions={props} />
          </>
        ) : (
          <>
            {waits > 0 ? (
              <section data-section="waiting">
                <Heading warn count={waits}>
                  Waiting on you
                </Heading>
                {arranged.waitingPlans.map(({ repo, mission }) => (
                  <Row key={mission.id} id={mission.id} row={`mission:${mission.id}`} title={mission.title} glyph={<MissionGlyph tone="you" />} note={<Badge variant="warning">Plan proposed</Badge>} onClick={() => props.openMission(repo, mission.id, true)} />
                ))}
                {arranged.waitingThreads.map((t) => (
                  <LiveThreadRow key={t.id} thread={t} openThread={props.openThread} />
                ))}
                {arranged.waitingItems.map(({ repo, row }) => (
                  <Row key={row.id} id={row.id} row={`item:${row.id}`} title={row.title} glyph={<StatusDot status={row.status} />} note={<Badge variant="warning">Needs you</Badge>} onClick={() => props.openItem(repo, row.id)} />
                ))}
              </section>
            ) : null}

            {holds > 0 ? (
              <section data-section="holds">
                <Heading count={holds}>Holds</Heading>
                {arranged.itemTies.map((t) => (
                  <ItemHold key={`${t.repo}/${t.id}`} tie={t} page={items.get(m.paths.item(t.repo, t.id))?.data ?? null} actions={props} />
                ))}
                {arranged.missionTies.map((t) => (
                  <MissionHold key={`${t.repo}/${t.id}`} tie={t} page={missionReads.get(m.paths.mission(t.repo, t.id))?.data ?? null} proposed={proposed(t.repo, t.id, t.status)} actions={props} />
                ))}
              </section>
            ) : null}

            {arranged.workTies.length > 0 ? (
              <section data-section="work">
                <Heading count={arranged.workTies.length}>Work under this thread</Heading>
                {arranged.workTies.map((t) => (
                  <MissionWork
                    key={`${t.repo}/${t.id}`}
                    tie={t}
                    page={missionReads.get(m.paths.mission(t.repo, t.id))?.data ?? null}
                    proposed={proposed(t.repo, t.id, t.status)}
                    infos={agentInfos}
                    actions={props}
                  />
                ))}
              </section>
            ) : null}

            {arranged.others.length > 0 ? (
              <section data-section="started">
                <Heading count={arranged.others.length}>Other threads it started</Heading>
                {arranged.others.map(({ id, ties }) => (
                  <ChildRow key={id} id={id} read={threadRead(infos.get(m.paths.thread(id)))} ties={ties} now={now} openThread={props.openThread} />
                ))}
              </section>
            ) : null}

            {arranged.openMentioned.length + arranged.doneMentioned.length > 0 ? <Mentioned open={arranged.openMentioned} done={arranged.doneMentioned} proposed={proposed} actions={props} /> : null}
          </>
        )}
      </div>
    </div>
  );
}

/** A bar of a mission's items per status. */
function ScopeBar({ counts }: { counts: StatusCount[] }) {
  return (
    <span className="flex h-1.5 w-full overflow-hidden rounded-full bg-secondary" role="img" aria-label={m.countsLine(counts)} title={m.countsLine(counts)}>
      {counts.map((c) => (
        <i key={c.status} className={`h-full ${dotClass[c.status] ?? ""} border-0`} style={{ flex: c.count }} />
      ))}
    </span>
  );
}

function Card({ name, children }: { name: string; children: ReactNode }) {
  return (
    <section className="mb-2 flex flex-col gap-1.5 rounded-lg border border-border bg-card px-3 py-2.5" data-card={name}>
      {children}
    </section>
  );
}

function CardTitle({ id, title, onClick, glyph, aside, action }: { id: string; title: string; onClick(): void; glyph: ReactNode; aside?: ReactNode; action: string }) {
  return (
    <header className="flex min-w-0 items-center gap-2">
      {glyph}
      <button type="button" className="min-w-0 flex-1 cursor-pointer truncate text-left font-medium hover:underline" onClick={onClick} title={`${id}: ${action}`} data-title={id}>
        <span className="font-mono font-normal text-muted-foreground text-xs">{id}</span> {title}
      </button>
      {aside}
    </header>
  );
}

function ItemHold({ tie, page, actions }: { tie: Tie; page: ItemPage | null; actions: ThreadActions }) {
  const row = page?.row;
  const status = row?.status ?? tie.status;
  const ticked = page?.done_when.filter((d) => d.ticked).length ?? 0;
  const reviewer = row?.reviewer ? agentName(page?.threads ?? [], row.reviewer) : null;
  const open = () => actions.openItem(tie.repo, tie.id);
  return (
    <Card name={`item:${tie.id}`}>
      <CardTitle id={tie.id} title={row?.title ?? tie.title} onClick={open} glyph={<StatusDot status={status} />} aside={<Badge variant="outline">{m.statusTitle(status)}</Badge>} action="open its page" />
      <p className="text-muted-foreground text-xs">This thread {m.VERBS[tie.role][0]} this item.</p>
      {page && page.done_when.length > 0 ? (
        <>
          <div className="text-muted-foreground text-xs">
            Done when{" "}
            <span className="tabular-nums">
              {ticked}/{page.done_when.length}
            </span>
          </div>
          <ul className="flex flex-col gap-1 text-xs" data-done-when="">
            {page.done_when.map((d, at) => (
              <li key={at} className={`grid grid-cols-[14px_minmax(0,1fr)] gap-1.5 ${d.ticked ? "text-muted-foreground" : ""}`} data-ticked={d.ticked ? "" : undefined}>
                {d.ticked ? <Check className="mt-px size-3.5 text-success-foreground" aria-label="Ticked" /> : <Square className="mt-px size-3.5 text-muted-foreground" aria-label="Not ticked" />}
                <span className="line-clamp-3">{d.text}</span>
              </li>
            ))}
          </ul>
        </>
      ) : null}
      <div className="flex flex-wrap items-center gap-x-3 gap-y-1 text-muted-foreground text-xs">
        {page?.pull_request ? (
          <button type="button" className="inline-flex cursor-pointer items-center gap-1 hover:text-foreground" onClick={() => actions.openUrl(page.pull_request!.url)} data-pr={page.pull_request.number}>
            <GitPullRequest className="size-3" aria-hidden />#{page.pull_request.number} {page.pull_request.state.toLowerCase()}
          </button>
        ) : row?.branch ? (
          <span>No pull request yet</span>
        ) : null}
        {reviewer ? <span>Reviewed by {reviewer}</span> : row?.status === "in_review" ? <span>No reviewer holds it yet</span> : null}
        <Button size="xs" variant="ghost" className="ml-auto" onClick={open}>
          Open item
          <ChevronRight />
        </Button>
      </div>
    </Card>
  );
}

/** The live thread that is `identity` among a page's agents, as a short name. */
function agentName(agents: Agent[], identity: string): string {
  return agents.find((a) => a.identity === identity)?.thread?.title ?? m.shortName(identity);
}

function MissionHold({ tie, page, proposed, actions }: { tie: Tie; page: MissionPage | null; proposed: boolean; actions: ThreadActions }) {
  const open = page?.items.filter((i) => !m.closed(i.status)) ?? [];
  const counts = page?.mission.counts ?? [];
  const openMission = () => actions.openMission(tie.repo, tie.id, proposed);
  return (
    <Card name={`mission:${tie.id}`}>
      <CardTitle
        id={tie.id}
        title={page?.mission.title ?? tie.title}
        onClick={openMission}
        glyph={<MissionGlyph tone={proposed ? "you" : "run"} />}
        aside={
          <span className="shrink-0 text-muted-foreground text-xs tabular-nums" title={m.countsLine(counts)}>
            {m.finished(counts)}/{m.total(counts)} done
          </span>
        }
        action={proposed ? "a proposed plan: open its record page" : "open the mission page"}
      />
      <p className="text-muted-foreground text-xs">
        This thread {m.VERBS[tie.role][0]} this mission{proposed ? ", a plan still proposed" : ""}.
      </p>
      {page ? <ScopeBar counts={counts} /> : null}
      {open.length > 0 ? (
        <div className="flex flex-col">
          {open.map((i) => (
            <Row
              key={i.id}
              id={i.id}
              row={`item:${i.id}`}
              title={i.title}
              glyph={<StatusDot status={i.status} />}
              note={
                <>
                  <span>{m.statusName(i.status)}</span>
                  {i.holder ? <span className="max-w-32 truncate">{agentName(page?.threads ?? [], i.holder)}</span> : null}
                </>
              }
              onClick={() => actions.openItem(tie.repo, i.id)}
            />
          ))}
        </div>
      ) : null}
    </Card>
  );
}

/** One implementer or reviewer thread on an item, as a chip that opens it in T3 Code. */
function ThreadChip({ agent, read, openThread }: { agent: Agent; read: ThreadRead; openThread(identity: string): void }) {
  const info = read.info;
  const status = agent.thread?.status ?? info?.status ?? (read.state === "failed" ? "not read" : read.state === "pending" ? "…" : "unknown");
  const title = agent.thread?.title ?? info?.title ?? m.shortName(agent.identity);
  const model = agent.thread?.model ?? info?.model ?? null;
  return (
    <button
      type="button"
      className="inline-flex h-5 cursor-pointer items-center gap-1 rounded-sm border border-border px-1.5 text-muted-foreground text-xs hover:bg-accent hover:text-foreground"
      onClick={() => openThread(agent.identity)}
      title={`${title}\n${agent.role}, ${status}${model ? `, ${model}` : ""}`}
      data-chip={agent.identity}
    >
      <span className={`size-1.5 rounded-full ${m.ENDED.has(status) || !(agent.thread ?? info) ? "bg-muted-foreground/50" : status === "waiting" ? "bg-warning" : "bg-success"}`} aria-hidden />
      {agent.role === "implementer" ? "impl" : "review"}
      <span>{status.replaceAll("_", " ")}</span>
      {model ? <span className="font-mono">{model.replace(/^claude-|^gpt-/, "")}</span> : null}
    </button>
  );
}

/** A mission this thread planned or leads, with its items and the threads on each. */
function MissionWork({ tie, page, proposed, infos, actions }: { tie: Tie; page: MissionPage | null; proposed: boolean; infos: Map<string, DeskAnswer<ThreadInfo>>; actions: ThreadActions }) {
  const counts = page?.mission.counts ?? [];
  const done = page?.mission.status === "done";
  const openMission = () => actions.openMission(tie.repo, tie.id, proposed);
  return (
    <Card name={`work:${tie.id}`}>
      <CardTitle
        id={tie.id}
        title={page?.mission.title ?? tie.title}
        onClick={openMission}
        glyph={<MissionGlyph tone={proposed ? "you" : done ? "done" : page?.mission.in_progress ? "run" : "plain"} />}
        aside={
          <span className="shrink-0 text-muted-foreground text-xs tabular-nums" title={m.countsLine(counts)}>
            {m.finished(counts)}/{m.total(counts)} done
          </span>
        }
        action={proposed ? "a proposed plan: open its record page" : "open the mission page"}
      />
      <p className="text-muted-foreground text-xs">
        This thread {m.VERBS[tie.role][tie.current ? 0 : 1]} this mission{done ? ", which is done" : proposed ? ", a plan still proposed" : ""}.
      </p>
      {page ? <ScopeBar counts={counts} /> : null}
      {(page?.items ?? []).map((i) => {
        const agents = (page?.threads ?? []).filter((a) => a.item === i.id && (a.role === "implementer" || a.role === "reviewer"));
        return (
          <div key={i.id} data-work-item={i.id}>
            <Row id={i.id} row={`item:${i.id}`} title={i.title} glyph={<StatusDot status={i.status} />} note={<span>{m.statusName(i.status)}</span>} onClick={() => actions.openItem(tie.repo, i.id)} dim={m.closed(i.status)} />
            {agents.length > 0 ? (
              <div className="flex flex-wrap gap-1 pb-1 pl-7">
                {agents.map((a, at) => (
                  <ThreadChip key={`${a.identity}${at}`} agent={a} read={threadRead(infos.get(m.paths.thread(a.identity)))} openThread={actions.openThread} />
                ))}
              </div>
            ) : null}
          </div>
        );
      })}
    </Card>
  );
}

function ChildRow({ id, read, ties, now, openThread }: { id: string; read: ThreadRead; ties: Tie[]; now: number; openThread(identity: string): void }) {
  const info = read.info;
  const ended = info ? m.ENDED.has(info.status) : false;
  const changed = info?.updated_at ? Date.parse(info.updated_at) : NaN;
  const quiet = !ended && info !== null && !Number.isNaN(changed) && now - changed > m.QUIET_MS;
  const line = m.tieLine(ties);
  const age = m.since(info?.updated_at, now);
  return (
    <button
      type="button"
      data-row={`thread:${id}`}
      onClick={() => openThread(id)}
      title="Open this thread in T3 Code"
      className="grid w-full cursor-pointer grid-cols-[6px_minmax(0,1fr)_auto] items-center gap-2.5 rounded-md px-1.5 py-1 text-left outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset"
    >
      <span className={`size-1.5 rounded-full ${!info || ended ? "bg-muted-foreground/50" : info.status === "waiting" ? "bg-warning" : "bg-success"}`} aria-hidden />
      <span className="flex min-w-0 flex-col">
        <span className="truncate">{info ? info.title : <span className="font-mono">{m.shortName(id)}</span>}</span>
        <span className="flex min-w-0 gap-2 text-muted-foreground text-xs">
          <span className={info?.status === "waiting" ? "text-warning-foreground" : undefined}>{info ? m.threadStatus(info.status) : unread(read.state)}</span>
          {info?.model ? <span className="font-mono">{info.model}</span> : null}
          {line ? <span className="truncate">{line}</span> : null}
        </span>
      </span>
      <span className="text-muted-foreground text-xs tabular-nums">
        {quiet ? (
          <Badge variant="warning" data-quiet="">
            <Clock />
            Quiet {age}
          </Badge>
        ) : (
          (age ?? "")
        )}
      </span>
    </button>
  );
}

function LiveThreadRow({ thread, openThread }: { thread: LiveThread; openThread(identity: string): void }) {
  return (
    <button
      type="button"
      data-row={`thread:${thread.id}`}
      onClick={() => openThread(thread.id)}
      title="Open this thread in T3 Code"
      className="grid w-full cursor-pointer grid-cols-[6px_minmax(0,1fr)_auto] items-center gap-2.5 rounded-md px-1.5 py-1 text-left outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset"
    >
      <span className={`size-1.5 rounded-full ${thread.status === "waiting" ? "bg-warning" : "bg-success"}`} aria-hidden />
      <span className="flex min-w-0 flex-col">
        <span className="truncate">{thread.title}</span>
        <span className="flex min-w-0 gap-2 text-muted-foreground text-xs">
          <span className="font-mono">{thread.model}</span>
          {thread.holds.length ? <span className="truncate">{thread.holds.map((h) => `${h.role} of ${h.id}`).join(", ")}</span> : null}
        </span>
      </span>
      {thread.status === "waiting" ? <Badge variant="warning">Waiting on a person</Badge> : <span className="text-muted-foreground text-xs">{m.threadStatus(thread.status)}</span>}
    </button>
  );
}

function Mentioned({ open, done, proposed, actions }: { open: RecordName[]; done: RecordName[]; proposed(repo: string, id: string, status: RecordName["status"]): boolean; actions: ThreadActions }) {
  const [showDone, setShowDone] = useState(false);
  const row = (r: RecordName) => {
    const isProposed = r.kind === "mission" && proposed(r.repo, r.id, r.status);
    return (
      <Row
        key={r.id}
        id={r.id}
        row={`${r.kind}:${r.id}`}
        title={r.title}
        glyph={r.kind === "mission" ? <MissionGlyph tone={isProposed ? "you" : r.status === "done" ? "done" : "run"} /> : <StatusDot status={r.status} />}
        note={
          <>
            <span>{r.kind}</span>
            <span>{isProposed ? "proposed" : m.statusName(r.status)}</span>
          </>
        }
        onClick={() => (r.kind === "mission" ? actions.openMission(r.repo, r.id, isProposed) : actions.openItem(r.repo, r.id))}
        dim={m.closed(r.status)}
      />
    );
  };
  return (
    <section data-section="mentioned">
      <Heading count={open.length + done.length}>Mentioned</Heading>
      {open.map(row)}
      {done.length > 0 ? (
        <>
          <div className="px-1.5 pt-2 pb-1 text-muted-foreground text-xs">
            Done or dropped: {done.length} of {open.length + done.length}
          </div>
          {(showDone ? done : done.slice(0, 4)).map(row)}
          {done.length > 4 && !showDone ? (
            <Button size="xs" variant="ghost-muted" onClick={() => setShowDone(true)} data-action="more-done">
              Show {done.length - 4} more done
            </Button>
          ) : null}
        </>
      ) : null}
    </section>
  );
}
