---
schema: atelier.review
schema_version: 3
round: 1
commit: 7f71563dadf29e4bec55f2dd8ca9ec736a53c775
reviewers:
- 01a20224-aea4-7341-bc95-2d4c778eee98
model: grok-4.6
verdict: approve
created_at: 2026-11-22T07:25:01.603406105Z
---

## Summary

Complexity: medium. A second Git listing and two new log actions sit next to a Desk route, a 15s deadline, and a cache keyed on ref tips; it is still one walk to review.

Value: high. M5 is the mission timeline, and `plan-merge` is what keeps garden.md's `$4 == "merge"` from counting a merged plan as an item.

Risk of merging it: medium. A wrong log line is silent until someone counts it, and it reaches `atelier log` and the Desk, but it is derived data and a revert restores the old walk.

The value is worth that. I read the three commits against merge-base `40ffeea81`, the Done when lines, and the Report. I checked `log.rs` (`Plan`, `PlanMerge`, the 24-hour rule, `mission_merges`, `history_until`), the shared helpers in `plan.rs`, `listing_until` in `git.rs`, the Desk route, cache, types, and the docs. `GET /api/history` and `/api/launch` are untouched; `t3-extension/` has no file change.

`cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk --locked`: 409 passed, including `types` and `history_is_read_from_commit_subjects`. `cargo fmt -- --check` and `git diff --check` passed. Desk: `tsc --noEmit`, vitest 57, Playwright 134. On this repository the built CLI gives no `plan` for atelier-g6rp or atelier-qr5s, atelier-osel has plan, plan-merge and an amendment, and 4a94d05c5 is `plan-merge` for atelier-kw8u, not `merge`.

I did not run `clippy --all-targets`. I did not reproduce the Report's hung `git cat-file` 15.00s 500; `listing_until` and `bounded::within` have unit tests. I did not match the Report's "34 actions" for atelier-ww7w: this worktree's `atelier log atelier-ww7w` prints 42 lines, three of them atelier-js4a. `node t3-extension/run-tests.mjs` had 6 of 7 pass; `test-extension.mjs` failed twice here with a JSON parse of a Node warning on the Chrome control channel after `custom.js` said the composer was not filled. That test is in frozen `t3-extension/`. CI's extension job still gates merge.

## Finding 1: `atelier log` drops an item's `merge` when the base had no records yet
Recommendation: defer
Dropped by mcp:launch:b3b5ffd62a4c227e4ec8ce23f1e32b83538dd28e5e6c91e0a2d3d91ead863ef6: filed as atelier-tuln
`Walker::commit` in [crates/atelier-core/src/log.rs](/home/supadava/.t3/worktrees/atelier/t3-c624a94b-528e329e838e8bfc/crates/atelier-core/src/log.rs) only emits `Happened::Merge` when `commit.parents.len() > 1`. The walk lists with `--parents`, so Git rewrites a merge to one parent when the first parent changed no record files. I reproduced the Report's case: a repository with only `config.toml`, `start --new`, a Report, `merge --trivial`; `atelier log` printed the `start` line and no `merge`. `plan-merge` is not this hole: it looks up the full hash in the second `git log`. This is older than this item; the Report already asks the lead to file it.

## Finding 2: a backlog merge past a rule still prints `merge` for every record it changed
Recommendation: defer
Dropped by mcp:launch:b3b5ffd62a4c227e4ec8ce23f1e32b83538dd28e5e6c91e0a2d3d91ead863ef6: filed as atelier-itqc
`Happened::Passed { merge: true }` is still named `merge` in `Happened::name`. After this change a plan's merge is `plan-merge`, so garden.md's `$4 == "merge"` no longer counts those. A backlog branch merged past a rule still goes through `records_of` and `Passed`, so each record it changed still looks like a merged item. The Report names this and suggests a name of its own, such as `records-merge`. Out of this item's Done when.
