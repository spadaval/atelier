// Responses the Playwright suite serves in place of atelier-desk, typed with
// the generated API types so they cannot drift from the server's contract.
// They are modelled on atelier-z4cr, with a review, a finished item, a
// proposed plan, a stop under way and a dispute added so every part of a
// page shows.
import type { ActionRequest } from "../src/api/ActionRequest";
import type { ActionResponse } from "../src/api/ActionResponse";
import type { Agent } from "../src/api/Agent";
import type { ChecksResponse } from "../src/api/ChecksResponse";
import type { DisputedResponse } from "../src/api/DisputedResponse";
import type { DocResponse } from "../src/api/DocResponse";
import type { FetchMark } from "../src/api/FetchMark";
import type { HistoryEvent } from "../src/api/HistoryEvent";
import type { HistoryResponse } from "../src/api/HistoryResponse";
import type { InboxResponse } from "../src/api/InboxResponse";
import type { ItemPage } from "../src/api/ItemPage";
import type { ItemRow } from "../src/api/ItemRow";
import type { MissionPage } from "../src/api/MissionPage";
import type { MissionRow } from "../src/api/MissionRow";
import type { MissionsResponse } from "../src/api/MissionsResponse";
import type { LiveThread } from "../src/api/LiveThread";
import type { MotionResponse } from "../src/api/MotionResponse";
import type { ProposedResponse } from "../src/api/ProposedResponse";
import type { RecordName } from "../src/api/RecordName";
import type { ReposResponse } from "../src/api/ReposResponse";
import type { SessionRequest } from "../src/api/SessionRequest";
import type { SessionResponse } from "../src/api/SessionResponse";
import type { StatusCount } from "../src/api/StatusCount";
import type { T3State } from "../src/api/T3State";
import type { ThreadInfo } from "../src/api/ThreadInfo";
import type { ThreadsResponse } from "../src/api/ThreadsResponse";
import type { Tie } from "../src/api/Tie";

const commit = "2116defdde63afdadad2dddead0945cb50c21c42";
/** The time `seconds` before the fixtures were loaded, as RFC 3339. */
const ago = (seconds: number) => new Date(Date.now() - seconds * 1000).toISOString();
const at = "2026-11-11T03:50:46+00:00";
/** T3 Code read 4 seconds ago. */
const t3: T3State = { reachable: true, age_seconds: 4, error: null };
/** T3 Code never read: these pages say threads are unavailable. */
export const unreachable: T3State = { reachable: false, age_seconds: null, error: "T3 Code did not answer: connection refused" };
/** Fetched from origin two minutes before the page is read. */
export const fetched: FetchMark = { remote: "origin", at: new Date(Date.now() - 120_000).toISOString(), error: null, every_seconds: 300 };
const thread = "thread:delegated-task:command%3Amcp%3Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%3Adelegate-task%3A4e47a5ad";
/** A Codex subagent's identity: a slash, and no T3 thread of its own. */
export const codexChild = "019a6d2e-codex-parent/review-child:7";

/** T3 Code's live threads, as `/api/threads` gives them. */
export const live = {
  uwxy: {
    id: thread,
    title: "Implement atelier-uwxy: the Desk's reading routes",
    status: "running",
    provider: "claudeAgent",
    model: "claude-opus-5-5",
    parent: "thread:orchestrator",
    pull_requests: [{ repository: "spadaval/atelier", number: 131, url: "https://github.com/spadaval/atelier/pull/131", state: "OPEN" }],
    link: "https://app.t3.codes/threads/env-1/thread%3Adelegated-task%3Acommand%253Amcp",
    updated_at: at,
    holds: [{ repo: "atelier", kind: "item", id: "atelier-uwxy", role: "implementer" }],
  },
  asking: {
    id: "thread:aab1-probe",
    title: "Find how the Desk reads T3 threads",
    status: "waiting",
    provider: "codex",
    model: "gpt-6.1-sol",
    parent: null,
    pull_requests: [],
    link: "https://app.t3.codes/threads/env-1/thread%3Aaab1-probe",
    updated_at: at,
    holds: [{ repo: "atelier", kind: "item", id: "atelier-aab1", role: "implementer" }],
  },
  lor: {
    id: "thread:lor-lead",
    title: "Lead the LoR habits mission",
    status: "running",
    provider: "claudeAgent",
    model: "claude-fable-5-2",
    parent: null,
    pull_requests: [],
    link: "https://app.t3.codes/threads/env-1/thread%3Alor-lead",
    updated_at: at,
    holds: [{ repo: "lor", kind: "mission", id: "lor-h4bt", role: "lead" }],
  },
  free: {
    id: "thread:scratch",
    title: "Tidy the dotfiles",
    status: "queued",
    provider: "claudeAgent",
    model: "claude-opus-5-5",
    parent: null,
    pull_requests: [],
    link: null,
    updated_at: at,
    holds: [],
  },
} satisfies Record<string, LiveThread>;

