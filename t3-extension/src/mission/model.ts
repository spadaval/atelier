// What the mission page shows, worked out from the Desk's reads with no React: the running page's
// sentence, what needs the human, the threads at work and the folded lists; the done page's lanes
// and merged items. check/mission.tsx draws it from fixtures (test-mission.mjs).
import type { Agent } from "../../../desk/src/api/Agent";
import type { HistoryEvent } from "../../../desk/src/api/HistoryEvent";
import type { ItemPage } from "../../../desk/src/api/ItemPage";
import type { ItemRow } from "../../../desk/src/api/ItemRow";
import type { MissionPage } from "../../../desk/src/api/MissionPage";
import type { Role } from "../../../desk/src/api/Role";
import type { ThreadInfo } from "../../../desk/src/api/ThreadInfo";

/**
 * `2h 14m`, `36m`, `48s`: a duration, short. The Desk's own `span` (desk/src/lib/mission.ts),
 * restated: the release bundle is built from t3-extension/ alone (scripts/t3-extension-release.sh),
 * so the extension may import only types from desk/, which the build erases.
 */
export function span(ms: number): string {
  const s = Math.max(0, Math.round(ms / 1000));
  if (s < 60) return `${s}s`;
  const m = Math.round(s / 60);
  if (m < 60) return `${m}m`;
  const h = Math.floor(m / 60);
  if (h < 48) return `${h}h ${m % 60}m`;
  return `${Math.round(h / 24)}d`;
}

/** A live thread whose `updated_at` has not changed for this long is quiet: This thread's rule (desk/src/pages/ThisThread.tsx). */
export const QUIET_MS = 30 * 60 * 1000;
/** T3 Code's statuses for a thread whose run is over, as This thread reads them. */
const ENDED = new Set(["completed", "cancelled", "failed", "interrupted", "rolled_back"]);

const seg = encodeURIComponent;
/** The Desk reads the page makes, each one the loader's gate admits (t3-custom.mjs, deskReadUrl). */
export const paths = {
  mission: (repo: string, id: string) => `/api/repos/${seg(repo)}/missions/${seg(id)}`,
  item: (repo: string, id: string) => `/api/repos/${seg(repo)}/items/${seg(id)}`,
  // The gate takes `ids` as record IDs joined by plain commas.
  history: (repo: string, ids: string[]) => `/api/history?repo=${seg(repo)}&ids=${ids.join(",")}`,
  thread: (identity: string) => `/api/thread?id=${seg(identity)}&full=1`,
};

/** Where a mission stands on the page. A proposed plan never reaches it (custom.js opens its record page). */
export type PageState = "running" | "agreed" | "done" | "dropped";
export function pageState(page: MissionPage): PageState {
  if (page.mission.status === "done") return "done";
  if (page.mission.status === "dropped") return "dropped";
  return page.mission.in_progress || page.items.some((i) => i.status !== "draft" && i.status !== "ready" && i.status !== "dropped") ? "running" : "agreed";
}

/** The mission's items that count: every one not dropped. */
export const counted = (page: MissionPage) => page.items.filter((i) => i.status !== "dropped");

/** One thread on the mission: its identity, the role and item it is listed under first, and what T3 Code says of it. */
/**
 * `unknown` is a thread whose full read has no answer yet, or none at all: whether it is quiet
 * cannot be told, so it is neither at work nor said to need nothing. `info` may be the last answer
 * of a read that has since failed.
 */
export type Worker = { identity: string; role: Role; item: ItemRow | null; info: ThreadInfo | null; waiting: boolean; quiet: boolean; unknown: boolean; changedMs: number | null };

/**
 * The identities to read in full: each the Desk names as a thread T3 Code lists (`thread` set),
 * once, so a thread in two roles is one row.
 */
export function listedIdentities(page: MissionPage): string[] {
  return [...new Set(page.threads.filter((a) => a.thread).map((a) => a.identity))];
}

/**
 * The mission's live threads. A thread is live when T3 Code lists it, says it is live and its run is
 * not over. It is waiting when T3 Code says so, and quiet when its `updated_at` is 30 minutes old or
 * more, whatever the stage of its item: a thread still changing is never quiet.
 */
