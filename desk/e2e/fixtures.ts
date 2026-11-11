// Responses the Playwright suite serves in place of atelier-desk, typed with
// the generated API types so they cannot drift from the server's contract.
// They are modelled on the converted clone's atelier-z4cr, with a review, a
// question and a finished item added so every part of a page shows.
import type { ActRequest } from "../src/api/ActRequest";
import type { ActResponse } from "../src/api/ActResponse";
import type { Agent } from "../src/api/Agent";
import type { DocResponse } from "../src/api/DocResponse";
import type { InboxResponse } from "../src/api/InboxResponse";
import type { ItemPage } from "../src/api/ItemPage";
import type { ItemRow } from "../src/api/ItemRow";
import type { MissionPage } from "../src/api/MissionPage";
import type { MissionRow } from "../src/api/MissionRow";
import type { MissionsResponse } from "../src/api/MissionsResponse";
import type { LiveThread } from "../src/api/LiveThread";
import type { Question } from "../src/api/Question";
import type { ReposResponse } from "../src/api/ReposResponse";
import type { SessionRequest } from "../src/api/SessionRequest";
import type { SessionResponse } from "../src/api/SessionResponse";
import type { StatusCount } from "../src/api/StatusCount";
import type { T3State } from "../src/api/T3State";
import type { ThreadsResponse } from "../src/api/ThreadsResponse";

const commit = "2116defdde63afdadad2dddead0945cb50c21c42";
const at = "2026-11-11T03:50:46+00:00";
/** T3 Code read 4 seconds ago. */
const t3: T3State = { reachable: true, age_seconds: 4, error: null };
/** T3 Code never read: these pages say threads are unavailable. */
export const unreachable: T3State = { reachable: false, age_seconds: null, error: "T3 Code did not answer: connection refused" };
/** atelier-uwxy's second open question, asked after the one about the cache key. */
export const secondQuestion = "Should the cache be dropped instead?";
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
    question: null,
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

function mission(id: string, title: string, status: MissionRow["status"], items: ItemRow[], more: Partial<MissionRow> = {}): MissionRow {
  return {
    id,
    title,
    status,
    labels: [],
    lead: null,
    ready: status === "draft" ? null : { by: "sumeet", at },
    created_at: at,
    in_progress: items.some((i) => ["building", "in_review", "changes_requested", "approved"].includes(i.status)),
    counts: counts(items),
    waiting: items.filter((i) => i.needs_human || i.question).map((i) => i.id),
    ...more,
  };
}

export const z4crItems: ItemRow[] = [
  row("atelier-94pv", "Move the spike's app to desk/ on the new API", "draft", {
    blockers: [{ id: "atelier-uwxy", status: "building" }],
    waiting_on: ["atelier-uwxy"],
  }),
  row("atelier-e05n", "Show the agents working on a mission and its items", "draft", { priority: "medium" }),
  row("atelier-h3hl", "Let Sumeet say go, answer a question and start a planning session from a page", "draft", { priority: "medium" }),
  row("atelier-va99", "Install the Desk as a service on this host, on converted clones", "draft", { priority: "medium" }),
  row("atelier-qxmr", "Point the Desk at the live repositories after this host's cutover", "draft", { priority: "low" }),
  row("atelier-psjw", "Move the acts and repository access into atelier-app", "ready", {
    cannot_start: ["atelier-psjw waits on its mission's go"],
  }),
  row("atelier-aab1", "Find how atelier serve can read T3 thread status", "ready", { labels: ["needs-human"], needs_human: true, startable: true }),
  row("atelier-uwxy", "Serve the Desk's reading routes from commits: the atelier-desk crate", "building", {
    holder: thread,
    branch: { merged: false },
    question: "Should the cache key include the checkout's HEAD?",
    needs: ["atelier-uwxy needs its Report written"],
  }),
  row("atelier-1ie1", "Write the Desk's API as a document first", "done", { branch: { merged: true }, report_written: true, approved_at: at }),
];

