---
created_at: "2026-11-09T06:41:35.317557607+00:00"
id: "atelier-uma8"
issue_type: "feature"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: t3/ce3458cf
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ahpf"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "List what waits on Sumeet across every repository, linking into the site's pages"
updated_at: "2026-11-09T21:03:12.366224563+00:00"
---

## Goal

Give Sumeet one queue of what waits on him across every repository on this host, linking into the brief, debrief, board and item pages, so a notification takes him straight to the page he must read. The inbox is not the home page.

## Done when

- [x] `GET /api/inbox` and the `/inbox` page list, grouped by repository, exactly the open items `atelier status` on each base branch lists under Needs a human, and each draft mission with a link to its brief. A Rust fixture test shows the set matches `status` with two repositories, and a Vitest test renders it from the fixture JSON.
- [x] each entry links to its item page, and, under the rule the Unknowns settle, to the page of the mission it asks the human to read, which opens the brief or debrief by the mission's status. A fixture test covers a decision, a loose end that names a mission, a needs-human item related only to a superseded mission, and a draft mission.
- [x] `/` and every board link to `/inbox` and show its count.
- [x] Served from this host's repositories, every inbox link opens a page that renders in Playwright without an error; the Report records the command and counts.

## Scope

Read-only. No new link role or record field to choose an entry's target.

## Context

Planned at the evidence boundary after atelier-zpmb and atelier-ahpf: they show how status-like views cost when read from refs, and the item page this item links to. Sumeet's 2026-11-08 direction on atelier-yfqu: "the inbox is a notification queue linking into those pages, not the home page."

## Unknowns

- Which missions an entry links besides its item page. Any linked mission is too loose: needs-human atelier-3km4 asks for atelier-vt8q's debrief but relates only to superseded mission atelier-gg6b (finding atelier-ei8w). Settle a rule, or link only the item page, and say which in the Report.
- Whether listing every draft mission is noisy; if it is, show only drafts with a recorded planning approval, and say so in the Report.

## Report

All four Done when lines hold. `GET /api/inbox` and `/inbox` list, by repository, the work `atelier status` counts under Needs a human (the same `status_view`) and the draft missions planning review approved: `serve::inbox::the_inbox_lists_what_status_counts_as_needing_a_human_and_approved_drafts` checks each fixture repository's items against `atelier status -q`'s `human=` and `atelier work human -q`, and passes. The same test asserts each entry's mission links by hand: decision alpha-a2 links draft alpha-m1 on its brief, loose end alpha-l2 (blocked_by done alpha-m2) links alpha-m2 on its debrief, validation beta-v1 related only to superseded beta-m3 links nothing, and approved draft alpha-m1 is listed while unapproved draft beta-m1 is not. `inbox::tests::an_item_links_its_scope_and_direct_missions_but_no_retired_one` fails when the blocker-graph links are removed and passes with them. Vitest (`src/pages/inbox.test.tsx`) renders the page from the fixture JSON and checks `Inbox (4)` on `/`, `/alpha` and `/beta`; Playwright on the fixture fleet loads `/inbox` and clicks from a board through the inbox to a debrief (36 passed). On this host, a release build of this branch served `~/projects` on port 7351 and `ATELIER_SERVE=http://127.0.0.1:7351 pnpm exec playwright test -c playwright.host.config.ts` loaded every inbox link: 5 repositories, 50 items, 5 draft missions, 61 pages, 0 failed (and 84 brief and debrief pages, 0 failed). `atelier status -q` matched the inbox in the checkouts on their base: astromech 35, cosplay-studio 2, worldcrafter 2.

### Decisions

