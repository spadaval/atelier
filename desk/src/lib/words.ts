import type { FetchMark } from "../api/FetchMark";
import type { MissionRow } from "../api/MissionRow";
import type { MissionState } from "../api/MissionState";
import type { MissionStatus } from "../api/MissionStatus";
import type { Role } from "../api/Role";
import type { T3State } from "../api/T3State";
import type { Status } from "../api/Status";
import type { StatusCount } from "../api/StatusCount";

/** Every status, in the order of the lifecycle, as `atelier show` groups items. */
export const LIFECYCLE: Status[] = ["draft", "ready", "building", "in_review", "changes_requested", "approved", "done", "dropped"];

/** A status as the CLI names it: `in review`, not `in_review`. */
export function statusName(status: Status | MissionStatus): string {
  return status.replaceAll("_", " ");
}

/** The same, starting with a capital, for a heading or a pill. */
export function statusTitle(status: Status | MissionStatus): string {
  const name = statusName(status);
  return name[0].toUpperCase() + name.slice(1);
}

/** `5 draft, 2 ready`: a mission's items per status, as one line. */
export function countsLine(counts: StatusCount[]): string {
  if (counts.length === 0) return "No items";
  return counts.map((c) => `${c.count} ${statusName(c.status)}`).join(", ");
}

export function total(counts: StatusCount[]): number {
  return counts.reduce((sum, c) => sum + c.count, 0);
}

export function finished(counts: StatusCount[]): number {
  return counts.filter((c) => c.status === "done" || c.status === "dropped").reduce((sum, c) => sum + c.count, 0);
}

/** Where a mission stands, for its glyph: work under way shows as running. */
export function missionState(mission: Pick<MissionRow, "state" | "in_progress">): string {
  if (mission.in_progress) return "building";
  return { proposed: "draft", open: "ready", stop_pending: "changes_requested", stopped: "dropped", done: "done" }[mission.state];
}

/** A mission's state in words, for its pill. */
export const stateWords: Record<MissionState, string> = {
  proposed: "Proposed",
  open: "Agreed",
  stop_pending: "Stop not published yet",
  stopped: "Stopped",
  done: "Done",
};

/**
 * When a repository was last fetched, as a page says it: `Fetched from
 * origin 3 minutes ago`, with the Desk's period and its last failure.
 */
export function fetchedLine(fetched: FetchMark, now = Date.now()): string {
  if (fetched.remote === null) return "This repository has no remote, so nothing is fetched; branches are read as this clone has them.";
  const at = fetched.at ? Date.parse(fetched.at) : NaN;
  const when = Number.isNaN(at) ? `never fetched from ${fetched.remote}` : `fetched from ${fetched.remote} ${ago(Math.max(0, (now - at) / 1000))}`;
  const every = fetched.every_seconds > 0 ? ` The Desk fetches every ${plural(Math.round(fetched.every_seconds / 60), "minute")}.` : " The Desk does not fetch it.";
  const failed = fetched.error ? ` Its last fetch failed: ${fetched.error}` : "";
  return `${when[0].toUpperCase()}${when.slice(1)}.${every}${failed}`;
}

export function plural(n: number, one: string, many = `${one}s`): string {
  return `${n} ${n === 1 ? one : many}`;
}

/** `Nov 11`, or `Nov 11, 2025` outside this year. */
export function shortDate(iso: string): string {
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return iso;
  const thisYear = d.getFullYear() === new Date().getFullYear();
  return d.toLocaleDateString(undefined, { month: "short", day: "numeric", ...(thisYear ? {} : { year: "numeric" }) });
}

/** `Nov 11, 14:02`: a time to the minute, for a review or a go. */
export function dateTime(iso: string): string {
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return iso;
  return `${shortDate(iso)}, ${d.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" })}`;
}

export function shortCommit(commit: string | null | undefined): string {
  return commit ? commit.slice(0, 8) : "";
}

/** What a thread is to the records, as the agents list names it. */
export const roleWords: Record<Role, string> = {
  planner: "Planner",
  implementer: "Implementer",
  reviewer: "Reviewer",
  lead: "Lead",
  decider: "Answered a question",
};

/** T3 Code's live statuses in words; `waiting` is a thread waiting on a person. */
const threadWords: Record<string, string> = {
  preparing: "Preparing",
  queued: "Queued",
  starting: "Starting",
  running: "Running",
  waiting: "Waiting on a person",
};

export function threadStatus(status: string): string {
  return threadWords[status] ?? statusTitle(status as never);
}

/**
 * What a page says about T3 Code's threads when they are not fresh, or
 * `null` when they are. A list read before a failed poll is still served,
 * with its age; with none read, the page knows no thread at all.
 */
export function t3Note(t3: T3State): { known: boolean; text: string; error: string | null } | null {
  if (t3.reachable) return null;
  if (t3.age_seconds === null) return { known: false, text: "Threads are unavailable: T3 Code cannot be reached.", error: t3.error };
  return { known: true, text: `T3 Code cannot be reached just now, so threads are shown as they were ${ago(t3.age_seconds)}.`, error: t3.error };
}

/** `40 seconds ago`, `3 minutes ago`. */
export function ago(seconds: number): string {
  if (seconds < 60) return `${plural(Math.round(seconds), "second")} ago`;
  if (seconds < 3600) return `${plural(Math.round(seconds / 60), "minute")} ago`;
  if (seconds < 172_800) return `${plural(Math.round(seconds / 3600), "hour")} ago`;
  return `${plural(Math.round(seconds / 86_400), "day")} ago`;
}

export const recommendationWords: Record<string, string> = {
  fix_now: "Fix now",
  defer: "Defer",
  no_action: "No action",
};