export const z4cr: MissionPage = {
  commit,
  mission: mission("atelier-z4cr", "Build the Desk: the human's place to plan missions and read results, on the redesigned records", "ready", z4crItems),
  sections: [
    {
      heading: "Why",
      markdown:
        "Sumeet has no tool of his own. A page that lists records asks him to read records.\n[ADR 0032](../../../docs/adr/0032-the-desk-is-the-humans-product.md) makes the human's tool a product of its own, and [its intent](../../../docs/product/desk.md) says what it is for.",
    },
    { heading: "Outcome", markdown: "On this host, a service reached over the tailnet shows Atelier repositories on the redesigned records." },
    { heading: "Success criteria", markdown: "- desk-matches-cli: every open mission has a page that shows what `atelier show` prints.\n- act-is-a-record: saying go leaves one commit." },
    { heading: "Scope", markdown: "- In: the `atelier-desk` crate and the app.\n- Out: health, the feed and any database." },
    { heading: "Authority", markdown: "The orchestrator may choose the API's routes and fields." },
  ],
  dropped: null,
  can_ready: false,
  documents: [
    { path: ".atelier/missions/atelier-z4cr/design.md", title: "Sizing" },
    { path: ".atelier/missions/atelier-z4cr/notes.md", title: "Notes of atelier-z4cr" },
  ],
  items: z4crItems,
  threads: z4crAgents,
  t3,
  pending_answers: [],
  questions: [
    { item: "atelier-uwxy", question: "Should the cache key include the checkout's HEAD?", finding: "1-cccc.2", answer: null, outcome: null },
    { item: "atelier-uwxy", question: secondQuestion, finding: null, answer: null, outcome: null },
    { item: "atelier-1ie1", question: "Is the history of acts in this mission?", finding: null, answer: { by: "sumeet", text: "No: it waits for M5." }, outcome: "defer" },
  ],
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

const otherMissions: [string, string, MissionRow["status"], ItemRow[], Partial<MissionRow>?][] = [
  ["atelier-295u", "Give the gardener `atelier log` and a readiness checklist", "draft", [row("atelier-k1aa", "Write the log", "draft")]],
  ["atelier-k9rf", "Try a plan that has had no review", "draft", [row("atelier-k9r1", "Write the plan", "draft")]],
  ["atelier-g6rp", "Ship Agent Factory from this repository", "ready", []],
  [
    "atelier-qr5s",
    "Move all five repositories and Agent Factory onto the new loop, one cutover per host",
    "ready",
    [row("atelier-u1ko", "Delete the old workspace", "in_review", { reviewer: "codex-reviewer" }), row("atelier-wd1i", "Run the cutover on this host", "ready")],
  ],
];

export const missionPages: Record<string, MissionPage> = { [z4cr.mission.id]: z4cr };
for (const [id, title, status, items, more] of otherMissions) {
  missionPages[id] = {
    commit,
    mission: mission(id, title, status, items, more),
    sections: [{ heading: "Why", markdown: `${title}.` }],
    dropped: null,
    can_ready: status === "draft",
    documents: [],
    items,
    // atelier-qr5s is read while T3 Code cannot be reached.
    threads: id === "atelier-qr5s" ? [{ identity: codexChild, role: "reviewer", item: "atelier-u1ko", thread: null }] : [],
    t3: id === "atelier-qr5s" ? unreachable : t3,
    questions: [],
    plan_reviews: [],
    pending_answers: [],
  };
}

export const missions: MissionsResponse = { commit, missions: Object.values(missionPages).map((p) => p.mission), loose: 80 };

export const repos: ReposResponse = {
  repos: [
    { name: "atelier", path: "/clones/atelier", base: "master", base_tip: commit, in_progress: 2, drafts: 1, waiting: 1, error: null },
    { name: "broken", path: "/clones/broken", base: null, base_tip: null, in_progress: 0, drafts: 0, waiting: 0, error: "`.atelier/items/x/item.md` does not read: missing field `status`" },
  ],
  built_from: "5e1f0c2d9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d",
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
    can_decide: false,
    decide_at: null,
    pending_answers: [],
    ...more,
  };
}

