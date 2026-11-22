// What This thread shows, worked out from the Desk's reads with no React: the records the page is
// about, what of them waits on the human, the threads it started that no record shows, and whether
// it may say the thread holds nothing. The Desk's page it replaces is desk/src/pages/ThisThread.tsx,
// which stays until atelier-yy4j; its rules are restated here, since the extension may import only
// types from desk/ (src/mission/model.ts, `span`). check/thread.tsx draws it from fixtures
// (test-thread.mjs).
import type { InboxResponse } from "../../../desk/src/api/InboxResponse";
import type { ItemRow } from "../../../desk/src/api/ItemRow";
import type { LiveThread } from "../../../desk/src/api/LiveThread";
import type { MissionPage } from "../../../desk/src/api/MissionPage";
import type { MissionRow } from "../../../desk/src/api/MissionRow";
import type { RecordName } from "../../../desk/src/api/RecordName";
import type { RecordStatus } from "../../../desk/src/api/RecordStatus";
import type { StatusCount } from "../../../desk/src/api/StatusCount";
import type { Tie } from "../../../desk/src/api/Tie";

const seg = encodeURIComponent;
/** The Desk reads the page makes, each one the loader's gate admits (t3-custom.mjs, deskReadUrl). */
export const paths = {
  thread: (identity: string) => `/api/thread?id=${seg(identity)}`,
  ties: (identity: string) => `/api/ties?thread=${seg(identity)}`,
  inbox: () => "/api/inbox",
  record: (id: string) => `/api/records/${seg(id)}`,
  item: (repo: string, id: string) => `/api/repos/${seg(repo)}/items/${seg(id)}`,
  mission: (repo: string, id: string) => `/api/repos/${seg(repo)}/missions/${seg(id)}`,
  missions: (repo: string) => `/api/repos/${seg(repo)}/missions`,
  motion: () => "/api/motion",
};

/** T3 Code's statuses for a thread whose work has ended. */
export const ENDED = new Set(["completed", "cancelled", "failed", "interrupted", "rolled_back"]);
/** A thread that says it runs and has not changed for this long is quiet. */
export const QUIET_MS = 30 * 60 * 1000;
const CLOSED = new Set<string>(["done", "dropped"]);
export const closed = (status: string) => CLOSED.has(status);

/** A status as the CLI names it: `in review`, not `in_review`. */
export const statusName = (status: string) => status.replaceAll("_", " ");
/** The same, starting with a capital. */
export const statusTitle = (status: string) => {
  const name = statusName(status);
  return name.charAt(0).toUpperCase() + name.slice(1);
};
/** `5 open, 1 building`: a mission's items per status, as one line. */
export const countsLine = (counts: StatusCount[]) => (counts.length ? counts.map((c) => `${c.count} ${statusName(c.status)}`).join(", ") : "No items");
export const total = (counts: StatusCount[]) => counts.reduce((sum, c) => sum + c.count, 0);
export const finished = (counts: StatusCount[]) => counts.filter((c) => closed(c.status)).reduce((sum, c) => sum + c.count, 0);

/** T3 Code's live statuses in words; `waiting` is a thread waiting on a person. */
export const threadStatus = (status: string) => ({ preparing: "Preparing", queued: "Queued", starting: "Starting", running: "Running", waiting: "Waiting on a person" })[status] ?? statusTitle(status);

/** `12m`, `3h`, `2d`, or `now` within a minute: how long ago, short. */
export function since(iso: string | null | undefined, now: number): string | null {
  if (!iso) return null;
  const at = Date.parse(iso);
  if (Number.isNaN(at)) return null;
  const s = Math.max(0, (now - at) / 1000);
  if (s < 60) return "now";
  if (s < 3600) return `${Math.round(s / 60)}m`;
  if (s < 172_800) return `${Math.round(s / 3600)}h`;
  return `${Math.round(s / 86_400)}d`;
}

/** A thread identity as a short name, when T3 Code gives no title: the end of its identity. */
export function shortName(identity: string): string {
  const tail = identity.split(/%3A|:/).at(-1) ?? identity;
  return `thread …${tail.slice(-6)}`;
}

/** What a thread is to a record, as a verb, now and before. */
export const VERBS: Record<Tie["role"], [string, string]> = {
  implementer: ["implements", "implemented"],
  reviewer: ["reviews", "reviewed"],
  planner: ["plans", "planned"],
  lead: ["leads", "led"],
  decider: ["answers on", "answered on"],
};

/** `implements atelier-x1, reviewed atelier-x2 (done)`: the ties now, else the last two. */
export function tieLine(ties: Tie[]): string | null {
  const current = ties.filter((t) => t.current);
  const shown = current.length > 0 ? current : ties.slice(-2);
  const seen = new Set<string>();
  const line = shown
    .filter((t) => (seen.has(`${t.role}${t.id}`) ? false : (seen.add(`${t.role}${t.id}`), true)))
    .map((t) => `${VERBS[t.role][t.current ? 0 : 1]} ${t.id}${t.current ? "" : ` (${statusName(t.status)})`}`)
    .join(", ");
  return line || null;
}

/** Each record once, by repository and ID, in the order first given. */
export const unique = (ties: Tie[]) => ties.filter((t, at) => ties.findIndex((u) => u.repo === t.repo && u.id === t.id) === at);

