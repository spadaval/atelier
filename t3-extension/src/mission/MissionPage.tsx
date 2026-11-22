// The mission page, drawn over T3's main area in place of the Desk's mission control. Running, it
// says in one sentence how the mission stands, lists what needs the human with the button that
// deals with each, shows the threads at work with their latest line, and folds the rest. Done, it
// shows the Result, a lane per item over time and the merged items, most reviewed first. Its
// design is concept A of the mockups (t3-extension/spike/mockups/mission-concepts.html on
// spike/extension-react), with concept B's lanes in the retro. Its model is model.ts.
import { AlertTriangle, Check, GitPullRequest, MessageSquare, User, X } from "lucide-react";
import { useEffect, useMemo, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import type { HistoryEvent } from "../../../desk/src/api/HistoryEvent";
import type { HistoryResponse } from "../../../desk/src/api/HistoryResponse";
import type { ItemPage } from "../../../desk/src/api/ItemPage";
import type { MissionPage as MissionRead } from "../../../desk/src/api/MissionPage";
import type { ThreadInfo } from "../../../desk/src/api/ThreadInfo";
import { useDeskRead, useDeskReads, type DeskAnswer } from "../react/desk";
import { Badge } from "../t3/ui/badge";
import { Button } from "../t3/ui/button";
import { CollapsibleSectionHeader } from "../t3/ui/collapsible-section-header";
import { Kbd } from "../t3/ui/kbd";
import { Markdown } from "./markdown";
import * as m from "./model";

export type MissionPageProps = {
  /** The Desk, as `desk.urls` names it, and the mission's repository and ID there. */
  desk: string;
  repo: string;
  id: string;
  onClose(): void;
  openThread(identity: string): void;
  /** Opens an item's Desk page, framed. */
  openItem(id: string): void;
  /** Opens a pull request on the forge. */
  openUrl(url: string): void;
  /** The clock; a fixture gives a fixed one. */
  now?: () => number;
};

/** The clock, read again every 30 seconds so ages move between reads. */
function useNow(clock: () => number): number {
  const [now, setNow] = useState(clock);
  useEffect(() => {
    const timer = setInterval(() => setNow(clock()), 30_000);
    return () => clearInterval(timer);
  }, [clock]);
  return now;
}


export function MissionPage({ desk, repo, id, onClose, openThread, openItem, openUrl, now: clock = Date.now }: MissionPageProps) {
  const now = useNow(clock);
  const read = useDeskRead<MissionRead>(desk, m.paths.mission(repo, id));
  const mission = read.data;
  const ids = useMemo(() => (mission ? [id, ...mission.items.map((i) => i.id)] : []), [id, mission]);
  const history = useDeskReads<HistoryResponse>(desk, ids.length ? [m.paths.history(repo, ids)] : [], 30_000);
  const historyAnswer = [...history.values()][0];
  const events = useMemo(() => historyAnswer?.data?.events ?? [], [historyAnswer]);
  const historyError = historyAnswer?.error ?? null;
  const state = mission ? m.pageState(mission) : null;
  const finished = state === "done" || state === "dropped";
  // A finished mission's threads are about something else now; only its merged items' pages are read, for their pull requests.
  const threadIds = mission && !finished ? m.listedIdentities(mission) : [];
  const threadReads = useDeskReads<ThreadInfo>(desk, threadIds.map(m.paths.thread));
  const itemIds = mission && finished ? mission.items.filter((i) => i.status === "done").map((i) => i.id) : [];
  const itemReads = useDeskReads<ItemPage>(
    desk,
    itemIds.map((item) => m.paths.item(repo, item)),
    60_000,
  );

  const root = useRef<HTMLDivElement>(null);
  useEffect(() => root.current?.focus({ preventScroll: true }), [desk, repo, id]);
  const lead = mission ? m.leadThread(mission) : null;
  const onKeyDown = (event: KeyboardEvent) => {
    const target = event.target as HTMLElement;
    if (event.defaultPrevented || event.metaKey || event.ctrlKey || event.altKey || target.closest("input, textarea, [contenteditable]")) return;
    if (event.key === "Escape") onClose();
    else if ((event.key === "l" || event.key === "L") && lead) {
      event.preventDefault();
      openThread(lead.identity);
    }
  };

  const title = mission?.mission.title ?? id;
  return (
    <div ref={root} tabIndex={-1} onKeyDown={onKeyDown} data-mission-page={state ?? "loading"} className="@container h-full overflow-y-auto bg-background text-foreground outline-none">
      <div className="flex flex-col gap-1 px-6 py-5 text-sm @max-[560px]:px-4">
        <header className="flex min-w-0 items-center gap-2">
          <h1 className="min-w-0 flex-1 truncate font-semibold text-base" title={`${id}: ${title}`}>
            {title}
          </h1>
          {lead ? (
            <Button size="xs" variant="outline" onClick={() => openThread(lead.identity)} data-action="lead" title={`Open the ${lead.role}'s thread`}>
              <MessageSquare />
              {lead.role === "lead" ? "Lead" : "Planner"}
              <Kbd>L</Kbd>
            </Button>
          ) : null}
          <Button size="xs" variant="ghost-muted" onClick={onClose} data-action="close" aria-label="Close">
            <X />
            <Kbd>Esc</Kbd>
          </Button>
        </header>
        {!mission ? (
          <p className={read.error ? "text-destructive-foreground" : "text-muted-foreground"} data-mission-error={read.error ? "" : undefined}>
            {read.error ? `Could not read the mission from ${desk}: ${read.error.message}` : "Reading the mission…"}
          </p>
        ) : (
          <>
            {read.error ? <Note>{`The mission could not be read again from ${desk}, so this is as of ${read.readAt?.toLocaleTimeString() ?? "the last read"}: ${read.error.message}`}</Note> : null}
            {historyError ? <Note>{`The records' history could not be read, so merge times and lanes are missing: ${historyError.message}`}</Note> : null}
            {finished ? (
              <Retro mission={mission} events={events} pages={itemReads} repo={repo} now={now} openItem={openItem} openUrl={openUrl} />
            ) : (
              <Running mission={mission} events={events} threads={threadReads} now={now} openItem={openItem} openThread={openThread} />
            )}
          </>
        )}
      </div>
    </div>
  );
}

function Note({ children }: { children: ReactNode }) {
  return (
    <p className="max-w-[75ch] text-destructive-foreground text-xs" data-note="">
      {children}
    </p>
  );
}

function Progress({ done, total }: { done: number; total: number }) {
  return (
    <span className="h-1 w-full max-w-44 overflow-hidden rounded-full bg-secondary" role="img" aria-label={`${done} of ${total} merged`}>
      <span className="block h-full rounded-full bg-success" style={{ width: total ? `${(done / total) * 100}%` : 0 }} />
    </span>
  );
}

/** A section heading that does not fold: a label, a rule and a count, as T3's section headers. */
function Heading({ children, count, warn }: { children: ReactNode; count?: ReactNode; warn?: boolean }) {
  return (
    <h2 className={`mt-4 flex h-8 items-center gap-2 px-2 font-medium text-xs ${warn ? "text-warning-foreground" : "text-muted-foreground"}`}>
      <span className="shrink-0">{children}</span>
      <span aria-hidden className="h-px min-w-2 flex-1 bg-border" />
      {count !== undefined ? <span className="shrink-0 font-normal tabular-nums">{count}</span> : null}
    </h2>
  );
}

/** A section folded to one line that opens in place. */
function Fold({ name, label, children }: { name: string; label: ReactNode; children: ReactNode }) {
  const [open, setOpen] = useState(false);
  return (
    <section data-fold={name}>
      <CollapsibleSectionHeader expanded={open} onClick={() => setOpen(!open)} accessory={<span className="shrink-0 font-normal tabular-nums">{label}</span>}>
        {name}
      </CollapsibleSectionHeader>
      {open ? <div className="px-2 pt-1 pb-3">{children}</div> : null}
    </section>
  );
}

function Running({
  mission,
  events,
  threads,
  now,
  openItem,
  openThread,
}: {
  mission: MissionRead;
  events: HistoryEvent[];
  threads: Map<string, DeskAnswer<ThreadInfo>>;
  now: number;
  openItem(id: string): void;
  openThread(identity: string): void;
}) {
  // Each listed thread's last answer, kept through a failed read, by the identity it was asked for.
  const infos = new Map<string, ThreadInfo>();
  const failed: string[] = [];
  for (const identity of m.listedIdentities(mission)) {
    const answer = threads.get(m.paths.thread(identity));
    if (answer?.data) infos.set(identity, answer.data);
    if (answer?.error) failed.push(`${m.paths.thread(identity)}: ${answer.error.message}`);
  }
  // T3 Code's threads are the Desk's to read; a mission whose threads could not be read is never said to need nothing.
  const t3Error = mission.t3.reachable ? null : (mission.t3.error ?? "the Desk has not read T3 Code's threads");
  const workers = m.liveWorkers(mission, infos, now);
  const asks = m.asks(mission, workers);
  const work = m.atWork(workers);
  const sentence = m.summary(mission, workers);
  const items = m.counted(mission);
  const merged = m.mergedItems(mission, events);
  const notStarted = m.notStarted(mission);
  const outcome = m.section(mission, "Outcome");
  const criteria = m.section(mission, "Success criteria");
  const state = m.pageState(mission);
  const act = (target: m.Target) => (target.kind === "item" ? openItem(target.id) : openThread(target.identity));
  return (
    <>
      <div className="flex min-w-0 items-center gap-2.5 text-muted-foreground text-xs">
        {state === "running" ? (
          <Badge variant="success" data-state="running">
            <span className="size-1.5 rounded-full bg-success" />
            Running
          </Badge>
        ) : (
          <Badge variant="info" data-state="agreed">
            Agreed
          </Badge>
        )}
        <Progress done={merged.length} total={items.length} />
        <span className="shrink-0 tabular-nums">
          {merged.length} of {items.length} merged
        </span>
      </div>
      <p className="mt-3 max-w-[75ch] text-pretty text-base leading-normal" data-summary="">
        {sentence.work} {t3Error ? null : <span className={sentence.needs ? "text-warning-foreground" : undefined}>{sentence.warn}</span>}
      </p>
      {t3Error ? <Note>{`T3 Code's threads could not be read, so threads waiting on you or gone quiet are not shown: ${t3Error}`}</Note> : null}
      {failed.length ? <Note>{`${failed.length === 1 ? "One thread" : `${failed.length} threads`} could not be read again; what is shown of ${failed.length === 1 ? "it" : "them"} is from the last read that answered: ${failed[0]}`}</Note> : null}
      <div className="grid gap-x-8 @min-[1100px]:grid-cols-[minmax(0,75ch)_minmax(0,75ch)]">
        {asks.length ? (
          <section data-section="needs-you">
            <Heading warn count={asks.length}>
              Needs you
            </Heading>
            {asks.map((ask) => (
              <div
                key={ask.key}
                data-row={ask.key}
                className="grid grid-cols-[14px_minmax(0,1fr)_auto] items-center gap-x-2.5 gap-y-1 border-border px-2 py-2 [&+&]:border-t @max-[560px]:grid-cols-[14px_minmax(0,1fr)]"
              >
                {ask.icon === "user" ? <User className="size-3.5 text-warning-foreground" aria-hidden /> : <AlertTriangle className="size-3.5 text-warning-foreground" aria-hidden />}
                <div className="min-w-0">
                  <div className="max-w-[75ch] truncate font-medium" title={ask.title}>
                    {ask.title}
                  </div>
                  <div className="max-w-[75ch] truncate text-muted-foreground text-xs" title={ask.line ?? ask.detail}>
                    {ask.detail}
                    {ask.line ? (
                      <>
                        {" "}
                        Last: <span className="font-mono">{ask.line}</span>
                      </>
                    ) : null}
                  </div>
                </div>
                <Button size="xs" variant="outline" onClick={() => act(ask.target)} className="@max-[560px]:col-start-2 @max-[560px]:justify-self-start">
                  {ask.button}
                </Button>
              </div>
            ))}
          </section>
        ) : null}
        <section data-section="at-work">
          <Heading count={`${work.length} ${work.length === 1 ? "thread" : "threads"}`}>At work</Heading>
          {work.length === 0 ? (
            <p className="px-2 text-muted-foreground text-xs">No thread is at work on this mission{asks.some((a) => a.target.kind === "thread") ? " but those above" : ""}.</p>
          ) : null}
          {work.map((w) => (
            <button
              key={w.identity}
              type="button"
              data-row={`thread:${w.identity}`}
              onClick={() => openThread(w.identity)}
              title={`${m.roleWord(w.role)}: ${w.info?.title ?? w.identity}. Open the thread`}
              className="grid h-7 w-full cursor-pointer grid-cols-[6px_minmax(0,14rem)_minmax(0,1fr)_auto] items-center gap-2.5 rounded-md px-2 text-left outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset @max-[560px]:grid-cols-[6px_minmax(0,1fr)_auto]"
            >
              <span className={`size-1.5 rounded-full ${w.unknown ? "bg-muted-foreground/50" : "bg-success"}`} aria-hidden data-unknown={w.unknown ? "" : undefined} />
              <span className="truncate">{m.workerTitle(w)}</span>
              {w.unknown ? (
                <span className="truncate text-muted-foreground text-xs @max-[560px]:hidden">Not read yet, so whether it is quiet is not known</span>
              ) : (
                <span className="truncate font-mono text-muted-foreground text-xs @max-[560px]:hidden">{w.info?.last_line ?? w.info?.title ?? ""}</span>
              )}
              <span className="text-muted-foreground text-xs tabular-nums">{m.running(w, now) ?? ""}</span>
            </button>
          ))}
        </section>
      </div>
      <div className="mt-4">
        <Fold name="Not started" label={notStarted.label}>
          <ItemList
            rows={notStarted.rows.map((row) => ({ row, note: m.startNote(row) }))}
            openItem={openItem}
          />
        </Fold>
        <Fold name="Merged" label={merged.length}>
          <ItemList
            rows={merged.map((row) => {
              const at = m.mergedAt(events, row.id);
              return { row, note: at ? `merged ${m.ago(at, now)}` : "merged" };
            })}
            openItem={openItem}
          />
          {merged.length ? <p className="mt-1 text-muted-foreground text-xs">Merge times are read from commit subjects, so they are approximate.</p> : null}
        </Fold>
        <Fold name="Outcome and success criteria" label="">
          <div className="flex flex-col gap-3">
            {outcome ? <Markdown text={outcome} /> : <p className="text-muted-foreground text-xs">The mission's record has no Outcome.</p>}
            {criteria ? (
              <>
                <h3 className="font-medium text-muted-foreground text-xs">Success criteria</h3>
                <Markdown text={criteria} />
              </>
            ) : null}
          </div>
        </Fold>
      </div>
    </>
  );
}

function ItemList({ rows, openItem }: { rows: { row: { id: string; title: string }; note: string }[]; openItem(id: string): void }) {
  if (!rows.length) return <p className="text-muted-foreground text-xs">None.</p>;
  return (
    <div className="flex max-w-[75ch] flex-col">
      {rows.map(({ row, note }) => (
        <button
          key={row.id}
          type="button"
          data-row={`item:${row.id}`}
          onClick={() => openItem(row.id)}
          title={`${row.id}: open its page`}
          className="grid h-7 cursor-pointer grid-cols-[minmax(0,1fr)_fit-content(60%)] items-center gap-2.5 rounded-md px-2 text-left outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset"
        >
          <span className="truncate">{row.title}</span>
          <span className="truncate text-muted-foreground text-xs" title={note}>
            {note}
          </span>
        </button>
      ))}
    </div>
  );
}

const stageClass: Record<m.Stage, string> = { building: "bg-info", review: "bg-violet-500", merging: "bg-success" };
const stageName: Record<m.Stage, string> = { building: "Building", review: "In review", merging: "Approved, merging" };
const rounds = (n: number) => `${n} ${n === 1 ? "round" : "rounds"}`;

function Retro({
  mission,
  events,
  pages,
  repo,
  now,
  openItem,
  openUrl,
}: {
  mission: MissionRead;
  events: HistoryEvent[];
  pages: Map<string, DeskAnswer<ItemPage>>;
  repo: string;
  now: number;
  openItem(id: string): void;
  openUrl(url: string): void;
}) {
  const read = new Map<string, ItemPage>();
  for (const item of mission.items) {
    const answer = pages.get(m.paths.item(repo, item.id));
    if (answer?.data) read.set(item.id, answer.data);
  }
  const items = m.counted(mission);
  const merged = m.retroItems(mission, events, read, now);
  const closed = m.closedAt(events, mission.mission.id);
  const planned = m.planMergedAt(events, mission.mission.id);
  const drawn = m.lanes(mission, events, now);
  const result = m.section(mission, "Result");
  const dropped = mission.mission.status === "dropped";
  const when = closed ? ` ${dropped ? "Dropped" : "Closed"} ${m.ago(closed, now)}${planned ? `, ${m.span(Date.parse(closed) - Date.parse(planned))} after the plan merged` : ""}.` : "";
  return (
    <>
      <div className="flex min-w-0 items-center gap-2.5 text-muted-foreground text-xs">
        <Badge variant="secondary" data-state={dropped ? "dropped" : "done"}>
          {dropped ? <X /> : <Check />}
          {dropped ? "Dropped" : "Done"}
        </Badge>
        <span className="min-w-0 truncate" data-closed="">
          {merged.length} of {items.length} merged.{when}
        </span>
      </div>
      <Heading>Result</Heading>
      <div className="px-2" data-section="result">
        {result ? <Markdown text={result} /> : <p className="text-muted-foreground text-xs">The mission's record has no Result.</p>}
      </div>
      <Heading>Where the time went</Heading>
      <section className="px-2" data-section="lanes">
        {drawn.lanes.length === 0 ? <p className="text-muted-foreground text-xs">It had no items.</p> : null}
        {drawn.lanes.length ? (
          <>
            <div className="mb-2 flex flex-wrap gap-x-3.5 gap-y-1 text-muted-foreground text-xs">
              {(Object.keys(stageName) as m.Stage[]).map((stage) => (
                <span key={stage} className="inline-flex items-center gap-1.5" data-legend={stage}>
                  <i className={`size-2.5 rounded-sm ${stageClass[stage]}`} />
                  {stageName[stage]}
                </span>
              ))}
            </div>
            {drawn.lanes.map((lane) => (
              <Lane key={lane.item.id} lane={lane} from={drawn.from} to={drawn.to} />
            ))}
            <div className="mt-1 grid grid-cols-[minmax(0,12rem)_minmax(0,1fr)_7rem] gap-2.5 text-muted-foreground text-xs @max-[560px]:grid-cols-[6rem_minmax(0,1fr)]">
              <span />
              <div className="flex justify-between border-border border-t pt-0.5">
                <span>{planned ? "plan merged" : "first step"}</span>
                <span>{m.span(drawn.to - drawn.from)}</span>
                <span>{closed ? (dropped ? "dropped" : "closed") : "now"}</span>
              </div>
            </div>
            <p className="mt-1 max-w-[75ch] text-muted-foreground text-xs">The lanes are read from commit subjects, so their times are approximate.</p>
          </>
        ) : null}
      </section>
      <Heading count={merged.length > 1 ? `${merged.length}, most review rounds first` : merged.length}>Merged</Heading>
      <section className="flex max-w-[75ch] flex-col" data-section="merged">
        {merged.map(({ item, rounds: n, took, pr }) => (
          <button
            key={item.id}
            type="button"
            data-row={`item:${item.id}`}
            onClick={() => (pr ? openUrl(pr.url) : openItem(item.id))}
            title={pr ? `${item.id}: open pull request #${pr.number} on the forge` : `${item.id}: its pull request is not known; open its page`}
            className="grid h-7 cursor-pointer grid-cols-[5.5rem_minmax(0,1fr)_auto] items-center gap-2.5 rounded-md px-2 text-left outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset"
          >
            <span className="inline-flex items-center gap-1 text-muted-foreground text-xs" data-pr={pr ? pr.number : "unknown"}>
              <GitPullRequest className="size-3" aria-hidden />
              {pr ? `#${pr.number}` : "not known"}
            </span>
            <span className="truncate">{item.title}</span>
            <span className="text-muted-foreground text-xs tabular-nums">
              {rounds(n)}
              {took !== null ? ` · ${m.span(took)}` : ""}
            </span>
          </button>
        ))}
        {merged.length === 0 ? <p className="px-2 text-muted-foreground text-xs">No item of it merged.</p> : null}
      </section>
    </>
  );
}

function Lane({ lane, from, to }: { lane: m.Lane; from: number; to: number }) {
  const width = Math.max(1, to - from);
  const pct = (at: number) => `${(Math.max(0, Math.min(width, at - from)) / width) * 100}%`;
  return (
    <div data-lane={lane.item.id} className="grid h-6 grid-cols-[minmax(0,12rem)_minmax(0,1fr)_7rem] items-center gap-2.5 @max-[560px]:grid-cols-[6rem_minmax(0,1fr)]">
      <span className="truncate text-xs" title={`${lane.item.id}: ${lane.item.title}`}>
        {lane.item.title}
      </span>
      <div className="relative h-2.5">
        {lane.spans.map((s, at) => (
          <i
            key={at}
            data-span={s.stage}
            title={`${stageName[s.stage]} ${m.span(s.to - s.from)}`}
            className={`absolute top-0 h-full min-w-[3px] rounded-sm ${stageClass[s.stage]}`}
            style={{ left: pct(s.from), width: `calc(${pct(s.to)} - ${pct(s.from)})` }}
          />
        ))}
      </div>
      <span className="truncate text-right text-muted-foreground text-xs tabular-nums @max-[560px]:hidden">
        {lane.spans.length ? `${lane.dropped ? "dropped · " : ""}${rounds(lane.rounds)}${lane.took !== null ? ` · ${m.span(lane.took)}` : ""}` : lane.dropped ? "dropped" : "no step recorded"}
      </span>
    </div>
  );
}