const done = z4crItems.find((i) => i.id === "atelier-1ie1")!;
const building = z4crItems.find((i) => i.id === "atelier-uwxy")!;

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
  "atelier-uwxy": itemPage(building, {
    read_from: "branch",
    implementers: [thread],
    threads: [
      { identity: thread, role: "implementer", item: null, thread: mark(live.uwxy) },
      { identity: codexChild, role: "reviewer", item: null, thread: null },
    ],
    can_decide: true,
    decide_at: "/clones/atelier-uwxy",
    goal: "Serve the reading routes. See [the API](../../../docs/architecture/desk.md#routes).",
    scope: "`crates/atelier-desk` only.",
    done_when: [
      { ticked: true, text: "`GET /api/repos` answers." },
      { ticked: false, text: "`GET /api/inbox` answers." },
    ],
    decisions: [
      { item: "atelier-uwxy", question: "Should the cache key include the checkout's HEAD?", finding: "1-cccc.2", answer: null, outcome: null },
      { item: "atelier-uwxy", question: secondQuestion, finding: null, answer: null, outcome: null },
    ],
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
      drafts: [missionPages["atelier-295u"].mission],
      questions: [
        { item: "atelier-uwxy", mission: "atelier-z4cr", question: "Should the cache key include the checkout's HEAD?", finding: "1-cccc.2" },
        { item: "atelier-u1ko", mission: "atelier-qr5s", question: "May the old workspace go before the runbook is tried?", finding: null },
      ],
      threads: [live.asking],
    },
    { repo: "broken", commit: null, error: "`.atelier/items/x/item.md` does not read: missing field `status`", needs_human: [], drafts: [], questions: [], threads: [] },
  ],
  t3,
};

export const threads: ThreadsResponse = { t3, threads: [live.uwxy, live.asking, live.lor, live.free] };

/** What the fixture server answers to each act it is sent. */
export const answers = {
  go: {
    result: "done",
    text: "Made ready, as sumeet said: atelier-295u.\nAgents may start its ready items.",
    paths: [".atelier/missions/atelier-295u/mission.md"],
    commit: "76cbb6c0ffee0000000000000000000000000000",
  },
  goRefused: {
    result: "refused",
    rules: ["A mission is made ready only after its plan is reviewed."],
    text: "`atelier ready` refused; atelier-k9rf's plan has no review.",
    next: "Ask for a plan review with `atelier review request atelier-k9rf`, then say go again.",
  },
  answered: {
    result: "done",
    text: "Answered atelier-uwxy's question as sumeet, and deferred finding 1-cccc.2.\nIts implementer's submit commits the answer.",
    paths: [".atelier/items/atelier-uwxy/item.md"],
    commit: null,
  },
  answerRefused: {
    result: "refused",
    rules: [],
    text: "`atelier decide` refused: no checkout has branch atelier-u1ko out.",
    next: "Check out branch atelier-u1ko in a worktree, then answer again.",
  },
  started: { result: "started", thread_id: "thread:plan-z4cr", link: "https://app.t3.codes/threads/env-1/thread%3Aplan-z4cr", project_id: "project-atelier" },
  sessionRefused: { result: "refused", text: "No T3 Code project has /clones/atelier as its workspace root, so no session can start there." },
} satisfies Record<string, ActResponse | SessionResponse>;

/** A request the app sent to an act or session route. */
export type Sent = { path: string; header: string | undefined; body: unknown };

/**
 * The fixture server: every response above, and acts that change them as
 * atelier-desk would. Each test gets its own, so an act never leaks into
 * another test.
 */
export class World {
  repos = structuredClone(repos);
  inbox = structuredClone(inbox);
  threads = structuredClone(threads);
  missionPages = structuredClone(missionPages);
  items = structuredClone(items);
  sent: Sent[] = [];

  missions(): MissionsResponse {
    return { commit, missions: Object.values(this.missionPages).map((p) => p.mission), loose: 80 };
  }

  item(id: string): ItemPage | undefined {
    if (this.items[id]) return this.items[id];
    const known = Object.values(this.missionPages)
      .flatMap((p) => p.items)
      .find((i) => i.id === id);
    return known ? itemPage(known, {}) : undefined;
  }