const mark = (t: LiveThread) => ({ title: t.title, status: t.status, provider: t.provider, model: t.model, link: t.link });

/** The agents of atelier-z4cr's items: one live implementer, a waiting one, and identities T3 Code does not list. */
const z4crAgents: Agent[] = [
  { identity: thread, role: "implementer", item: "atelier-uwxy", thread: mark(live.uwxy) },
  { identity: live.asking.id, role: "implementer", item: "atelier-aab1", thread: mark(live.asking) },
  { identity: codexChild, role: "reviewer", item: "atelier-1ie1", thread: null },
  { identity: "sumeet", role: "decider", item: "atelier-1ie1", thread: null },
  { identity: "planner-thread", role: "planner", item: null, thread: null },
];

function row(id: string, title: string, status: ItemRow["status"], more: Partial<ItemRow> = {}): ItemRow {
  return {
    id,
    title,
    status,
    priority: "high",
    labels: [],
    needs_human: false,
    startable: false,
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

function counts(items: ItemRow[]): StatusCount[] {
  const order = ["draft", "ready", "building", "in_review", "changes_requested", "approved", "done", "dropped"] as const;
  return order.map((status) => ({ status, count: items.filter((i) => i.status === status).length })).filter((c) => c.count > 0);
}

function mission(id: string, title: string, state: MissionRow["state"], items: ItemRow[], more: Partial<MissionRow> = {}): MissionRow {
  const status: MissionRow["status"] = state === "done" ? "done" : state === "stopped" ? "dropped" : "open";
  return {
    id,
    title,
    status,
    state,
    labels: [],
    lead: null,
    created_at: at,
    in_progress: state === "open" && items.some((i) => ["building", "in_review", "changes_requested", "approved"].includes(i.status)),
    counts: counts(items),
    waiting: items.filter((i) => i.needs_human).map((i) => i.id),
    branch: state === "proposed" ? `origin/${id}` : null,
    ...more,
  };
}

export const z4crItems: ItemRow[] = [
  row("atelier-94pv", "Move the spike's app to desk/ on the new API", "draft", {
    blockers: [{ id: "atelier-uwxy", status: "building" }],
    waiting_on: ["atelier-uwxy"],
  }),
  row("atelier-e05n", "Show the agents working on a mission and its items", "draft", { priority: "medium" }),
  row("atelier-h3hl", "Let Sumeet start a planning session from a page", "draft", { priority: "medium" }),
  row("atelier-va99", "Install the Desk as a service on this host, on converted clones", "draft", { priority: "medium" }),
  row("atelier-qxmr", "Point the Desk at the live repositories after this host's cutover", "draft", { priority: "low" }),
  row("atelier-psjw", "Move the actions and repository access into atelier-core", "ready", {
    cannot_start: ["atelier-psjw is blocked by atelier-uwxy"],
  }),
  row("atelier-aab1", "Find how atelier serve can read T3 thread status", "ready", { labels: ["needs-human"], needs_human: true, startable: true }),
  row("atelier-uwxy", "Serve the Desk's reading routes from commits: the atelier-desk crate", "building", {
    holder: thread,
    branch: { merged: false, name: "origin/atelier-uwxy" },
    needs: ["atelier-uwxy needs its Report written"],
  }),
  row("atelier-1ie1", "Write the Desk's API as a document first", "done", { branch: { merged: true, name: "atelier-1ie1" }, report_written: true, approved_at: at }),
];

function stopCommands(id: string): string[] {
  return [`atelier start --backlog stop-${id}`, `atelier drop ${id} --reason "<why>"`, `git commit -m "Drop ${id}" .atelier/missions/${id}/mission.md`, "atelier submit", "atelier merge"];
}

export const z4cr: MissionPage = {
  commit,
  mission: mission("atelier-z4cr", "Build the Desk: the human's place to plan missions and read results, on the redesigned records", "open", z4crItems),
  sections: [
    {
      heading: "Why",
      markdown:
        "Sumeet has no tool of his own. A page that lists records asks him to read records.\n[ADR 0032](../../../docs/adr/0032-the-desk-is-the-humans-product.md) makes the human's tool a product of its own, and [its intent](../../../docs/product/desk.md) says what it is for.",
    },
    { heading: "Outcome", markdown: "On this host, a service reached over the tailnet shows Atelier repositories on the redesigned records." },
    { heading: "Success criteria", markdown: "- desk-matches-cli: every open mission has a page that shows what `atelier show` prints.\n- action-is-a-record: a stop leaves the commit `atelier drop` leaves." },
    { heading: "Scope", markdown: "- In: the `atelier-desk` crate and the app.\n- Out: health, the feed and any database." },
    { heading: "Authority", markdown: "The orchestrator may choose the API's routes and fields." },
  ],
  dropped: null,
  read_at: commit,
  pull_request: null,
  round: { round: 2, reviews: [{ name: "plan-r2", reviewer: "codex-reviewer-2", model: "gpt-6.1-sol", verdict: "approve" }] },
  stop_on: null,
  can_stop: false,
  stop_commands: stopCommands("atelier-z4cr"),
  fetched,
  documents: [
    { path: ".atelier/missions/atelier-z4cr/design.md", title: "Sizing" },
    { path: ".atelier/missions/atelier-z4cr/notes.md", title: "Notes of atelier-z4cr" },
  ],
  items: z4crItems,
  threads: z4crAgents,
  t3,
  plan_reviews: [
    {
      name: "plan-r1",
      round: 1,
      commit: "009cdab9c0ffee00000000000000000000000000",
      reviewers: ["codex-reviewer"],
      model: "gpt-6.1-sol",
      verdict: "changes",
      created_at: at,
      findings: [
        {
          name: "1-pl4n.1",
          title: "The strategy does not say where the app's types come from",
          recommendation: "fix_now",
          was: null,
          now: null,
          decided: null,
          settlement: null,
          text: "Name the generator and the test that keeps the types current.",
          open: false,
          deferred: false,
        },
      ],
      changed_since: [".atelier/missions/atelier-z4cr/design.md"],
    },
    {
      name: "plan-r2",
      round: 2,
      commit: "0abc1234c0ffee00000000000000000000000000",
      reviewers: ["codex-reviewer-2"],
      model: "gpt-6.1-sol",
      verdict: "approve",
      created_at: at,
      findings: [
        {
          name: "2-pl4n.1",
          title: "The types' source is named now",
          recommendation: "fix_now",
          was: "1-pl4n.1",
          now: "fixed",
          decided: null,
          settlement: null,
          text: null,
          open: false,
          deferred: false,
        },
      ],
      changed_since: null,
    },
  ],
};

/** atelier-295u's plan: proposed on its branch, with a pull request and a split first round. */
export const planPr = { number: 412, url: "https://github.com/spadaval/atelier/pull/412", state: "OPEN" };

const otherMissions: [string, string, MissionRow["state"], ItemRow[], Partial<MissionRow>?][] = [
  ["atelier-295u", "Give the gardener `atelier log` and a readiness checklist", "proposed", [row("atelier-k1aa", "Write the log", "draft")]],
  ["atelier-k9rf", "Try a plan whose stop is refused", "proposed", [row("atelier-k9r1", "Write the plan", "draft")]],
  ["atelier-g6rp", "Ship Agent Factory from this repository", "open", []],
  ["atelier-st0p", "A mission someone is stopping", "stop_pending", []],
  ["atelier-pr0x", "A plan whose pull request will not close", "proposed", []],
  [
    "atelier-qr5s",
    "Move all five repositories and Agent Factory onto the new loop, one cutover per host",
    "open",
    [row("atelier-u1ko", "Delete the old workspace", "in_review", { reviewer: "codex-reviewer" }), row("atelier-wd1i", "Run the cutover on this host", "ready")],
  ],
];

export const missionPages: Record<string, MissionPage> = { [z4cr.mission.id]: z4cr };
for (const [id, title, state, items, more] of otherMissions) {
  const proposed = state === "proposed";
  missionPages[id] = {
    commit,
    mission: mission(id, title, state, items, more),
    sections: [{ heading: "Why", markdown: `${title}.` }],
    dropped: null,
    read_at: proposed ? "295a000000000000000000000000000000000000" : commit,
    pull_request: id === "atelier-295u" ? planPr : null,
    round:
      id === "atelier-295u"
        ? {
            round: 1,
            reviews: [
              { name: "1-aaaa", reviewer: "codex-reviewer", model: "gpt-6.1-sol", verdict: "approve" },
              { name: "1-bbbb", reviewer: "grok-reviewer", model: "grok-4.6", verdict: "changes" },
            ],
          }
        : null,
    stop_on: state === "stop_pending" ? "origin/backlog/stop-atelier-st0p" : null,
    can_stop: proposed,
    stop_commands: state === "open" ? stopCommands(id) : [],
    fetched,
    documents: [],
    items,
    // atelier-qr5s is read while T3 Code cannot be reached.
    threads: id === "atelier-qr5s" ? [{ identity: codexChild, role: "reviewer", item: "atelier-u1ko", thread: null }] : [],
    t3: id === "atelier-qr5s" ? unreachable : t3,
    plan_reviews: [],
  };
}

export const missions: MissionsResponse = { commit, missions: Object.values(missionPages).map((p) => p.mission), loose: 80, fetched };

export const repos: ReposResponse = {
  repos: [
    { name: "atelier", path: "/projects/atelier", identity: "github.com/spadaval/atelier", base: "master", base_tip: commit, in_progress: 2, proposed: 2, waiting: 1, error: null, fetched },
    {
      name: "broken",
      path: "/projects/broken",
      identity: null,
      base: null,
      base_tip: null,
      in_progress: 0,
      proposed: 0,
      waiting: 0,
      error: "`.atelier/items/x/item.md` does not read: missing field `status`",
      fetched: { remote: null, at: null, error: null, every_seconds: 300 },
    },
  ],
  built_from: "5e1f0c2d9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d",
  caller: "sumeet",
};

function itemPage(r: ItemRow, more: Partial<ItemPage>): ItemPage {
  return {
    commit,
    row: r,
    mission: "atelier-z4cr",
    blocked_by: r.blockers.map((b) => b.id),
    implementers: [],
    ready: { by: "sumeet", at },
    trivial: null,
    caused_by: null,
    created_at: at,
    read_from: "base",
    body: `# ${r.title}\n\n## Goal\n\nThe goal.\n`,
    goal: "The goal.",
    scope: null,
    report: null,
    done_when: [],
    decisions: [],
    responses: [],
    reviews: [],
    unsettled: [],
    threads: [],
    t3,
    fetched,
    pull_request: null,
    ...more,
  };
}

const done = z4crItems.find((i) => i.id === "atelier-1ie1")!;
function building() {
  return z4crItems.find((i) => i.id === "atelier-uwxy")!;
}

export const items: Record<string, ItemPage> = {
  "atelier-1ie1": itemPage(done, {
    implementers: [thread],
    goal: "Write the Desk's API as a document before any server code.",
    done_when: [
      { ticked: true, text: "`docs/architecture/desk.md` has the route table." },
      { ticked: true, text: "Every field names the function that gives it." },
    ],
    report: "All Done when lines hold. The document maps every field to the core.\n\n### Validation\n\nRead against master.",
    reviews: [
      {
        name: "r1",
        round: 1,
        commit: "1111aaaa00000000000000000000000000000000",
        reviewers: ["codex-reviewer"],
        model: "gpt-6.1-sol",
        verdict: "changes",
        created_at: at,
        findings: [
          {
            name: "1-aaaa.1",
            title: "The inbox route omits draft missions",
            recommendation: "fix_now",
            was: null,
            now: null,
            decided: null,
            settlement: null,
            text: "Add `drafts` to the inbox response.",
            open: false,
            deferred: false,
          },
          {
            name: "1-aaaa.3",
            title: "The thread join splits identities on slashes",
            recommendation: "fix_now",
            was: null,
            now: null,
            decided: null,
            settlement: null,
            text: null,
            open: false,
            deferred: false,
          },
          {
            name: "1-aaaa.2",
            title: "The history route has no owner",
            recommendation: "defer",
            was: null,
            now: null,
            decided: null,
            settlement: { kind: "taken", item: "atelier-e05n", by: "sumeet" },
            text: null,
            open: false,
            deferred: true,
          },
        ],
        changed_since: null,
      },
      {
        name: "r2",
        round: 2,
        commit: "2222bbbb00000000000000000000000000000000",
        reviewers: ["codex-reviewer-2"],
        model: "gpt-6.1-sol",
        verdict: "approve",
        created_at: at,
        findings: [
          {
            name: "2-bbbb.1",
            title: "Drafts are in the inbox now",
            recommendation: "fix_now",
            was: "1-aaaa.1",
            now: "fixed",
            decided: null,
            settlement: null,
            text: null,
            open: false,
            deferred: false,
          },
          {
            name: "2-bbbb.2",
            title: "Identities are not split after all",
            recommendation: "fix_now",
            was: "1-aaaa.3",
            now: "withdrawn",
            decided: null,
            settlement: null,
            text: null,
            open: false,
            deferred: false,
          },
        ],
        changed_since: null,
      },
    ],
    responses: [{ kind: "fixed", finding: "1-aaaa.1", commit: "3333cccc00000000000000000000000000000000" }],
  }),
  "atelier-uwxy": itemPage(building(), {
    read_from: "branch",
    pull_request: { number: 131, url: "https://github.com/spadaval/atelier/pull/131", state: "OPEN" },
    implementers: [thread],
    threads: [
      { identity: thread, role: "implementer", item: null, thread: mark(live.uwxy) },
      { identity: codexChild, role: "reviewer", item: null, thread: null },
    ],
    goal: "Serve the reading routes. See [the API](../../../docs/architecture/desk.md#routes).",
    scope: "`crates/atelier-desk` only.",
    done_when: [
      { ticked: true, text: "`GET /api/repos` answers." },
      { ticked: false, text: "`GET /api/inbox` answers." },
    ],
    decisions: [{ item: "atelier-uwxy", question: "Should the cache key include the checkout's HEAD?", finding: "1-cccc.2", answer: { by: "sumeet", text: "No." }, outcome: null }],
  }),
};

const inReview = otherMissions.find(([id]) => id === "atelier-qr5s")![3][0];
items["atelier-u1ko"] = itemPage(inReview, {
  mission: "atelier-qr5s",
  threads: [{ identity: codexChild, role: "reviewer", item: null, thread: null }],
  t3: unreachable,
});

export const docs: Record<string, DocResponse> = {
  ".atelier/missions/atelier-z4cr/design.md": {
    commit,
    path: ".atelier/missions/atelier-z4cr/design.md",
    markdown:
      '---\nstatus: draft\nlast_changed: "2026-11-11: amended at Sumeet\'s direction"\n---\n\n# Sizing\n\n- Complexity: high.\n\nThe notes are in [notes](notes.md), and the product intent in [the intent](../../../docs/product/desk.md#why-it-exists). The server is [api.rs](../../../crates/atelier-desk/src/api.rs).\n\n# Risks and Unknowns\n\n- The view gap is large.\n',
  },
  ".atelier/missions/atelier-z4cr/notes.md": {
    commit,
    path: ".atelier/missions/atelier-z4cr/notes.md",
    markdown: "# Notes of atelier-z4cr\n\nWhat the old mission record held beyond its mission sections. Back to [the design](design.md).\n",
  },
  "docs/product/desk.md": {
    commit,
    path: "docs/product/desk.md",
    markdown: "# The Desk: product intent\n\n## Why it exists\n\nAtelier has two kinds of user.\n",
  },
  "docs/adr/0032-the-desk-is-the-humans-product.md": {
    commit,
    path: "docs/adr/0032-the-desk-is-the-humans-product.md",
    markdown: "# ADR 0032: The Desk is the human's product\n\nDecided.\n",
  },
};

export const inbox: InboxResponse = {
  repos: [
    {
      repo: "atelier",
      commit,
      error: null,
      needs_human: [
        { mission: "atelier-z4cr", mission_title: z4cr.mission.title, items: [z4crItems.find((i) => i.id === "atelier-aab1")!] },
        { mission: null, mission_title: null, items: [row("atelier-pywh", "Read the fleet site on your phone for taste", "ready", { labels: ["needs-human"], needs_human: true, priority: "low", startable: true })] },
      ],
      proposed: [missionPages["atelier-295u"].mission, missionPages["atelier-k9rf"].mission],
      threads: [live.asking],
      fetched,
    },
    {
      repo: "broken",
      commit: null,
      error: "`.atelier/items/x/item.md` does not read: missing field `status`",
      needs_human: [],
      proposed: [],
      threads: [],
      fetched: { remote: null, at: null, error: null, every_seconds: 300 },
    },
  ],
  t3,
  news: [
    { repo: "atelier", mission: "atelier-295u", title: missionPages["atelier-295u"].mission.title, kind: "proposed", at, link: "http://desk:3780/r/atelier/mission/atelier-295u" },
    { repo: "atelier", mission: "atelier-z4cr", title: z4cr.mission.title, kind: "merged", at, link: "http://desk:3780/r/atelier/mission/atelier-z4cr" },
  ],
};

export const proposed: ProposedResponse = {
  repos: [
    {
      repo: "atelier",
      commit,
      error: null,
      fetched,
      plans: [
        ...["atelier-295u", "atelier-k9rf"].map((id) => ({ mission: missionPages[id].mission, amends: false, pull_request: missionPages[id].pull_request, round: missionPages[id].round })),
        { mission: { ...missionPages["atelier-g6rp"].mission, branch: "origin/atelier-g6rp" }, amends: true, pull_request: null, round: null },
      ],
      changes: [
        {
          branch: "origin/backlog/file-bugs",
          pull_request: { number: 415, url: "https://github.com/spadaval/atelier/pull/415", state: "OPEN" },
          records: [
            { kind: "item", id: "atelier-n3w1", title: "Fix the fetch timer", change: "added" },
            { kind: "item", id: "atelier-94pv", title: "Move the spike's app to desk/ on the new API", change: "dropped" },
          ],
        },
      ],
    },
  ],
};

export const motion: MotionResponse = {
  repos: [
    {
      repo: "atelier",
      commit,
      error: null,
      fetched,
      items: [
        {
          id: "atelier-uwxy",
          title: building().title,
          status: "building",
          mission: "atelier-z4cr",
          branch: "origin/atelier-uwxy",
          holder: { identity: thread, role: "implementer", item: "atelier-uwxy", thread: mark(live.uwxy) },
          reviewer: null,
          changed_at: ago(25 * 60),
        },
        {
          id: "atelier-u1ko",
          title: "Delete the old workspace",
          status: "in_review",
          mission: "atelier-qr5s",
          branch: "atelier-u1ko",
          holder: { identity: "thread:gone", role: "implementer", item: "atelier-u1ko", thread: null },
          reviewer: { identity: codexChild, role: "reviewer", item: "atelier-u1ko", thread: null },
          changed_at: ago(3 * 3600),
        },
      ],
      missions: [{ mission: z4cr.mission, lead: { identity: live.lor.id, role: "lead", item: null, thread: mark(live.lor) } }],
      ready: [row("atelier-wd1i", "Run the cutover on this host", "ready", { startable: true, priority: "medium" })],
    },
  ],
  t3,
};

export const disputed: DisputedResponse = {
  repos: [
    {
      repo: "atelier",
      commit,
      error: null,
      fetched,
      rebuttals: [
        {
          item: "atelier-uwxy",
          mission: "atelier-z4cr",
          finding: "1-cccc.2",
          title: "The cache key leaves out HEAD",
          fact: "HEAD is in the key since 3f2a",
          implementer: { identity: thread, model: "claude-opus-5-5", provider: "claudeAgent", link: live.uwxy.link },
          raised_by: { identity: "codex-reviewer", model: "gpt-6.1-sol", provider: null, link: null },
          judged: [{ finding: "2-dddd.1", judged: "withdrawn", reviewer: { identity: "codex-reviewer-2", model: "gpt-6.1-sol", provider: null, link: null } }],
        },
      ],
      plans: [
        {
          mission: "atelier-295u",
          round: 1,
          reviews: [
            { name: "1-aaaa", reviewer: { identity: "codex-reviewer", model: "gpt-6.1-sol", provider: null, link: null }, verdict: "approve" },
            { name: "1-bbbb", reviewer: { identity: "grok-reviewer", model: null, provider: null, link: null }, verdict: "changes" },
          ],
        },
      ],
    },
  ],
  t3,
};

export const threads: ThreadsResponse = { t3, threads: [live.uwxy, live.asking, live.lor, live.free] };

/** What the fixture server answers to each action it is sent. */
export const answers = {
  stopped: {
    result: "done",
    text: "Dropped the proposed plan of mission atelier-295u: Not this month\nPushed atelier-295u to origin and closed its pull request. The plan is stopped.",
    paths: [".atelier/missions/atelier-295u/mission.md"],
    commit: "76cbb6c0ffee0000000000000000000000000000",
  },
  stopHalfDone: {
    result: "done",
    text: "Dropped the proposed plan of mission atelier-pr0x: Not now\nPushed atelier-pr0x to origin, but its pull request was not closed: gh pr close failed: HTTP 403. The stop is published once it is closed: gh pr close atelier-pr0x.",
    paths: [".atelier/missions/atelier-pr0x/mission.md"],
    commit: "88aa000000000000000000000000000000000000",
  },
  stopRefused: {
    result: "refused",
    rules: [],
    text: "This repository names no forge remote, so a stop of atelier-k9rf is published by committing the drop on its branch in the planner's checkout. Nothing changed.",
    next: "There: atelier drop atelier-k9rf --reason \"<why>\", then commit it.",
  },
  started: { result: "started", thread_id: "thread:plan-z4cr", link: "https://app.t3.codes/threads/env-1/thread%3Aplan-z4cr", project_id: "project-atelier" },
  sessionRefused: { result: "refused", text: "No T3 Code project has /projects/atelier as its workspace root, so no session can start there." },
} satisfies Record<string, ActionResponse | SessionResponse>;

/** Threads a host may show, by what they are tied to, for `/api/ties`. */
export const tied = {
  implementer: thread,
  lead: "thread:z4cr-lead",
  /** Implemented atelier-1ie1, now done, and reviews atelier-u1ko's open round. */
  pastAndReview: "thread:1ie1-then-u1ko",
  /** Implemented and reviewed only work that is done. */
  pastOnly: "thread:1ie1-only",
  none: live.free.id,
  /** A thread whose ties cannot be read: `/api/ties` answers 503. */
  unreadable: "thread:ties-unreadable",
};

const tie = (kind: Tie["kind"], id: string, title: string, status: Tie["status"], role: Tie["role"], current: boolean): Tie => ({ repo: "atelier", kind, id, title, status, role, current });
const u1ko = "Delete the old workspace";
const ie1 = "Write the Desk's API as a document first";

/** `/api/ties` for each identity in `tied`; any other identity is tied to nothing. */
export const ties: Record<string, Tie[]> = {
  [tied.implementer]: [tie("item", "atelier-uwxy", "Serve the Desk's reading routes from commits: the atelier-desk crate", "building", "implementer", true)],
  [tied.lead]: [tie("mission", "atelier-z4cr", z4cr.mission.title, "open", "lead", true)],
  [tied.pastAndReview]: [tie("item", "atelier-1ie1", ie1, "done", "implementer", false), tie("item", "atelier-u1ko", u1ko, "in_review", "reviewer", true)],
  [tied.pastOnly]: [tie("item", "atelier-1ie1", ie1, "done", "implementer", false), tie("item", "atelier-1ie1", ie1, "done", "reviewer", false)],
};

/** A request the app sent to an action or session route. */
export type Sent = { path: string; header: string | undefined; body: unknown };

/**
 * The fixture server: every response above, and actions that change them as
 * atelier-desk would. Each test gets its own, so an action never leaks into
 * another test.
 */
export class World {
  repos = structuredClone(repos);
  inbox = structuredClone(inbox);
  threads = structuredClone(threads);
  missionPages = structuredClone(missionPages);
  items = structuredClone(items);
  sent: Sent[] = [];
  /** The origins that may frame the Desk, as the configuration's `embed_origins` lists them. */
  embedOrigins: string[] = [];
  /** The query of each document read. */
  docsRead: string[] = [];

  missions(): MissionsResponse {
    return { commit, missions: Object.values(this.missionPages).map((p) => p.mission), loose: 80, fetched };
  }

  item(id: string): ItemPage | undefined {
    if (this.items[id]) return this.items[id];
    const known = Object.values(this.missionPages)
      .flatMap((p) => p.items)
      .find((i) => i.id === id);
    return known ? itemPage(known, {}) : undefined;
  }

  /** What `/api/records/<id>` names: an item or mission of atelier. */
  record(id: string): RecordName | null {
    const mission = this.missionPages[id];
    if (mission) return { repo: "atelier", kind: "mission", id, title: mission.mission.title, status: mission.mission.status };
    const item = this.item(id);
    return item ? { repo: "atelier", kind: "item", id, title: item.row.title, status: item.row.status } : null;
  }

  /** The response for one API request, as atelier-desk would give it. */
  respond(method: string, url: URL, header: string | undefined, body: string | null): { status: number; body: unknown } {
    const path = url.pathname;
    if (method === "POST") {
      const parsed: unknown = body ? JSON.parse(body) : null;
      this.sent.push({ path, header, body: parsed });
      if (header !== "1") return { status: 403, body: { error: "An action needs the header x-atelier-desk: 1." } };
      if (path === "/api/action") return { status: 200, body: this.action(parsed as ActionRequest) };
      if (path === "/api/sessions") return { status: 200, body: this.session(parsed as SessionRequest) };
      return { status: 404, body: { error: "No route" } };
    }
    return this.read(url);
  }

  private action(request: ActionRequest): ActionResponse {
    if (request.id === "atelier-pr0x") {
      // Pushed, and its pull request is still open.
      const page = this.missionPages["atelier-pr0x"];
      page.mission = { ...page.mission, status: "dropped", state: "stop_pending" };
      page.dropped = request.reason;
      page.pull_request = { number: 413, url: "https://github.com/spadaval/atelier/pull/413", state: "OPEN" };
      page.can_stop = false;
      return answers.stopHalfDone;
    }
    if (request.id !== "atelier-295u") return answers.stopRefused;
    // The stop is pushed and its pull request closed: stopped, and no longer proposed.
    const page = this.missionPages["atelier-295u"];
    page.mission = { ...page.mission, status: "dropped", state: "stopped" };
    page.dropped = request.reason;
    page.pull_request = { ...planPr, state: "CLOSED" };
    page.can_stop = false;
    for (const r of this.inbox.repos) r.proposed = r.proposed.filter((m) => m.id !== "atelier-295u");
    for (const r of this.repos.repos) if (r.name === "atelier") r.proposed -= 1;
    return answers.stopped;
  }

  private session(request: SessionRequest): SessionResponse {
    return request.mission === "atelier-z4cr" ? answers.started : answers.sessionRefused;
  }

  private read(url: URL): { status: number; body: unknown } {
    const path = url.pathname;
    const missing = (what: string) => ({ status: 404, body: { error: `No ${what}` } });
    if (path === "/api/repos") return { status: 200, body: this.repos };
    if (path === "/api/embed") return { status: 200, body: { origins: this.embedOrigins } };
    if (path === "/api/ties") {
      const identity = url.searchParams.get("thread");
      if (!identity) return { status: 400, body: { error: "A thread identity is needed." } };
      if (identity === tied.unreadable) return { status: 503, body: { error: "The records could not be read." } };
      return { status: 200, body: { thread: identity, ties: ties[identity] ?? [] } };
    }
    if (path === "/api/inbox") return { status: 200, body: this.inbox };
    if (path === "/api/thread") {
      const found = threadInfo(url.searchParams.get("id") ?? "", url.searchParams.has("full"));
      return found ? { status: 200, body: found } : missing("thread");
    }
    const record = /^\/api\/records\/([^/]+)$/.exec(path);
    if (record) {
      const found = this.record(decodeURIComponent(record[1]));
      return found ? { status: 200, body: found } : missing(`record ${record[1]}`);
    }
    if (path === "/api/history" || path === "/api/roadmap" || path === "/api/checks") {
      if (url.searchParams.get("repo") !== "atelier") return missing(`repository ${url.searchParams.get("repo")}`);
      if (path === "/api/roadmap") return { status: 200, body: { repo: "atelier", loose } };
      if (path === "/api/checks") return { status: 200, body: checks };
      return { status: 200, body: history(url.searchParams.get("days"), url.searchParams.get("ids")) };
    }
    if (path === "/api/threads") return { status: 200, body: this.threads };
    if (path === "/api/proposed") return { status: 200, body: proposed };
    if (path === "/api/motion") return { status: 200, body: motion };
    if (path === "/api/disputed") return { status: 200, body: disputed };
    const repo = /^\/api\/repos\/([^/]+)(\/.*)$/.exec(path);
    if (!repo) return missing("route");
    if (repo[1] !== "atelier") return missing(`repository ${repo[1]}`);
    const rest = repo[2];
    if (rest === "/missions") return { status: 200, body: this.missions() };
    if (rest === "/doc") {
      this.docsRead.push(url.search);
      const doc = docs[url.searchParams.get("path") ?? ""];
      return doc ? { status: 200, body: doc } : missing("document");
    }
    const mission = /^\/missions\/([^/]+)$/.exec(rest);
    if (mission) return this.missionPages[mission[1]] ? { status: 200, body: this.missionPages[mission[1]] } : missing(`mission ${mission[1]}`);
    const item = /^\/items\/([^/]+)$/.exec(rest);
    if (item) {
      const page = this.item(item[1]);
      return page ? { status: 200, body: page } : missing(`item ${item[1]}`);
    }
    return missing("route");
  }
}

/** Threads T3 Code knows that are no longer live, for `/api/thread`. */
export const ended: Record<string, ThreadInfo> = {
  "thread:gone": { id: "thread:gone", title: "Implement atelier-u1ko", status: "completed", updated_at: ago(5 * 3600), parent: "thread:orchestrator", model: "claude-opus-5-5", live: false, started_at: null, last_line: null },
  "planner-thread": { id: "planner-thread", title: "Plan the Desk", status: "completed", updated_at: ago(30 * 3600), parent: null, model: "claude-fable-5-2", live: false, started_at: null, last_line: null },
  "thread:child-quiet": { id: "thread:child-quiet", title: "Look into the flaky e2e test", status: "running", updated_at: ago(2 * 3600), parent: thread, model: "gpt-6.1-sol", live: false, started_at: null, last_line: null },
};

/** `/api/thread`: a live thread from the list, else one that has ended; in full, with its latest run and line. */
export function threadInfo(id: string, full: boolean): ThreadInfo | null {
  const listed = Object.values(live).find((t) => t.id === id);
  const found: ThreadInfo | null = listed
    ? { id, title: listed.title, status: listed.status, updated_at: listed.updated_at, parent: listed.parent, model: listed.model, live: true, started_at: null, last_line: null }
    : (ended[id] ?? null);
  if (!found || !full) return found;
  return { ...found, started_at: ago(40 * 60), last_line: found.live ? "Bash: cargo nextest run -p atelier-desk" : "Merged and pushed." };
}

/** The items in no mission, for `/api/roadmap`. */
export const loose: ItemRow[] = [
  row("atelier-c1z3", "Ready the scorecard scripts and rubric for the rerun", "ready", { startable: true }),
  row("atelier-7w9s", "Convert the work host with the cutover runbook", "ready", {
    labels: ["needs-human"],
    needs_human: true,
    blockers: [{ id: "atelier-52g7", status: "draft" }],
    waiting_on: ["atelier-52g7"],
  }),
  row("atelier-52g7", "Make the cutover find the docs that name removed commands", "draft", { priority: "medium" }),
  row("atelier-0019", "Garden atelier, 2026-11-11", "approved", { holder: "thread:gone" }),
];

/** CI on a pull request, for `/api/checks`. */
export const checks: ChecksResponse = { state: "passing", checks: [{ name: "rust", state: "SUCCESS" }, { name: "desk", state: "SUCCESS" }, { name: "docs", state: "SKIPPED" }] };

function event(hoursAgo: number, kind: HistoryEvent["kind"], id: string, more: Partial<HistoryEvent> = {}): HistoryEvent {
  const known = id === z4cr.mission.id ? { record: "mission" as const, title: z4cr.mission.title } : { record: "item" as const, title: z4crItems.find((i) => i.id === id)?.title ?? id };
  return { commit: `${hoursAgo}`.padEnd(40, "0"), at: ago(hoursAgo * 3600), kind, id, round: null, verdict: null, subject: `${kind} ${id}`, ...known, ...more };
}

/** The records' steps, newest first, for `/api/history`. */
export const events: HistoryEvent[] = [
  event(0.4, "started", "atelier-uwxy"),
  event(2, "merged", "atelier-1ie1"),
  event(3, "review", "atelier-1ie1", { round: 2, verdict: "approved" }),
  event(4, "submitted", "atelier-1ie1"),
  event(5, "review", "atelier-1ie1", { round: 1, verdict: "changes requested" }),
  event(6, "submitted", "atelier-1ie1"),
  event(8, "started", "atelier-1ie1"),
  event(30, "merged", "atelier-z4cr"),
  event(32, "plan_review", "atelier-z4cr", { round: 2, verdict: "approved" }),
  event(40, "planned", "atelier-z4cr"),
  event(24 * 9, "merged", "atelier-old1", { record: null, title: null }),
];

/** `/api/history` with its `days` and `ids` filters. */
export function history(days: string | null, ids: string | null): HistoryResponse {
  const wanted = ids ? new Set(ids.split(",")) : null;
  const since = days ? Date.now() - Number(days) * 86_400_000 : -Infinity;
  return { repo: "atelier", events: events.filter((e) => Date.parse(e.at) >= since && (!wanted || wanted.has(e.id))) };
}
