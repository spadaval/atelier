// The project home, drawn over T3's main area in place of the Desk's /home/<repo>: the project as a
// roadmap without dates, Now, Next and Later, each mission folding open to its items and who holds
// them, and a strip of the last seven days. Its layout is the Desk's page's, in T3's components and
// colours; its rules are model.ts.
import { ChevronDown, ChevronRight, Circle, CircleCheck, CircleDot, CircleX, Eye, GitMerge, Play, RotateCcw, ShieldCheck, Target, X } from "lucide-react";
import { useEffect, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import type { HistoryResponse } from "../../../desk/src/api/HistoryResponse";
import type { ItemRow } from "../../../desk/src/api/ItemRow";
import type { MissionPage } from "../../../desk/src/api/MissionPage";
import type { MissionRow } from "../../../desk/src/api/MissionRow";
import type { MissionsResponse } from "../../../desk/src/api/MissionsResponse";
import type { RoadmapResponse } from "../../../desk/src/api/RoadmapResponse";
import type { Status } from "../../../desk/src/api/Status";
import type { StatusCount } from "../../../desk/src/api/StatusCount";
import { ago } from "../mission/model";
import { useDeskRead } from "../react/desk";
import { Button } from "../t3/ui/button";
import { Kbd } from "../t3/ui/kbd";
import * as h from "./model";

export type ProjectHomeProps = {
  /** The Desk, as `desk.urls` names it, and the repository's name there. */
  desk: string;
  repo: string;
  onClose(): void;
  /** Opens a mission: the mission page, or for a proposed plan its record page, framed. */
  openMission(id: string, proposed: boolean): void;
  /** Opens an item's Desk page, framed. */
  openItem(id: string): void;
  /** The clock; a fixture gives a fixed one. */
  now?: () => number;
};

export function ProjectHome({ desk, repo, onClose, openMission, openItem, now: clock = Date.now }: ProjectHomeProps) {
  const missions = useDeskRead<MissionsResponse>(desk, h.paths.missions(repo));
  const roadmap = useDeskRead<RoadmapResponse>(desk, h.paths.roadmap(repo));
  const history = useDeskRead<HistoryResponse>(desk, h.paths.week(repo), 30_000);
  const root = useRef<HTMLDivElement>(null);
  useEffect(() => root.current?.focus({ preventScroll: true }), [desk, repo]);
  const onKeyDown = (event: KeyboardEvent) => {
    if (event.defaultPrevented || event.metaKey || event.ctrlKey || event.altKey) return;
    if (event.key === "Escape") onClose();
  };
  const rows = missions.data?.missions ?? [];
  // The items in no mission are left out while their read fails, with a note: the lanes still show the missions.
  const all = h.lanes(rows, roadmap.data?.loose ?? []);
  const proposed = (id: string) => rows.some((m) => m.id === id && m.state === "proposed");
  const now = clock();
  const state = missions.data ? (all.every((lane) => lane.count === 0) ? "empty" : "shown") : missions.error ? "error" : "loading";
  return (
    <div ref={root} tabIndex={-1} onKeyDown={onKeyDown} data-project-home={state} className="@container h-full overflow-y-auto bg-background text-foreground outline-none">
      <div className="mx-auto flex max-w-[1500px] flex-col gap-1 px-6 py-5 text-sm @max-[560px]:px-4">
        <header className="flex min-w-0 items-center gap-2">
          <h1 className="min-w-0 flex-1 truncate font-semibold text-base">{repo}</h1>
          <Button size="xs" variant="ghost-muted" onClick={onClose} data-action="close" aria-label="Close">
            <X />
            <Kbd>Esc</Kbd>
          </Button>
        </header>
        {!missions.data ? (
          <p className={missions.error ? "text-destructive-foreground" : "text-muted-foreground"} data-home-error={missions.error ? "" : undefined}>
            {missions.error ? `Could not read the missions of ${repo} from ${desk}: ${missions.error.message}` : "Reading the project…"}
          </p>
        ) : (
          <>
            {missions.error ? <Note>{`The missions could not be read again from ${desk}, so they are ${asOf(missions.readAt)}: ${missions.error.message}`}</Note> : null}
            {roadmap.error ? <Note>{`The items in no mission could not be read${roadmap.data ? ` again, so they are ${asOf(roadmap.readAt)}` : ", so they are left out"}: ${roadmap.error.message}`}</Note> : null}
            {history.error ? <Note>{`The records' history could not be read${history.data ? ` again, so the last seven days are ${asOf(history.readAt)}` : ", so the last seven days are left out"}: ${history.error.message}`}</Note> : null}
            <div className="flex flex-col gap-5">
              {all.map((lane) => (
                <section key={lane.name} data-lane={lane.name} className="mt-3">
                  <div className="mb-1.5 flex flex-wrap items-baseline gap-x-2 px-0.5">
                    <h2 className="font-semibold text-sm">{lane.title}</h2>
                    <span className="text-muted-foreground text-xs tabular-nums">{lane.count}</span>
                    <span className="text-muted-foreground text-xs">{lane.hint}</span>
                  </div>
                  {lane.count === 0 ? (
                    <p className="px-0.5 text-muted-foreground text-xs">Nothing here.</p>
                  ) : (
                    <div className="flex flex-col">
                      {lane.missions.map((m) => (
                        <MissionLine key={m.id} desk={desk} repo={repo} mission={m} startOpen={h.startsOpen(lane.name, all)} openMission={openMission} openItem={openItem} />
                      ))}
                      <LooseItems rows={lane.loose} limit={h.looseShown(lane.name)} openItem={openItem} />
                    </div>
                  )}
                </section>
              ))}
            </div>
            {history.data ? <Week events={history.data.events} now={now} open={(e) => (e.record === "mission" ? openMission(e.id, proposed(e.id)) : openItem(e.id))} /> : null}
            {missions.data.fetched.at ? <p className="mt-4 text-muted-foreground text-xs">Fetched {ago(missions.data.fetched.at, now)}.</p> : null}
          </>
        )}
      </div>
    </div>
  );
}

/** `as of 14:02:11`: when a read kept beside a failed one last answered. */
const asOf = (at: Date | null) => `as of ${at?.toLocaleTimeString() ?? "the last read"}`;

function Note({ children }: { children: ReactNode }) {
  return (
    <p className="max-w-[75ch] text-destructive-foreground text-xs" data-note="">
      {children}
    </p>
  );
}

/** Where work stands, as one glyph, as the Desk draws it (desk/src/components/ui.tsx, StatusGlyph). */
const GLYPHS: Record<Status, [typeof Circle, string]> = {
  open: [Circle, "text-muted-foreground"],
  building: [CircleDot, "text-info-foreground"],
  in_review: [Eye, "text-violet-500"],
  changes_requested: [RotateCcw, "text-destructive-foreground"],
  approved: [ShieldCheck, "text-success-foreground"],
  done: [CircleCheck, "text-success-foreground"],
  dropped: [CircleX, "text-muted-foreground/60"],
};
function Glyph({ status }: { status: Status }) {
  const [Icon, tone] = GLYPHS[status];
  return <Icon className={`size-3.5 shrink-0 ${tone}`} aria-label={h.statusWord(status)} />;
}

/** A mission's items per status, as one bar, in the Desk's order and status colours. */
const BAR: Record<Status, string> = {
  open: "bg-muted-foreground/40",
  building: "bg-info",
  in_review: "bg-violet-500",
  changes_requested: "bg-destructive",
  approved: "bg-success/60",
  done: "bg-success",
  dropped: "bg-muted-foreground/20",
};
function ScopeBar({ counts }: { counts: StatusCount[] }) {
  const label = counts.length ? counts.map((c) => `${c.count} ${h.statusWord(c.status)}`).join(", ") : "No items";
  return (
    <span className="flex h-1.5 w-full gap-px overflow-hidden rounded-full bg-secondary" role="img" aria-label={label} title={label}>
      {counts.map((c) => (
        <i key={c.status} className={BAR[c.status]} style={{ flex: c.count }} />
      ))}
    </span>
  );
}

const rowClass = "rounded-md outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset";

function MissionLine({
  desk,
  repo,
  mission: m,
  startOpen,
  openMission,
  openItem,
}: {
  desk: string;
  repo: string;
  mission: MissionRow;
  startOpen: boolean;
  openMission(id: string, proposed: boolean): void;
  openItem(id: string): void;
}) {
  const [open, setOpen] = useState(startOpen);
  const total = m.counts.reduce((n, c) => n + c.count, 0);
  const done = m.counts.filter((c) => c.status === "done").reduce((n, c) => n + c.count, 0);
  const proposed = m.state === "proposed";
  return (
    <div data-mission-row={m.id} data-open={open ? "" : undefined}>
      <div className="grid grid-cols-[22px_16px_minmax(0,1fr)_auto_10rem_3rem] items-center gap-2.5 rounded-md py-1 pr-2 pl-1 hover:bg-accent/50 @max-[1000px]:grid-cols-[22px_16px_minmax(0,1fr)_auto_6rem_2.5rem] @max-[560px]:grid-cols-[22px_16px_minmax(0,1fr)_auto_2.5rem]">
        <button
          type="button"
          data-action="fold"
          aria-expanded={open}
          aria-label={open ? "Fold its items" : "Show its items"}
          onClick={() => setOpen(!open)}
          className="grid size-[22px] cursor-pointer place-items-center rounded-md text-muted-foreground outline-none hover:bg-accent hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring"
        >
          {open ? <ChevronDown className="size-3.5" /> : <ChevronRight className="size-3.5" />}
        </button>
        <Target className={`size-3.5 ${m.in_progress ? "text-info-foreground" : proposed ? "text-warning-foreground" : "text-muted-foreground"}`} aria-hidden />
        <button
          type="button"
          data-action="mission"
          onClick={() => openMission(m.id, proposed)}
          title={proposed ? `${m.id}: open its plan` : `${m.id}: open the mission page`}
          className={`flex min-w-0 cursor-pointer items-baseline gap-2 px-1 py-0.5 text-left ${rowClass}`}
        >
          <span className="truncate font-semibold">{m.title}</span>
          <span className="shrink-0 font-mono text-muted-foreground text-xs">{m.id}</span>
        </button>
        <span>{m.waiting.length > 0 ? <span className="whitespace-nowrap text-warning-foreground text-xs">{m.waiting.length} need you</span> : null}</span>
        <span className="@max-[560px]:hidden">
          <ScopeBar counts={m.counts} />
        </span>
        <span className="text-right font-mono text-muted-foreground text-xs tabular-nums">
          {done}/{total}
        </span>
      </div>
      {open ? <MissionItems desk={desk} repo={repo} id={m.id} openItem={openItem} /> : null}
    </div>
  );
}

/** A mission's items, read when it is folded open, each with who holds it. */
function MissionItems({ desk, repo, id, openItem }: { desk: string; repo: string; id: string; openItem(id: string): void }) {
  const page = useDeskRead<MissionPage>(desk, h.paths.mission(repo, id));
  const pad = "pb-2 pl-[2.6rem] @max-[560px]:pl-6";
  if (!page.data) {
    return (
      <p className={`${pad} text-xs ${page.error ? "text-destructive-foreground" : "text-muted-foreground"}`} data-note={page.error ? "" : undefined}>
        {page.error ? `Could not read the items of ${id}: ${page.error.message}` : "Reading its items…"}
      </p>
    );
  }
  const { items, threads } = page.data;
  // A read that fails after one that answered keeps the items it gave, said to be as of then.
  const stale = page.error ? (
    <p className={`${pad} text-destructive-foreground text-xs`} data-note="">
      {`The items of ${id} could not be read again, so they are ${asOf(page.readAt)}: ${page.error.message}`}
    </p>
  ) : null;
  if (items.length === 0)
    return (
      <>
        {stale}
        <p className={`${pad} text-muted-foreground text-xs`}>No items yet.</p>
      </>
    );
  return (
    <div className={`flex flex-col ${pad}`} data-items={id}>
      {stale}
      {h.sortItems(items).map((row) => (
        <ItemLine key={row.id} row={row} holder={h.holderLine(row, threads)} stage openItem={openItem} />
      ))}
    </div>
  );
}

/** An item's priority as three bars, the Desk's (desk/src/components/ui.tsx, Priority): one for low, three for high, three in the warning colour for critical. */
const LEVELS: Record<ItemRow["priority"], number> = { low: 1, medium: 2, high: 3, critical: 3 };
function Priority({ priority }: { priority: ItemRow["priority"] }) {
  const level = LEVELS[priority];
  const on = priority === "critical" ? "bg-warning-foreground" : "bg-foreground/70";
  return (
    <span className="inline-flex h-3 items-end gap-[1.5px]" title={`${priority} priority`} role="img" aria-label={`${priority} priority`} data-priority={priority}>
      {[1, 2, 3].map((n) => (
        <i key={n} className={`w-[3px] rounded-[1px] ${n <= level ? on : "bg-border"}`} style={{ height: 4 + n * 2.6 }} />
      ))}
    </span>
  );
}

function ItemLine({ row, holder, stage, priority, openItem }: { row: ItemRow; holder: string | null; stage: boolean; priority?: boolean; openItem(id: string): void }) {
  const blocked = h.blockedLine(row);
  return (
    <button
      type="button"
      data-item-row={row.id}
      onClick={() => openItem(row.id)}
      title={`${row.id}: open its page`}
      className={`grid min-h-7 w-full cursor-pointer grid-cols-[14px_auto_minmax(0,1fr)_auto] items-center gap-2 px-1.5 py-0.5 text-left ${rowClass}`}
    >
      <Glyph status={row.status} />
      <span className="font-mono text-muted-foreground text-xs">{row.id}</span>
      <span className="truncate">{row.title}</span>
      <span className="flex min-w-0 items-center gap-2.5 whitespace-nowrap text-muted-foreground text-xs">
        {blocked ? <span className="text-destructive-foreground @max-[728px]:hidden">{blocked}</span> : null}
        {holder ? (
          <span className="max-w-56 truncate text-foreground/80 @max-[560px]:hidden" data-holder="">
            {holder}
          </span>
        ) : null}
        {row.needs_human ? <span className="text-warning-foreground">Needs you</span> : null}
        {stage ? <span>{h.statusWord(row.status)}</span> : null}
        {priority ? <Priority priority={row.priority} /> : null}
      </span>
    </button>
  );
}

function LooseItems({ rows, limit, openItem }: { rows: ItemRow[]; limit: number; openItem(id: string): void }) {
  const [all, setAll] = useState(false);
  if (rows.length === 0) return null;
  const shown = all ? rows : rows.slice(0, limit);
  return (
    <div className="flex flex-col pt-1.5 pb-2" data-loose="">
      <div className="px-1.5 py-1 font-semibold text-muted-foreground text-xs">In no mission</div>
      {shown.map((row) => (
        <ItemLine key={row.id} row={row} holder={null} stage={h.started(row)} priority openItem={openItem} />
      ))}
      {rows.length > shown.length ? (
        <button type="button" onClick={() => setAll(true)} className={`cursor-pointer self-start px-1.5 py-0.5 text-muted-foreground text-xs ${rowClass}`}>
          Show {rows.length - shown.length} more
        </button>
      ) : null}
    </div>
  );
}

function Week({ events, now, open }: { events: HistoryResponse["events"]; now: number; open(entry: h.WeekEntry): void }) {
  const days = h.week(events, now);
  const count = days.reduce((n, d) => n + d.entries.length, 0);
  return (
    <section className="mt-7" data-week="">
      <div className="mb-1.5 flex flex-wrap items-baseline gap-x-2 px-0.5">
        <h2 className="font-semibold text-sm">This week</h2>
        <span className="text-muted-foreground text-xs tabular-nums">{count}</span>
        <span className="text-muted-foreground text-xs">items started and merged, plans agreed, from the record history</span>
      </div>
      <div className="grid grid-cols-7 gap-2 @max-[1000px]:grid-cols-4 @max-[560px]:grid-cols-2">
        {days.map((d) => (
          <Day key={d.key} day={d} open={open} />
        ))}
      </div>
    </section>
  );
}

function Day({ day, open }: { day: h.Day; open(entry: h.WeekEntry): void }) {
  const [all, setAll] = useState(false);
  const shown = all ? day.entries : day.entries.slice(0, h.DAY_SHOWN);
  return (
    <div data-day={day.key} className={`min-h-22 min-w-0 rounded-lg border border-border p-2 ${day.entries.length ? "bg-card" : ""}`}>
      <div className="mb-1.5 flex justify-between font-semibold text-muted-foreground text-xs">
        <span>{day.label}</span>
        <span className="tabular-nums">{day.entries.length || ""}</span>
      </div>
      {shown.map((e) => {
        const Icon = e.tone === "ok" ? (e.record === "mission" && e.words.includes("Mission done") ? CircleCheck : GitMerge) : Play;
        return (
          <button
            key={e.id}
            type="button"
            data-entry={e.id}
            onClick={() => open(e)}
            title={`${h.phrase(e.words)} ${e.id}: ${e.title}`}
            className={`grid w-full cursor-pointer grid-cols-[14px_minmax(0,1fr)] items-start gap-1.5 px-0.5 py-1 text-left text-muted-foreground text-xs hover:text-foreground ${rowClass}`}
          >
            <Icon className={`mt-px size-3 ${e.tone === "ok" ? "text-success-foreground" : "text-info-foreground"}`} aria-hidden />
            <span className="line-clamp-2 leading-snug">
              <b className="font-semibold text-foreground">{h.phrase(e.words)}</b> {e.title}
            </span>
          </button>
        );
      })}
      {day.entries.length > shown.length ? (
        <button type="button" onClick={() => setAll(true)} className={`mt-1 cursor-pointer px-0.5 text-muted-foreground text-xs ${rowClass}`}>
          +{day.entries.length - shown.length} more
        </button>
      ) : null}
    </div>
  );
}
