// What the project home shows, worked out from the Desk's reads with no React: which missions and
// loose items go under Now, Next and Later, the order of a mission's items and who holds each, and
// the strip of the last seven days. The rules are the Desk's project home's
// (desk/src/pages/ProjectHome.tsx), restated: the extension may import only types from desk/
// (src/mission/model.ts, span). check/home.tsx draws it from fixtures (test-home.mjs).
import type { Agent } from "../../../desk/src/api/Agent";
import type { HistoryEvent } from "../../../desk/src/api/HistoryEvent";
import type { ItemRow } from "../../../desk/src/api/ItemRow";
import type { MissionRow } from "../../../desk/src/api/MissionRow";
import type { Status } from "../../../desk/src/api/Status";

const seg = encodeURIComponent;
/** The Desk reads the page makes, each one the loader's gate admits (t3-custom.mjs, deskReadUrl). */
export const paths = {
  missions: (repo: string) => `/api/repos/${seg(repo)}/missions`,
  roadmap: (repo: string) => `/api/roadmap?repo=${seg(repo)}`,
  week: (repo: string) => `/api/history?repo=${seg(repo)}&days=7`,
  mission: (repo: string, id: string) => `/api/repos/${seg(repo)}/missions/${seg(id)}`,
};

/** Statuses of an item someone has started and not finished. */
const STARTED = new Set<Status>(["building", "in_review", "changes_requested", "approved"]);

/** Whether an agreed mission has begun: work in progress, or any item past open that was not dropped (desk/src/lib/mission.ts, begun). */
export function begun(m: Pick<MissionRow, "in_progress" | "counts">): boolean {
  return m.in_progress || m.counts.some((c) => c.status !== "open" && c.status !== "dropped" && c.count > 0);
}

export type LaneName = "now" | "next" | "later";
export type Lane = { name: LaneName; title: string; hint: string; missions: MissionRow[]; loose: ItemRow[]; count: number };

/**
 * Now: agreed missions begun, and started items in no mission. Next: agreed missions with nothing
 * started, and open items in no mission. Later: proposed plans. A mission
 * stopping, stopped or done is in none of them.
 */
export function lanes(missions: MissionRow[], loose: ItemRow[]): Lane[] {
  const open = missions.filter((m) => m.state === "open");
  const lane = (name: LaneName, title: string, hint: string, ms: MissionRow[], items: ItemRow[]): Lane => ({ name, title, hint, missions: ms, loose: items, count: ms.length + items.length });
  return [
    lane("now", "Now", "missions in progress, and started work in no mission", open.filter(begun), loose.filter((i) => STARTED.has(i.status))),
    lane("next", "Next", "agreed, nothing started, and open work", open.filter((m) => !begun(m)), loose.filter((i) => i.status === "open")),
    lane("later", "Later", "proposed plans", missions.filter((m) => m.state === "proposed"), []),
  ];
}

/** Whether a mission's row shows its items at first: each under Now, and those under Next when nothing is under way and at most two are next. */
export function startsOpen(lane: LaneName, all: Lane[]): boolean {
  if (lane === "now") return true;
  if (lane === "later") return false;
  const [now, next] = all;
  return now.missions.length === 0 && next.missions.length <= 2;
}

/** How many of a lane's loose items show before "Show N more". */
export const looseShown = (lane: LaneName) => (lane === "later" ? 5 : 8);

/** A mission's items, open work first in the order of the lifecycle, finished ones last. */
const ORDER: Status[] = ["building", "in_review", "changes_requested", "approved", "open", "done", "dropped"];
export const sortItems = (items: ItemRow[]) => [...items].sort((a, b) => ORDER.indexOf(a.status) - ORDER.indexOf(b.status));

/** `in review`: a status as words. */
export const statusWord = (status: Status) => status.replaceAll("_", " ");

/** A thread's name: its title when T3 Code lists it, else the end of its identity (desk/src/components/Glance.tsx, who). */
export function who(agent: Agent | undefined, identity: string): string {
  if (agent?.thread) return agent.thread.title;
  const tail = identity.split(/%3A|:/).at(-1) ?? identity;
  return `thread …${tail.slice(-6)}`;
}

/** Who holds an open item, as its row names it: its reviewer while in review, else its holder; nobody for a finished one. */
export function holderLine(row: ItemRow, agents: Agent[]): string | null {
  if (row.status === "done" || row.status === "dropped") return null;
  const name = (identity: string) => who(agents.find((a) => a.identity === identity && a.thread) ?? agents.find((a) => a.identity === identity), identity);
  if (row.status === "in_review") return row.reviewer ? `review: ${name(row.reviewer)}` : null;
  return row.holder ? name(row.holder) : null;
}

/** `blocked by atelier-x (building)`: the blockers an item still waits on, or null. */
export function blockedLine(row: ItemRow): string | null {
  const open = row.blockers.filter((b) => row.waiting_on.includes(b.id));
  if (open.length === 0) return null;
  return `blocked by ${open.map((b) => `${b.id} (${b.status ? statusWord(b.status) : "not found"})`).join(", ")}`;
}

/** Whether an item has been started, so its loose row names its stage. */
export const started = (row: ItemRow) => STARTED.has(row.status);

export type WeekTone = "run" | "ok";
export type WeekEntry = { id: string; title: string; words: string[]; tone: WeekTone; record: "item" | "mission" };
export type Day = { key: string; label: string; entries: WeekEntry[] };

/** The word for a step the strip shows, by its kind and what record it is of; null for one it leaves out. */
function word(e: HistoryEvent): [string, WeekTone] | null {
  if (e.kind === "started" && e.record === "item") return ["Started", "run"];
  if (e.kind === "merged" && e.record === "mission") return ["Plan agreed", "ok"];
  if (e.kind === "merged" && e.record === "item") return ["Merged", "ok"];
  if (e.kind === "closed" && e.record === "mission") return ["Mission done", "ok"];
  return null;
}

/**
 * The last seven days, oldest first, each with the items started and merged, plans agreed and
 * missions closed that day, in local time. A record with two steps in a day is one entry naming
 * both; finished work comes first in a day. `events` is the history as the Desk gives it, newest first.
 */
export function week(events: HistoryEvent[], now: number): Day[] {
  const days: Day[] = [];
  for (let back = 6; back >= 0; back--) {
    const d = new Date(now);
    d.setHours(0, 0, 0, 0);
    d.setDate(d.getDate() - back);
    days.push({ key: d.toDateString(), label: back === 0 ? "Today" : d.toLocaleDateString(undefined, { weekday: "short", day: "numeric" }), entries: [] });
  }
  for (const e of [...events].reverse()) {
    const found = word(e);
    if (!found || (e.record !== "item" && e.record !== "mission")) continue;
    const [w, tone] = found;
    const day = days.find((d) => d.key === new Date(e.at).toDateString());
    if (!day) continue;
    const same = day.entries.find((x) => x.id === e.id);
    if (same) {
      if (!same.words.includes(w)) same.words.push(w);
      same.tone = tone;
      continue;
    }
    day.entries.push({ id: e.id, title: e.title ?? e.id, words: [w], tone, record: e.record });
  }
  for (const d of days) d.entries.sort((a, b) => Number(b.tone === "ok") - Number(a.tone === "ok"));
  return days;
}

/** `Started and merged`: an entry's words as one phrase. */
export const phrase = (words: string[]) => words.map((w, at) => (at === 0 ? w : w.toLowerCase())).join(" and ");

/** How many of a day's entries show before "+N more". */
export const DAY_SHOWN = 6;
