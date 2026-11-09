---
created_at: "2026-11-09T00:45:58.429754395+00:00"
id: "atelier-ahpf"
issue_type: "feature"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: t3/fleet-board-item-pages
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
  - kind: "issue"
    id: "atelier-yn69"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T20:55:40.018911202+00:00"
status: "done"
title: "Answer where are we, what's done and what's next: a board per repository and a page per item"
updated_at: "2026-11-09T20:55:40.018911202+00:00"
---

## Goal

Answer "where are we, what's done and what's next?" for each repository without an agent: a board per repository in the site's app, with a page per item, read from the base branch through the JSON API.

## Done when

- [x] `GET /api/<repo>/board` and `/api/<repo>/item/<id>` return the `atelier-app` views the board and item pages show, with generated types and fixture JSON as atelier-zpmb set up. `/<repo>` is the board: each open mission with its progress (items done of planned, criteria proven) linking its mission page; all open work grouped by mission, with work in no mission grouped by label, each with its status; work in progress with its branch, review state (unreviewed, approval current or stale, findings open) and how far it is behind; ready work in the order `atelier work ready` gives; and items done in the last 14 days. A Rust fixture test covers each section's data, and a Vitest test renders each section from the fixture JSON.
- [x] `/<repo>/item/<id>` renders the item's body (Markdown), status, links, notes, reviews and findings, and a decision shows its Question and Options. A fixture test covers a task, a decision and a loose end.
- [x] with a warm cache and the base unchanged, moving an owned work branch from unreviewed to approved, then to approval stale by a new code commit, changes the board API's review state on the next request to match `atelier status` each time; with an injected clock, an item leaves the recently-done section once it is more than 14 days old.
- [x] `/` shows each repository's counts and links its board; the board links each mission and item; brief, debrief and item pages link back to the board.
- [x] Load: on this host, served by a release build, a board request for this repository takes under 3 s cold and under 500 ms when neither the base nor any work branch has moved, within atelier-zpmb's two-build cap. The Report records the numbers.
- [x] The Playwright suite loads the board and an item page of each kind at 390 px. Served from this repository's `master`, `/atelier` lists the same in-progress, ready and needs-human items as `atelier status` run on `master`; the Report records the comparison.

## Context

Planned at the evidence boundary after atelier-zpmb and atelier-yn69, which show whether views read from a base commit and other refs are cheap enough and how the shared views must change. `atelier status` costs 1.6 s here today, most of it reading each work branch's tracker (atelier-3i7x). Reuse the app layer's views so fixes from mission atelier-4pzj (such as atelier-om6l) reach the board too. Sumeet's 2026-11-08 direction on atelier-yfqu: "a backlog board per repository (missions with progress, all open work by mission, label and status, recently done) answering where are we, what's done, what's next."

## Unknowns

- What a board request costs. Branch and review state come from each owned work branch's records at a pinned commit of that branch, read afresh per request as atelier-zpmb does for refs; `atelier status` takes 1.6 s here today, most of it reading each work branch's tracker (atelier-3i7x). If a cold board request takes over 3 s, cache what depends only on the base commit and each branch's commit, keyed by those commits.

## Report

Every Done when line holds. `/api/<repo>/board` and `/api/<repo>/item/<id>` serve the new `atelier_app::board` and `atelier_app::item_view` views, with types in `web/src/api/` and fixture JSON for both boards and four items. Each board section is covered by `serve::board::the_board_shows_each_section_from_the_base_and_its_work_branch` and rendered by the Vitest "the board" tests. Item pages for a task, a decision (Question and Options) and a loose end (with Notes) are covered by `item_pages_show_a_task_a_decision_and_a_loose_end` and the Vitest "an item page" tests. `review_state_follows_the_work_branch_on_the_next_request_as_status_says` takes a branch from unreviewed to approved to approval stale against a warm cache, and `atelier status` prints the matching words at each step. `recently_done_work_leaves_the_board_after_14_days` moves an injected clock. Links to and from the board are checked by Vitest and by a Playwright walk. All 31 Playwright tests pass at 390 px, including both boards and four item pages.

Load, release build of 671d348e serving all of `~/projects` on this host with the default two-build cap: the first `/api/atelier/board` on a fresh server took 2.20 s, and the next three 0.013, 0.012 and 0.012 s. An item took 0.011 s. In the fixture, `a_warm_board_builds_nothing_until_a_branch_moves` shows a warm board runs no build, and moving one branch runs exactly one.

Comparison on `master` at 567b3e83: in a throwaway detached worktree, `atelier status`, `atelier work ready -q` and `atelier work human -q` gave 14 ready items, the same IDs in the same order as the board's ready list; 3 needs-human items, the same as the board's; and no current work, as the board's active list. The board's in-progress rows also match status's branch rows: atelier-ahpf on t3/fleet-board-item-pages with the approval stale, and atelier-59fz on t3/remove-cli-html-briefs approved.

### Decisions

- Work in progress is every owner of an unmerged work branch, found and judged as `atelier status` finds and judges its branch rows, followed by any active work on the base that no branch owns. Branch work is read with `commit_records::load` at the tip, not a checkout. Light-path branches (no owner) are not listed.
- `unreviewed` means no review event targets the owner's scope; `atelier status` prints `⚠ not approved` for both it and `not_approved`. Open findings are those `findings.disposed` would not count as settled.
- To share one implementation, the work order (`work_order`), the ready gates (`work_classification::apply_ready_gates`), `branch_owner_ids` and `branch_merge` (`branch_work`) moved from the CLI into `atelier-app`. The CLI calls them unchanged.
- An item page shows the record body as committed, so Notes, Report and a decision's Question and Options render as its own sections rather than as separate fields.
- Recently done is counted in whole UTC days: work closed on today's date or within the 14 days before it.

### Surprises

- `atelier status` run in a fixture checkout writes `.atelier/runtime/.canonical-mutation.lock`, and the fixture's `git add -A` committed it as code, which made an approval stale. The test excludes `.atelier/runtime/` the way a real checkout does.
- The alpha fixture's new base commit raised `divergent_head`'s count of code commits since the receipt from 1 to 2. The test now expects 2.
