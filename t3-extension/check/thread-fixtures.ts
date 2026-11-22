// Fixture threads for test-thread.mjs, as the Desk's reads would give them: each scenario names the
// thread, the threads it started and the IDs its conversation names, as the extension knows them,
// and maps the paths This thread reads to their answers. The clock is fixed at NOW.
import type { FetchMark } from "../../desk/src/api/FetchMark";
import type { InboxResponse } from "../../desk/src/api/InboxResponse";
import type { ItemPage } from "../../desk/src/api/ItemPage";
import type { ItemRow } from "../../desk/src/api/ItemRow";
import type { LiveThread } from "../../desk/src/api/LiveThread";
import type { MissionPage } from "../../desk/src/api/MissionPage";
import type { MissionsResponse } from "../../desk/src/api/MissionsResponse";
import type { MotionResponse } from "../../desk/src/api/MotionResponse";
import type { RecordName } from "../../desk/src/api/RecordName";
import type { ThreadInfo } from "../../desk/src/api/ThreadInfo";
import type { Tie } from "../../desk/src/api/Tie";
import { paths } from "../src/thread/model";
import { agent, item, minutes, mission, NOW, REPO } from "./mission-fixtures";

export { NOW, REPO };

const fetched: FetchMark = { remote: "origin", at: minutes(2), error: null, every_seconds: 300 } as FetchMark;

function info(id: string, title: string, status: string, changedMinutesAgo: number, model = "claude-opus-5-5"): ThreadInfo {
  return { id, title, status, updated_at: minutes(changedMinutesAgo), parent: null, model, live: !["completed", "failed"].includes(status), started_at: null, last_line: null, pending_requests: null };
}
function tie(kind: Tie["kind"], id: string, title: string, status: Tie["status"], role: Tie["role"], current: boolean): Tie {
  return { repo: REPO, kind, id, title, status, role, current };
}
function inbox(more: { needs?: ItemRow[]; proposed?: MissionPage["mission"][]; threads?: LiveThread[] } = {}): InboxResponse {
  return {
    repos: [{ repo: REPO, commit: "0".repeat(40), error: null, needs_human: more.needs?.length ? [{ mission: null, mission_title: null, items: more.needs }] : [], proposed: more.proposed ?? [], fetched, threads: more.threads ?? [] }],
    t3: { reachable: true, age_seconds: 5, error: null },
    news: [],
  };
}
function itemPage(row: ItemRow, more: Partial<ItemPage> = {}): ItemPage {
  return { row, done_when: [], threads: [], pull_request: null, ...more } as unknown as ItemPage;
}
const record = (kind: RecordName["kind"], id: string, title: string, status: RecordName["status"]): RecordName => ({ repo: REPO, kind, id, title, status });

export type Scenario = { thread: string; children: string[]; mentioned: string[]; answers: Record<string, unknown> };

// Implements an item in building with three Done when lines, one ticked, and an open pull request.
const implemented = item("atelier-ti01", "Draw This thread in T3's panel", "building", { branch: { merged: false, name: "atelier-ti01" }, holder: "thread:impl" });
// Leads a running mission with three items, two with threads on them.
const ledItems = [
  item("atelier-tl01", "Read the ties through the gate", "in_review", { holder: "thread:tl-impl", reviewer: "thread:tl-review" }),
  item("atelier-tl02", "Frame record links in the panel", "building", { holder: "thread:tl-impl2" }),
  item("atelier-tl03", "Write the docs", "done"),
];
const led = mission("atelier-tlm1", "Draw the extension's pages natively", "open", ledItems, [
  agent("thread:lead", "lead", null, true),
  agent("thread:tl-impl", "implementer", "atelier-tl01", true),
  agent("thread:tl-review", "reviewer", "atelier-tl01", true),
  agent("thread:tl-impl2", "implementer", "atelier-tl02", false),
]);
// A plan proposed on its branch: its record stores `open`, its read says `proposed`.
const plan = mission("atelier-tpm1", "Schedule the gardener daily", "open", [item("atelier-tp01", "Write the schedule", "open")], []);
plan.mission.state = "proposed";
plan.mission.in_progress = false;