export function liveWorkers(page: MissionPage, infos: Map<string, ThreadInfo>, now: number): Worker[] {
  const seen = new Set<string>();
  const workers: Worker[] = [];
  for (const agent of page.threads) {
    if (!agent.thread || seen.has(agent.identity)) continue;
    seen.add(agent.identity);
    const info = infos.get(agent.identity) ?? null;
    const status = info?.status ?? agent.thread.status;
    if ((info && !info.live) || ENDED.has(status)) continue;
    const changed = info ? Date.parse(info.updated_at) : NaN;
    const changedMs = Number.isNaN(changed) ? null : now - changed;
    workers.push({
      identity: agent.identity,
      role: agent.role,
      item: page.items.find((i) => i.id === agent.item) ?? null,
      info,
      waiting: status === "waiting",
      quiet: changedMs !== null && changedMs >= QUIET_MS,
      unknown: info === null,
      changedMs,
    });
  }
  return workers;
}

/** What a row of *Needs you* opens: an item's Desk page, framed, or a thread. */
export type Target = { kind: "item"; id: string } | { kind: "thread"; identity: string };
export type Ask = { key: string; icon: "user" | "alert"; title: string; detail: string; line: string | null; button: string; target: Target };

const roleName: Record<Role, string> = { lead: "Lead", planner: "Planner", implementer: "Implementer", reviewer: "Reviewer", decider: "Decider" };
export const roleWord = (role: Role) => roleName[role];

/** What a thread is on: its item's title, else the mission's own role. */
export function workerTitle(w: Worker): string {
  return w.item ? w.item.title : w.role === "lead" ? "Leading the mission" : w.role === "planner" ? "Planning the mission" : `${roleWord(w.role)} of the plan`;
}

/**
 * *Needs you*: a row for each open item labelled needs-human, then one for each live thread that is
 * waiting on a person or quiet. A thread both waiting and quiet is one row.
 */
export function asks(page: MissionPage, workers: Worker[]): Ask[] {
  const rows: Ask[] = [];
  for (const item of page.items) {
    if (!item.needs_human || item.status === "done" || item.status === "dropped") continue;
    const behind = page.items.filter((i) => i.waiting_on.includes(item.id)).length;
    rows.push({
      key: `item:${item.id}`,
      icon: "user",
      title: item.title,
      detail: `${item.id} is labelled needs-human.${behind ? ` ${capital(count(behind, "item waits", "items wait"))} behind it.` : ""}`,
      line: null,
      button: "Open item",
      target: { kind: "item", id: item.id },
    });
  }
  for (const w of workers) {
    if (!w.waiting && !w.quiet) continue;
    const role = roleWord(w.role);
    const age = w.changedMs !== null ? span(w.changedMs) : null;
    const detail = w.waiting ? `${role} is waiting on you${w.quiet && age ? `, for ${age}` : ""}.` : `${role} has been quiet for ${age}.`;
    rows.push({
      key: `thread:${w.identity}`,
      icon: w.waiting ? "user" : "alert",
      title: workerTitle(w),
      detail,
      line: w.info?.last_line ?? null,
      button: `Open ${role.toLowerCase()}`,
      target: { kind: "thread", identity: w.identity },
    });
  }
  return rows;
}