  /** The response for one API request, as atelier-desk would give it. */
  respond(method: string, url: URL, header: string | undefined, body: string | null): { status: number; body: unknown } {
    const path = url.pathname;
    if (method === "POST") {
      const parsed: unknown = body ? JSON.parse(body) : null;
      this.sent.push({ path, header, body: parsed });
      if (header !== "1") return { status: 403, body: { error: "An act needs the header x-atelier-desk: 1." } };
      if (path === "/api/act") return { status: 200, body: this.act(parsed as ActRequest) };
      if (path === "/api/sessions") return { status: 200, body: this.session(parsed as SessionRequest) };
      return { status: 404, body: { error: "No route" } };
    }
    return this.read(url);
  }

  private act(request: ActRequest): ActResponse {
    if (request.act === "ready") {
      if (request.ids.join() !== "atelier-295u") return answers.goRefused;
      const page = this.missionPages["atelier-295u"];
      page.mission = { ...page.mission, status: "ready", ready: { by: "sumeet", at } };
      page.can_ready = false;
      for (const r of this.inbox.repos) r.drafts = r.drafts.filter((m) => m.id !== "atelier-295u");
      return answers.go;
    }
    if (request.id !== "atelier-uwxy") return answers.answerRefused;
    const item = this.items["atelier-uwxy"];
    // decide answers the oldest question the item's checkout still has open.
    const asked = item.decisions.filter((d) => d.answer === null)[item.pending_answers.length];
    if (!asked) return answers.answerRefused;
    const pending: Question = { ...asked, answer: { by: "sumeet", text: request.answer }, outcome: request.settle };
    item.pending_answers.push(pending);
    this.missionPages["atelier-z4cr"].pending_answers.push(pending);
    this.askNext("atelier-uwxy");
    return answers.answered;
  }

  /** Lists in the inbox the item's oldest question with no answer, committed or pending, as the server does. */
  private askNext(id: string) {
    const item = this.items[id];
    const next = item.decisions.filter((d) => d.answer === null)[item.pending_answers.length];
    for (const r of this.inbox.repos) {
      r.questions = r.questions.filter((q) => q.item !== id);
      if (next) r.questions.push({ item: id, mission: item.mission, question: next.question, finding: next.finding });
    }
  }

  /** The implementer's submit: a commit now holds every pending answer of the item, so none is pending. */
  commitAnswers(id: string) {
    const item = this.items[id];
    const mission = this.missionPages[item.mission!];
    for (const pending of item.pending_answers) {
      const at = item.decisions.findIndex((d) => d.answer === null && d.question === pending.question);
      item.decisions[at] = pending;
      const inMission = mission.questions.findIndex((q) => q.item === id && q.answer === null && q.question === pending.question);
      mission.questions[inMission] = pending;
    }
    item.pending_answers = [];
    mission.pending_answers = mission.pending_answers.filter((p) => p.item !== id);
    this.askNext(id);
  }

  /** Someone answers the item's oldest question elsewhere, as with the CLI, and commits it. */
  answerElsewhere(id: string, text: string) {
    const item = this.items[id];
    const at = item.decisions.findIndex((d) => d.answer === null);
    item.decisions[at] = { ...item.decisions[at], answer: { by: "codex-lead", text } };
    this.askNext(id);
  }

  private session(request: SessionRequest): SessionResponse {
    return request.mission === "atelier-z4cr" ? answers.started : answers.sessionRefused;
  }

  private read(url: URL): { status: number; body: unknown } {
    const path = url.pathname;
    const missing = (what: string) => ({ status: 404, body: { error: `No ${what}` } });
    if (path === "/api/repos") return { status: 200, body: this.repos };
    if (path === "/api/inbox") return { status: 200, body: this.inbox };
    if (path === "/api/threads") return { status: 200, body: this.threads };
    const repo = /^\/api\/repos\/([^/]+)(\/.*)$/.exec(path);
    if (!repo) return missing("route");
    if (repo[1] !== "atelier") return missing(`repository ${repo[1]}`);
    const rest = repo[2];
    if (rest === "/missions") return { status: 200, body: this.missions() };
    if (rest === "/doc") {
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