/**
 * Whether a mission's title opens its record page, framed, instead of the mission page: a proposed
 * plan, which the mission page does not serve. It is proposed when its own read says so, when the
 * inbox lists it as a plan under review (a plan on its branch may store `open` already), or, with
 * neither, when it is stored as a draft.
 */
export function proposed(repo: string, id: string, status: RecordStatus, pages: (MissionPage | null)[], inbox: InboxResponse | undefined): boolean {
  const page = pages.find((p) => p?.mission.id === id);
  if (page) return page.mission.state === "proposed";
  if (inbox?.repos.find((r) => r.repo === repo)?.proposed.some((m) => m.id === id)) return true;
  return status === "draft";
}

/** What This thread is about, worked out from its reads; `null` for a read with no answer. */
export type Inputs = {
  thread: string;
  children: string[];
  /** Every tie of the thread; null while unread or unreadable. */
  ties: Tie[] | null;
  /** Each child's ties, by the child's identity; a child missing here is unread or unreadable. */
  childTies: Map<string, Tie[]>;
  /** The mission pages of the missions it holds now, planned or leads. */
  missions: (MissionPage | null)[];
  /** The records its conversation names, as the Desk names them. */
  named: RecordName[];
  inbox: InboxResponse | undefined;
};

export type Arranged = {
  /** Its ties now, items and missions, each record once. */
  itemTies: Tie[];
  missionTies: Tie[];
  /** Missions it planned or leads, done or not. */
  workTies: Tie[];
  /** Records it names that it does not hold, open first, then closed. */
  openMentioned: RecordName[];
  doneMentioned: RecordName[];
  /** The children no record above shows, with their ties. */
  others: { id: string; ties: Tie[] }[];
  waitingPlans: { repo: string; mission: MissionRow }[];
  waitingThreads: LiveThread[];
  waitingItems: { repo: string; row: ItemRow }[];
  /** What it did before, when it holds nothing now. */
  earlier: string | null;
};

export function arrange(input: Inputs): Arranged {
  const all = input.ties ?? [];
  const current = all.filter((t) => t.current);
  const itemTies = unique(current.filter((t) => t.kind === "item"));
  const missionTies = unique(current.filter((t) => t.kind === "mission"));
  const workTies = unique(all.filter((t) => t.kind === "mission" && (t.role === "planner" || t.role === "lead")));
  // A record this thread holds is shown under Holds, not again under Mentioned.
  const held = new Set(current.map((t) => t.id));
  const mentioned = input.named.filter((r) => !held.has(r.id));

  // The records this page is about: what this thread and its children hold, the items of missions
  // it holds, the items of open missions it planned or leads, and what its conversation names.
  const records = new Set<string>([...current.map((t) => t.id), ...mentioned.map((r) => r.id)]);
  for (const ties of input.childTies.values()) for (const tie of ties) if (tie.current) records.add(tie.id);
  const missionIds = new Set(missionTies.map((t) => t.id));
  const workIds = new Set(workTies.map((t) => t.id));
  for (const page of input.missions) {
    if (!page) continue;
    if (missionIds.has(page.mission.id) || (workIds.has(page.mission.id) && page.mission.status !== "done")) for (const i of page.items) records.add(i.id);
  }
  const threads = new Set([input.thread, ...input.children]);
  const repos = input.inbox?.repos ?? [];
  const waitingItems = repos.flatMap((r) => r.needs_human.flatMap((g) => g.items.filter((i) => records.has(i.id) || (g.mission !== null && records.has(g.mission))).map((row) => ({ repo: r.repo, row }))));
  const waitingThreads = repos.flatMap((r) => r.threads.filter((t) => threads.has(t.id) || t.holds.some((h) => records.has(h.id))));
  const waitingPlans = repos.flatMap((r) => r.proposed.filter((m) => records.has(m.id)).map((mission) => ({ repo: r.repo, mission })));

  // A child is listed apart unless all it works on now is drawn above, where its chip or its record shows it.
  const drawn = new Set<string>(current.map((t) => t.id));
  for (const page of input.missions) {
    if (!page) continue;
    drawn.add(page.mission.id);
    for (const i of page.items) drawn.add(i.id);
  }
  const others = input.children
    .map((id) => ({ id, ties: input.childTies.get(id) ?? [] }))
    .filter(({ ties }) => {
      const now = ties.filter((t) => t.current);
      return now.length === 0 || now.some((t) => !drawn.has(t.id));
    });

  return {
    itemTies,
    missionTies,
    workTies,
    openMentioned: mentioned.filter((r) => !closed(r.status)),
    doneMentioned: mentioned.filter((r) => closed(r.status)),
    others,
    waitingPlans,
    waitingThreads,
    waitingItems,
    earlier: itemTies.length + missionTies.length === 0 ? tieLine(all.filter((t) => !t.current)) : null,
  };
}

/** How many rows wait on the human. */
export const waits = (a: Arranged) => a.waitingItems.length + a.waitingThreads.length + a.waitingPlans.length;

/**
 * Whether the page may say the thread holds, started and names nothing: only when every read that
 * could have shown something answered (`failed` is empty) and none did.
 */
export function holdsNothing(a: Arranged, failed: string[]): boolean {
  return failed.length === 0 && a.itemTies.length + a.missionTies.length + a.workTies.length + a.others.length + a.openMentioned.length + a.doneMentioned.length + waits(a) === 0;
}
