// Fixture repositories for test-home.mjs, as the Desk's reads would give them: each scenario maps
// the paths the project home reads to their answers. The clock is fixed at NOW, in UTC.
import type { Agent } from "../../desk/src/api/Agent";
import type { HistoryEvent } from "../../desk/src/api/HistoryEvent";
import type { ItemRow } from "../../desk/src/api/ItemRow";
import type { MissionPage } from "../../desk/src/api/MissionPage";
import type { MissionRow } from "../../desk/src/api/MissionRow";
import type { MissionsResponse } from "../../desk/src/api/MissionsResponse";
import type { Status } from "../../desk/src/api/Status";
import { paths } from "../src/home/model";

export const NOW = Date.parse("2026-11-17T12:00:00Z");
export const REPO = "atelier";
const minutes = (n: number) => new Date(NOW - n * 60_000).toISOString();

function item(id: string, title: string, status: Status, more: Partial<ItemRow> = {}): ItemRow {
  return {
    id,
    title,
    status,
    priority: "medium",
    labels: [],
    needs_human: false,
    startable: status === "ready",
    blockers: [],
    waiting_on: [],
    holder: null,
    reviewer: null,
    cannot_start: [],
    needs: [],
    branch: null,
    report_written: false,
    dropped: null,
    approved_at: null,
    ...more,
  };
}
function row(id: string, title: string, state: MissionRow["state"], items: ItemRow[], inProgress = false): MissionRow {
  return {
    id,
    title,
    status: state === "proposed" ? "proposed" : "open",
    state,
    labels: [],
    lead: null,
    created_at: minutes(10_000),
    in_progress: inProgress,
    counts: [...new Set(items.map((i) => i.status))].map((status) => ({ status, count: items.filter((i) => i.status === status).length })),
    waiting: items.filter((i) => i.needs_human).map((i) => i.id),
    branch: null,
  } as MissionRow;
}
function page(mission: MissionRow, items: ItemRow[], threads: Agent[] = []): MissionPage {
  return { mission, items, threads, sections: [], plan_reviews: [], documents: [], t3: { reachable: true, age_seconds: 5, error: null } } as unknown as MissionPage;
}
const event = (id: string, kind: HistoryEvent["kind"], record: HistoryEvent["record"], title: string, minutesAgo: number): HistoryEvent => ({
  commit: "0".repeat(40),
  at: minutes(minutesAgo),
  kind,
  id,
  record,
  title,
  round: null,
  verdict: null,
  subject: `${kind} ${id}`,
});
const missions = (rows: MissionRow[]): MissionsResponse => ({ commit: "0".repeat(40), missions: rows, loose: 0, fetched: { remote: "origin", at: minutes(3), error: null, every_seconds: 300 } });

type Scenario = Record<string, unknown>;

// A mission in each of Now, Next and Later, a loose item for each, and a week with steps on three days.
const nowItems = [
  item("atelier-n001", "Draw the lanes", "done"),
  item("atelier-n002", "Fold a mission open", "building", { holder: "thread:impl" }),
  item("atelier-n003", "Read the week", "in_review", { holder: "thread:impl", reviewer: "thread:gone%3Areviewer01" }),
  item("atelier-n004", "Wait for the lanes", "ready", { blockers: [{ id: "atelier-n002", status: "building" }], waiting_on: ["atelier-n002"], startable: false }),
  item("atelier-n005", "Decide the colours", "ready", { needs_human: true }),
];
const nowMission = row("atelier-nm01", "Draw the project home natively", "open", nowItems, true);
const nextItems = [item("atelier-x001", "Write the plan's first item", "ready"), item("atelier-x002", "Then the second", "draft")];
const nextMission = row("atelier-xm01", "Agreed, not started", "open", nextItems);
const laterMission = row("atelier-lm01", "A proposed plan", "proposed", []);
const loose = [
  item("atelier-l001", "A loose item being built", "building", { holder: "thread:loose" }),
  item("atelier-l002", "A loose ready item", "ready"),
  item("atelier-l003", "A loose draft", "draft"),
];
const threads: Agent[] = [
  { identity: "thread:impl", role: "implementer", item: "atelier-n002", thread: { title: "Implement the fold", status: "running", provider: "claudeAgent", model: "claude-opus-5-5", link: null } },
  { identity: "thread:gone%3Areviewer01", role: "reviewer", item: "atelier-n003", thread: null },
];
const full: Scenario = {
  [paths.missions(REPO)]: missions([nowMission, nextMission, laterMission, row("atelier-dm01", "Done and gone", "done", [item("atelier-d001", "Old", "done")])]),
  [paths.roadmap(REPO)]: { repo: REPO, loose },
  [paths.week(REPO)]: {
    repo: REPO,
    // Newest first, as the Desk gives them.
    events: [
      event("atelier-n003", "submitted", "item", "Read the week", 20),
      event("atelier-n001", "merged", "item", "Draw the lanes", 60),
      event("atelier-n001", "started", "item", "Draw the lanes", 300),
      event("atelier-n002", "started", "item", "Fold a mission open", 26 * 60),
      event("atelier-xm01", "merged", "mission", "Agreed, not started", 3 * 24 * 60),
      event("atelier-old1", "merged", "item", "Older than a week", 9 * 24 * 60),
    ],
  },
  [paths.mission(REPO, nowMission.id)]: page(nowMission, nowItems, threads),
  [paths.mission(REPO, nextMission.id)]: page(nextMission, nextItems),
  [paths.mission(REPO, laterMission.id)]: page(laterMission, []),
};

// A repository with nothing open: no mission, no loose item, nothing this week.
const empty: Scenario = {
  [paths.missions(REPO)]: missions([]),
  [paths.roadmap(REPO)]: { repo: REPO, loose: [] },
  [paths.week(REPO)]: { repo: REPO, events: [] },
};

export const scenarios: Record<string, Scenario> = { full, empty };
