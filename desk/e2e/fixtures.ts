// Responses the Playwright suite serves in place of atelier-desk, typed with
// the generated API types so they cannot drift from the server's contract.
// They are modelled on the converted clone's atelier-z4cr, with a review, a
// question and a finished item added so every part of a page shows.
import type { DocResponse } from "../src/api/DocResponse";
import type { InboxResponse } from "../src/api/InboxResponse";
import type { ItemPage } from "../src/api/ItemPage";
import type { ItemRow } from "../src/api/ItemRow";
import type { MissionPage } from "../src/api/MissionPage";
import type { MissionRow } from "../src/api/MissionRow";
import type { MissionsResponse } from "../src/api/MissionsResponse";
import type { ReposResponse } from "../src/api/ReposResponse";
import type { StatusCount } from "../src/api/StatusCount";
import type { T3State } from "../src/api/T3State";

const commit = "2116defdde63afdadad2dddead0945cb50c21c42";
const at = "2026-11-11T03:50:46+00:00";
/** T3 Code not reached: the pages here show no threads. */
const t3: T3State = { reachable: false, age_seconds: null, error: null };
const thread = "thread:delegated-task:command%3Amcp%3Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%3Adelegate-task%3A4e47a5ad";

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
  threads: [],
  t3,
  questions: [
    { item: "atelier-uwxy", question: "Should the cache key include the checkout's HEAD?", finding: "1-cccc.2", answer: null, outcome: null },
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
    threads: [],
    t3,
    questions: [],
    plan_reviews: [],
  };
}

export const missions: MissionsResponse = { commit, missions: Object.values(missionPages).map((p) => p.mission), loose: 80 };

export const repos: ReposResponse = {
  repos: [
    { name: "atelier", path: "/clones/atelier", base: "master", base_tip: commit, in_progress: 2, drafts: 1, waiting: 1, error: null },
    { name: "broken", path: "/clones/broken", base: null, base_tip: null, in_progress: 0, drafts: 0, waiting: 0, error: "`.atelier/items/x/item.md` does not read: missing field `status`" },
  ],
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
    goal: "Serve the reading routes. See [the API](../../../docs/architecture/desk.md#routes).",
    scope: "`next/crates/atelier-desk` only.",
    done_when: [
      { ticked: true, text: "`GET /api/repos` answers." },
      { ticked: false, text: "`GET /api/inbox` answers." },
    ],
    decisions: [{ item: "atelier-uwxy", question: "Should the cache key include the checkout's HEAD?", finding: "1-cccc.2", answer: null, outcome: null }],
  }),
};

export const docs: Record<string, DocResponse> = {
  ".atelier/missions/atelier-z4cr/design.md": {
    commit,
    path: ".atelier/missions/atelier-z4cr/design.md",
    markdown:
      '---\nstatus: draft\nlast_changed: "2026-11-11: amended at Sumeet\'s direction"\n---\n\n# Sizing\n\n- Complexity: high.\n\nThe notes are in [notes](notes.md), and the product intent in [the intent](../../../docs/product/desk.md#why-it-exists). The server is [api.rs](../../../next/crates/atelier-desk/src/api.rs).\n\n# Risks and Unknowns\n\n- The view gap is large.\n',
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
      questions: [{ item: "atelier-uwxy", mission: "atelier-z4cr", question: "Should the cache key include the checkout's HEAD?", finding: "1-cccc.2" }],
      threads: [],
    },
    { repo: "broken", commit: null, error: "`.atelier/items/x/item.md` does not read: missing field `status`", needs_human: [], drafts: [], questions: [], threads: [] },
  ],
  t3,
};

/** The response for one API request, as atelier-desk would give it. */
export function respond(url: URL): { status: number; body: unknown } {
  const path = url.pathname;
  const missing = (what: string) => ({ status: 404, body: { error: `No ${what}` } });
  if (path === "/api/repos") return { status: 200, body: repos };
  if (path === "/api/inbox") return { status: 200, body: inbox };
  const repo = /^\/api\/repos\/([^/]+)(\/.*)$/.exec(path);
  if (!repo) return missing("route");
  if (repo[1] !== "atelier") return missing(`repository ${repo[1]}`);
  const rest = repo[2];
  if (rest === "/missions") return { status: 200, body: missions };
  if (rest === "/doc") {
    const doc = docs[url.searchParams.get("path") ?? ""];
    return doc ? { status: 200, body: doc } : missing("document");
  }
  const mission = /^\/missions\/([^/]+)$/.exec(rest);
  if (mission) return missionPages[mission[1]] ? { status: 200, body: missionPages[mission[1]] } : missing(`mission ${mission[1]}`);
  const item = /^\/items\/([^/]+)$/.exec(rest);
  if (item) {
    if (items[item[1]]) return { status: 200, body: items[item[1]] };
    const known = Object.values(missionPages)
      .flatMap((p) => p.items)
      .find((i) => i.id === item[1]);
    return known ? { status: 200, body: itemPage(known, {}) } : missing(`item ${item[1]}`);
  }
  return missing("route");
}