export const scenarios: Record<string, Scenario> = {
  "implements-item": {
    thread: "thread:impl",
    children: [],
    mentioned: [],
    answers: {
      [paths.thread("thread:impl")]: info("thread:impl", "Implement atelier-ti01", "running", 1),
      [paths.ties("thread:impl")]: { thread: "thread:impl", ties: [tie("item", "atelier-ti01", implemented.title, "building", "implementer", true), tie("item", "atelier-to01", "An item reviewed before", "done", "reviewer", false)] },
      [paths.inbox()]: inbox(),
      [paths.item(REPO, "atelier-ti01")]: itemPage(implemented, {
        done_when: [
          { ticked: true, text: "The panel shows This thread with no iframe." },
          { ticked: false, text: "A read that fails is named." },
          { ticked: false, text: "Screenshots in light and dark." },
        ],
        pull_request: { number: 301, url: "https://example.invalid/pull/301", state: "OPEN" } as ItemPage["pull_request"],
      }),
    },
  },
  "leads-mission": {
    thread: "thread:lead",
    children: ["thread:tl-impl"],
    mentioned: [],
    answers: {
      [paths.thread("thread:lead")]: info("thread:lead", "Lead atelier-tlm1", "running", 2),
      [paths.thread("thread:tl-impl")]: info("thread:tl-impl", "Implement atelier-tl01", "running", 3),
      [paths.ties("thread:lead")]: { thread: "thread:lead", ties: [tie("mission", led.mission.id, led.mission.title, "open", "lead", true)] },
      // Its child works on an item the mission shows, so it is not listed apart.
      [paths.ties("thread:tl-impl")]: { thread: "thread:tl-impl", ties: [tie("item", "atelier-tl01", ledItems[0].title, "in_review", "implementer", true)] },
      [paths.inbox()]: inbox(),
      [paths.mission(REPO, led.mission.id)]: led,
      [paths.thread("thread:tl-impl2")]: info("thread:tl-impl2", "Implement atelier-tl02", "completed", 90),
    },
  },
  // Holds nothing; started two threads, one quiet 47 minutes and one that T3 Code does not know; names an open item, a done item and an agreed mission.
  "children-and-mentions": {
    thread: "thread:parent",
    children: ["thread:c-quiet", "thread:c-gone"],
    mentioned: ["atelier-tm01", "atelier-tm02", "atelier-tmm1", "atelier-tm09"],
    answers: {
      [paths.thread("thread:parent")]: info("thread:parent", "Sweep the backlog", "running", 1),
      [paths.thread("thread:c-quiet")]: info("thread:c-quiet", "Review atelier-tm01", "running", 47, "gpt-6.1-sol"),
      [paths.ties("thread:parent")]: { thread: "thread:parent", ties: [] },
      [paths.ties("thread:c-quiet")]: { thread: "thread:c-quiet", ties: [tie("item", "atelier-tm01", "Name what waits", "in_review", "reviewer", true)] },
      [paths.ties("thread:c-gone")]: { thread: "thread:c-gone", ties: [] },
      [paths.inbox()]: inbox({ needs: [item("atelier-tm01", "Name what waits", "in_review", { needs_human: true })] }),
      [paths.record("atelier-tm01")]: record("item", "atelier-tm01", "Name what waits", "in_review"),
      [paths.record("atelier-tm02")]: record("item", "atelier-tm02", "Fold what is done", "done"),
      [paths.record("atelier-tmm1")]: record("mission", "atelier-tmm1", "An agreed mission", "open"),
      // atelier-tm09 is on no Desk: GET /api/records answers 404, so it is not named and not a failure.
    },
  },
  "planned-proposed": {
    thread: "thread:planner",
    children: [],
    mentioned: [],
    answers: {
      [paths.thread("thread:planner")]: info("thread:planner", "Plan the gardener's schedule", "waiting", 1),
      [paths.ties("thread:planner")]: { thread: "thread:planner", ties: [tie("mission", plan.mission.id, plan.mission.title, "open", "planner", true)] },
      [paths.inbox()]: inbox({ proposed: [plan.mission] }),
      [paths.mission(REPO, plan.mission.id)]: plan,
    },
  },
  nothing: {
    thread: "thread:idle",
    children: [],
    mentioned: [],
    answers: {
      [paths.thread("thread:idle")]: info("thread:idle", "Answer a question", "completed", 5),
      [paths.ties("thread:idle")]: { thread: "thread:idle", ties: [] },
      [paths.inbox()]: inbox({ needs: [item("atelier-tn01", "Choose a provider", "open", { needs_human: true })] }),
      [paths.missions(REPO)]: { commit: "0".repeat(40), missions: [led.mission, plan.mission], loose: 2, fetched } satisfies MissionsResponse,
      [paths.motion()]: {
        repos: [
          {
            repo: REPO,
            commit: null,
            error: null,
            fetched,
            items: [{ id: "atelier-tl02", title: ledItems[1].title, status: "building", mission: led.mission.id, branch: "atelier-tl02", holder: agent("thread:tl-impl2", "implementer", null, false), reviewer: null, changed_at: minutes(20) }],
            missions: [],
            ready: [item("atelier-tn02", "Write the next item", "open", { startable: true })],
          },
        ],
        t3: { reachable: true, age_seconds: 5, error: null },
      } satisfies MotionResponse,
      [paths.thread("thread:tl-impl2")]: info("thread:tl-impl2", "Implement atelier-tl02", "completed", 90),
    },
  },
};

/** Paths the Desk answers 404, as it does for an ID on no repository it serves and a thread T3 Code does not know. */
export const missing = new Set([paths.record("atelier-tm09"), paths.thread("thread:c-gone")]);