- Which missions an entry links (notes of 2026-11-09, the second revising the first after finding atelier-jx4p): every mission whose scope holds the item, plus every mission it links under any role, typed relations and blockers alike, leaving out superseded and declined missions. atelier-3km4 therefore links only its item page, since its one mission, atelier-gg6b, is superseded.
- Drafts: listing every draft is noisy (6 of the host's 13 drafts, all in cosplay-studio, have no review at all), so only drafts an approval targets are listed.

### Surprises

- The needs-human set `atelier status` counts is not the board's: the board and `atelier work human` apply ready gates, status does not. The inbox follows status, as Done when says; in the fixtures and on this host the two agree.
- `blocked_by` links are kept only in the record index's blocker graph, not among typed relations (atelier-jx4p).

### Open items

- `scripts/smoke-repos.sh` was not run: it runs read-only CLI commands, and this change adds only a route and a page to `atelier serve`.
- The live `atelier-serve` service still runs the installed binary; it shows the inbox after the merge, a `cargo install` and a restart, which atelier-mtb3 validates.

## Notes

- 2026-11-09 [decision] An inbox entry links its item page and every mission the item belongs to: each mission whose scope holds it (the advances scope atelier-app's mission_overview computes, the same scope a brief's Asks come from) and each mission its record links under any role (get_typed_relations, either direction), leaving out missions whose status is superseded or declined. Each mission link opens the mission page, which shows its brief or debrief by the API's review. With this rule atelier-3km4 links only its item page, since its one mission, atelier-gg6b, is superseded.
  Reason: Scope plus direct links reaches every mission a needs-human item, a decision or a loose end (never in scope, so only by a direct link such as blocked_by) is about, and needs no new link role or record field. The ei8w problem was a retired mission, not the role, so dropping superseded and declined missions fixes it; a done mission stays, because asking for a mission's debrief after it closes is the common case. mission_review already treats superseded by name, so the status names follow precedent.
  Rejected: Link only the item page: loses the one-tap path to the brief or debrief the entry asks for.
  Rejected: Any linked mission under any role: opens superseded atelier-gg6b for atelier-3km4 (finding atelier-ei8w).
  Rejected: Parse mission IDs from the title or body: guesswork the CLI does nowhere.
- 2026-11-09 [decision] The inbox lists a draft mission (a mission in its workflow's initial status, so its page opens on the brief) only when at least one approval review event targets it, the record a planning review leaves.
  Reason: On this host 13 draft missions exist across 6 repositories and 6 of them, all in cosplay-studio, have no review at all: old exploration drafts nobody is planning, which would sit in the queue forever. Each of the 7 with an approval has been through planning review and waits on Sumeet's brief approval. The approval is already a record, so this adds no field.
  Rejected: Every draft mission: 6 of 13 entries are stale drafts not waiting on Sumeet.
  Rejected: Only drafts with two planning approvals: plans differ in how many passes they record, and one approval already shows planning has reached review.
- 2026-11-09 [decision] Expected size: about 900 hand-written lines across atelier-app (inbox view, ~150), atelier-site (route, types, ~200), fixtures and Rust tests (~250), the React page, nav count and Vitest test (~250), Playwright and docs (~100).
  Reason: The brief's estimate is 600 to 1,200; this is its middle.
- 2026-11-09 [plan] Design. (1) atelier-app gains inbox.rs: inbox(index, policy) -> Inbox { items: Vec<InboxItem { row: BoardRow, missions: Vec<String> }>, drafts: Vec<String> }. items is status_view(index, policy).work.human, the exact set and order atelier status counts as Needs a human (no ready gates, unlike the board's human list), with mission IDs by the linking decision; drafts are missions in their workflow's initial status with an approval event, in mission_overview order. (2) atelier-site computes it per base commit inside Loaded, beside the counts, so it is cached with the records and costs no extra build. GET /api/inbox returns InboxResponse { count, repos: Vec<InboxRepo { repo, base_branch, base_commit, error, items: Vec<InboxEntry { item: BoardRow, missions: Vec<MissionSummary> }>, drafts: Vec<MissionSummary> }> }, listing repositories that have an entry or an error, in fleet order; count is items plus drafts over all repositories. MissionSummary already carries review and links. (3) App: /inbox page grouped by repository, each entry linking /<repo>/item/<id> and /<repo>/mission/<id>; a shared Inbox (n) link in the Page header that / and the board show, from /api/inbox. (4) Tests: fixture records added to alpha and beta (a needs-human loose end blocked_by done mission alpha-m2, an approval on draft alpha-m1, a needs-human validation related only to superseded beta-m3, draft beta-m1 left unapproved), a Rust test comparing each repository's inbox items with atelier status -q's human count and atelier work human -q, fixture JSON for /api/inbox, a Vitest render, the e2e route list, and a host Playwright spec that loads every inbox link. (5) docs/architecture/site.md gains the route, page and rules.
- 2026-11-09 [decision] Revises the first linking decision (finding atelier-jx4p): an entry's direct mission links are the missions at the other end of any of its typed relations (get_typed_relations) or of its blocker graph, either way (get_blockers and get_blocking, which hold blocked_by and blocks links that get_typed_relations leaves out). Together with the scope rule, deduplicated, with superseded and declined missions left out, as before. get_typed_relations keeps its meaning. The Rust fixture test asserts the expected mission IDs and review of each entry by hand: the loose end blocked_by done mission alpha-m2 links alpha-m2 with review debrief.
  Reason: A loose end names what it waits on with blocked_by, which the record index keeps only in its blocker graph; without it the loose-end case in Done when would get no mission link.
