// Fixture missions for test-mission.mjs, as the Desk's reads would give them: each scenario maps
// the paths the mission page reads to their answers. The clock is fixed at NOW.
import type { Agent } from "../../desk/src/api/Agent";
import type { HistoryEvent } from "../../desk/src/api/HistoryEvent";
import type { ItemPage } from "../../desk/src/api/ItemPage";
import type { ItemRow } from "../../desk/src/api/ItemRow";
import type { MissionPage } from "../../desk/src/api/MissionPage";
import type { ThreadInfo } from "../../desk/src/api/ThreadInfo";
import { paths } from "../src/mission/model";

export const NOW = Date.parse("2026-11-17T12:00:00Z");
export const REPO = "atelier";
const minutes = (n: number) => new Date(NOW - n * 60_000).toISOString();

function item(id: string, title: string, status: ItemRow["status"], more: Partial<ItemRow> = {}): ItemRow {
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
function agent(identity: string, role: Agent["role"], itemId: string | null, live: boolean): Agent {
  return { identity, role, item: itemId, thread: live ? { title: identity, status: "running", provider: "claudeAgent", model: "claude-opus-5-5", link: null } : null };
}
function thread(id: string, title: string, status: string, changedMinutesAgo: number, startedMinutesAgo: number, lastLine: string): ThreadInfo {
  return { id, title, status, updated_at: minutes(changedMinutesAgo), parent: null, model: "claude-opus-5-5", live: true, started_at: minutes(startedMinutesAgo), last_line: lastLine };
}
const sections = [
  { heading: "Why", markdown: "The page wasted its space." },
  { heading: "Outcome", markdown: "The mission page says what needs you.\n\n- It has **no** progress log.\n- Its rows open `threads` and items." },
  { heading: "Success criteria", markdown: "- needs-you-first: the page lists what needs the human first." },
];
function mission(id: string, title: string, status: MissionPage["mission"]["status"], items: ItemRow[], threads: Agent[], more: Partial<MissionPage> = {}): MissionPage {
  const counts = [...new Set(items.map((i) => i.status))].map((s) => ({ status: s, count: items.filter((i) => i.status === s).length }));
  return {
    commit: "0".repeat(40),
    mission: {
      id,
      title,
      status,
      state: status === "done" ? "done" : "open",
      labels: [],
      lead: threads.find((t) => t.role === "lead")?.identity ?? null,
      created_at: minutes(600),
      in_progress: status === "open",
      counts,
      waiting: items.filter((i) => i.needs_human).map((i) => i.id),
      branch: null,
    },
    sections,
    dropped: null,
    read_at: "0".repeat(40),
    pull_request: null,
    round: null,
    stop_on: null,
    can_stop: false,
    stop_commands: [],
    documents: [],
    items,
    plan_reviews: [],
    threads,
    t3: { reachable: true, age_seconds: 5, error: null },
    fetched: { at: minutes(1), error: null } as unknown as MissionPage["fetched"],
    ...more,
  };
}
const event = (id: string, kind: HistoryEvent["kind"], minutesAgo: number, more: Partial<HistoryEvent> = {}): HistoryEvent => ({
  commit: "0".repeat(40),
  at: minutes(minutesAgo),
  kind,
  id,
  record: null,
  title: null,
  round: null,
  verdict: null,
  subject: `${kind} ${id}`,
  ...more,
});
function itemPage(row: ItemRow, pr: ItemPage["pull_request"]): ItemPage {
  return { row, pull_request: pr } as unknown as ItemPage;
}

type Scenario = { id: string; answers: Record<string, unknown> };
function scenario(page: MissionPage, events: HistoryEvent[], threads: ThreadInfo[], pages: ItemPage[] = []): Scenario {
  const id = page.mission.id;
  const answers: Record<string, unknown> = {
    [paths.mission(REPO, id)]: page,
    [paths.history(REPO, [id, ...page.items.map((i) => i.id)])]: { repo: REPO, events },
  };
  for (const t of threads) answers[paths.thread(t.id)] = t;
  for (const p of pages) answers[paths.item(REPO, p.row.id)] = p;
  return { id, answers };
}

const runningItems = [
  item("atelier-r001", "Decide whether the page keeps a progress log", "building", { needs_human: true }),
  item("atelier-r002", "Draw the lanes from commit subjects", "in_review", { reviewer: "thread:quiet" }),
  item("atelier-r003", "Read every Desk through the loader's gate", "building", { holder: "thread:busy" }),
  item("atelier-r004", "Fold the merged items to one line", "approved", { holder: "thread:merging" }),
  item("atelier-r005", "Show the Result as written", "done"),
  item("atelier-r006", "Stack the page below 560 pixels", "ready", { startable: true }),
  item("atelier-r007", "Name each key on its button", "draft"),
  item("atelier-r008", "Wait for the decision", "ready", { startable: false, waiting_on: ["atelier-r001"] }),
];

export const scenarios: Record<string, Scenario> = {
  // An item labelled needs-human, a reviewer quiet for 47 minutes, and two live threads still changing.
  "needs-you": scenario(
    mission("atelier-rm01", "Make the mission page say what needs you", "open", runningItems, [
      agent("thread:lead", "lead", null, true),
      agent("thread:quiet", "reviewer", "atelier-r002", true),
      agent("thread:busy", "implementer", "atelier-r003", true),
      agent("thread:merging", "implementer", "atelier-r004", true),
      agent("thread:ended", "implementer", "atelier-r005", false),
    ]),
    [event("atelier-rm01", "merged", 300), event("atelier-r005", "started", 240), event("atelier-r005", "merged", 120)],
    [
      thread("thread:lead", "Lead atelier-rm01", "running", 2, 280, "Waiting on r002's review"),
      thread("thread:quiet", "Review atelier-r002", "running", 47, 90, "$ cargo nextest run"),
      thread("thread:busy", "Implement atelier-r003", "running", 1, 38, "$ pnpm vitest run app/src/modules/route"),
      thread("thread:merging", "Merge atelier-r004", "running", 3, 12, "Waiting for CI on #214"),
    ],
  ),
  // No item labelled needs-human; one implementer began waiting on a person a minute ago.
  waiting: scenario(
    mission(
      "atelier-rm02",
      "Read the Desk through the gate",
      "open",
      [item("atelier-w001", "Ask which Desk to read", "building", { holder: "thread:asks" }), item("atelier-w002", "Count the merged items", "done")],
      [agent("thread:asks", "implementer", "atelier-w001", true), agent("thread:planner", "planner", null, false)],
    ),
    [event("atelier-rm02", "merged", 100)],
    [thread("thread:asks", "Implement atelier-w001", "waiting", 1, 20, "Which Desk should the page read?")],
  ),
  // Nothing needs the human: one thread at work, still changing.
  calm: scenario(
    mission(
      "atelier-rm03",
      "Keep the sidebar's count",
      "open",
      [item("atelier-c001", "Count waiting threads", "building", { holder: "thread:calm" }), item("atelier-c002", "Draft the next step", "draft")],
      [agent("thread:lead3", "lead", null, false), agent("thread:calm", "implementer", "atelier-c001", true)],
    ),
    [event("atelier-rm03", "merged", 60)],
    [thread("thread:calm", "Implement atelier-c001", "running", 4, 25, "Reading src/sidebar.ts")],
  ),
  // Done: two merged items, one with three review rounds and its pull request, one with one round
  // whose pull request the Desk does not know (pull_request null).
  done: (() => {
    const d1 = item("atelier-d001", "Let a host theme the embedded Desk", "done");
    const d2 = item("atelier-d002", "Lay the Desk out for a panel", "done");
    return scenario(
      mission("atelier-dm01", "Make the Desk feel native inside T3 Code", "done", [d2, d1], [agent("thread:lead4", "lead", null, true)], {
        sections: [...sections, { heading: "Result", markdown: "The Desk has an embedded mode under `/embed` and a host contract.\n\nAll 7 success criteria are checked by a test." }],
      }),
      [
        event("atelier-dm01", "merged", 600),
        event("atelier-d002", "started", 590),
        event("atelier-d002", "submitted", 560),
        event("atelier-d002", "review", 545, { round: 1, verdict: "approved" }),
        event("atelier-d002", "merged", 540),
        event("atelier-d001", "started", 500),
        event("atelier-d001", "submitted", 450),
        event("atelier-d001", "review", 420, { round: 1, verdict: "changes requested" }),
        event("atelier-d001", "submitted", 380),
        event("atelier-d001", "review", 360, { round: 2, verdict: "changes requested" }),
        event("atelier-d001", "submitted", 330),
        event("atelier-d001", "review", 300, { round: 3, verdict: "approved" }),
        event("atelier-d001", "merged", 290),
        event("atelier-dm01", "closed", 180),
      ].reverse(),
      [],
      [itemPage(d1, { number: 172, url: "https://example.invalid/pull/172", state: "MERGED" }), itemPage(d2, null)],
    );
  })(),
};