const words = ["no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
/** `one item`, `3 items`… in words up to ten. */
export function count(n: number, one: string, many: string): string {
  return `${n <= 10 ? words[n] : n} ${n === 1 ? one : many}`;
}
const capital = (text: string) => text.charAt(0).toUpperCase() + text.slice(1);
function list(parts: string[]): string {
  return parts.length <= 1 ? (parts[0] ?? "") : `${parts.slice(0, -1).join(", ")} and ${parts.at(-1)}`;
}

/**
 * The page's one sentence: how many items are being built, in review and merging; then what waits
 * on the human and how many threads are quiet, or that nothing needs them. `warn` is the second
 * part, drawn in the warning colour when something needs the human.
 */
export function summary(page: MissionPage, workers: Worker[]): { work: string; warn: string; needs: boolean } {
  const items = counted(page);
  const of = (...statuses: string[]) => items.filter((i) => statuses.includes(i.status)).length;
  const building = of("building", "changes_requested");
  const review = of("in_review");
  const merging = of("approved");
  const work: string[] = [];
  // The first part names the items; the rest say only how many: "three items are being built and one is merging".
  const part = (n: number, what: string) => work.push(`${work.length ? count(n, "is", "are") : count(n, "item is", "items are")} ${what}`);
  if (building) part(building, "being built");
  if (review) part(review, "in review");
  if (merging) part(merging, "merging");
  const workText = work.length ? `${capital(list(work))}.` : "No item is being built, reviewed or merged.";
  const waitingItems = items.filter((i) => i.needs_human && i.status !== "done").length;
  const waitingThreads = workers.filter((w) => w.waiting).length;
  const quiet = workers.filter((w) => w.quiet && !w.waiting);
  const warn: string[] = [];
  if (waitingItems || waitingThreads) {
    const who = list([...(waitingItems ? [count(waitingItems, "item", "items")] : []), ...(waitingThreads ? [count(waitingThreads, "thread", "threads")] : [])]);
    warn.push(`${who} ${waitingItems + waitingThreads === 1 ? "waits" : "wait"} on you`);
  }
  if (quiet.length) {
    const longest = Math.max(...quiet.map((w) => w.changedMs ?? 0));
    warn.push(quiet.length === 1 ? `one thread has been quiet for ${span(longest)}` : `${count(quiet.length, "thread", "threads")} have been quiet, the longest for ${span(longest)}`);
  }
  // A thread not read cannot be said to need nothing: the sentence says so instead.
  const unknown = workers.filter((w) => w.unknown && !w.waiting).length;
  if (unknown) warn.push(`whether ${count(unknown, "thread needs", "threads need")} you is not known yet`);
  return warn.length ? { work: workText, warn: `${capital(list(warn))}.`, needs: true } : { work: workText, warn: "Nothing needs you.", needs: false };
}

/** *At work*: each live thread not already under *Needs you*. One not read yet is listed as such, not as working. */
export const atWork = (workers: Worker[]) => workers.filter((w) => !w.waiting && !w.quiet);

/** How long a thread's latest run has gone on. */
export function running(w: Worker, now: number): string | null {
  const started = w.info?.started_at ? Date.parse(w.info.started_at) : NaN;
  return Number.isNaN(started) ? null : span(now - started);
}

/** *Not started*: its items that can start, those ready but waiting on others, and its drafts. */
export function notStarted(page: MissionPage): { rows: ItemRow[]; label: string } {
  const rows = page.items.filter((i) => i.status === "ready" || i.status === "draft");
  const canStart = rows.filter((i) => i.status === "ready" && i.startable).length;
  const blocked = rows.filter((i) => i.status === "ready" && !i.startable).length;
  const drafts = rows.filter((i) => i.status === "draft").length;
  const parts = [...(canStart ? [`${canStart} can start`] : []), ...(blocked ? [`${blocked} wait on others`] : []), ...(drafts ? [`${drafts} draft${drafts === 1 ? "" : "s"}`] : [])];
  return { rows, label: parts.join(", ") || "none" };
}

/** When the records' history says an ID took a step, oldest first. */
const steps = (events: HistoryEvent[], id: string) => events.filter((e) => e.id === id).sort((a, b) => Date.parse(a.at) - Date.parse(b.at));
export const mergedAt = (events: HistoryEvent[], id: string) => steps(events, id).findLast((e) => e.kind === "merged")?.at ?? null;
export const closedAt = (events: HistoryEvent[], id: string) => steps(events, id).findLast((e) => e.kind === "closed")?.at ?? null;
/** The plan's merge: the first `merged` step of the mission's own ID. */
export const planMergedAt = (events: HistoryEvent[], id: string) => steps(events, id).find((e) => e.kind === "merged")?.at ?? null;

/** The done items, newest merge first. */
export function mergedItems(page: MissionPage, events: HistoryEvent[]): ItemRow[] {
  return page.items.filter((i) => i.status === "done").sort((a, b) => (mergedAt(events, b.id) ?? "").localeCompare(mergedAt(events, a.id) ?? ""));
}

export type Stage = "building" | "review" | "merging";
export type Span = { stage: Stage; from: number; to: number };
/** An item's lane: its spans over time, its review rounds, and how long it took from its first step to its merge or its drop. */
export type Lane = { item: ItemRow; spans: Span[]; rounds: number; took: number | null; dropped: boolean };

/**
 * An item's lane from its steps: building from `started` (and after a review asking for changes),
 * in review from `submitted`, merging from an approving review, until `merged`. A lane with no step
 * has no spans; one still open at `end` runs to it.
 */
export function lane(item: ItemRow, events: HistoryEvent[], end: number): Lane {
  const spans: Span[] = [];
  let open: { stage: Stage; from: number } | null = null;
  const close = (at: number) => {
    if (open && at >= open.from) spans.push({ ...open, to: at });
    open = null;
  };
  let rounds = 0;
  let first: number | null = null;
  let ended: number | null = null;
  let dropped = false;
  for (const e of steps(events, item.id)) {
    const at = Date.parse(e.at);
    if (Number.isNaN(at)) continue;
    if (e.kind === "started") {
      close(at);
      open = { stage: "building", from: at };
      first ??= at;
    } else if (e.kind === "submitted") {
      close(at);
      open = { stage: "review", from: at };
      first ??= at;
    } else if (e.kind === "review") {
      rounds = Math.max(rounds + 1, e.round ?? 0);
      close(at);
      open = { stage: e.verdict === "approved" ? "merging" : "building", from: at };
    } else if (e.kind === "merged" || e.kind === "dropped") {
      // A dropped item's work ends when it was dropped, not at the mission's close.
      close(at);
      ended = at;
      dropped = e.kind === "dropped";
    }
  }
  if (ended === null) close(end);
  return { item, spans, rounds, took: first !== null && ended !== null ? ended - first : null, dropped: dropped || item.status === "dropped" };
}

/** The done page's lanes and the span they are drawn over: from the plan's merge to the close. */
export function lanes(page: MissionPage, events: HistoryEvent[], now: number): { lanes: Lane[]; from: number; to: number } {
  const id = page.mission.id;
  const closed = closedAt(events, id);
  const end = closed ? Date.parse(closed) : now;
  // Every item, dropped ones too: the time spent on an item later dropped is part of where it went.
  const all = page.items.map((i) => lane(i, events, end));
  const starts = all.flatMap((l) => l.spans.map((s) => s.from));
  const planned = planMergedAt(events, id);
  const from = planned ? Date.parse(planned) : starts.length ? Math.min(...starts) : end;
  const to = Math.max(end, ...all.flatMap((l) => l.spans.map((s) => s.to)));
  return { lanes: all, from, to };
}

/** A merged item on the done page: its review rounds, how long it took, and its pull request where the Desk gives one. */
export type Merged = { item: ItemRow; rounds: number; took: number | null; pr: ItemPage["pull_request"] };
/** The merged items, most review rounds first, then the longest. */
export function retroItems(page: MissionPage, events: HistoryEvent[], pages: Map<string, ItemPage>, now: number): Merged[] {
  return mergedItems(page, events)
    .map((item) => {
      const l = lane(item, events, now);
      return { item, rounds: l.rounds, took: l.took, pr: pages.get(item.id)?.pull_request ?? null };
    })
    .sort((a, b) => b.rounds - a.rounds || (b.took ?? 0) - (a.took ?? 0));
}

/** The thread `L` opens: the lead's, else the planner's. */
export function leadThread(page: MissionPage): Agent | null {
  return page.threads.find((a) => a.role === "lead") ?? page.threads.find((a) => a.role === "planner") ?? null;
}

/** A section of the mission's record by its heading. */
export const section = (page: MissionPage, heading: string) => page.sections.find((s) => s.heading.toLowerCase() === heading.toLowerCase())?.markdown.trim() || null;

/** `3h ago`, `just now`. */
export function ago(at: string | number, now: number): string {
  const ms = now - (typeof at === "number" ? at : Date.parse(at));
  return ms < 60_000 ? "just now" : `${span(ms)} ago`;
}
