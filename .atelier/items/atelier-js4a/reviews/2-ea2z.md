---
schema: atelier.review
schema_version: 3
round: 2
commit: 19b162e01ba1941a8828b600b29a6bcddde73591
reviewers:
- 01a20236-022f-7ff0-bf16-a2b2996ed50d
model: grok-4.6
verdict: approve
created_at: 2026-11-22T07:43:59.435667358Z
---

## Summary

Complexity: medium. A second Git listing and two new log actions sit next to a Desk route, a 15s deadline, and a cache keyed on ref tips; it is still one walk to review.

Value: high. M5 is the mission timeline, and `plan-merge` is what keeps garden.md's `$4 == "merge"` from counting a merged plan as an item.

Risk of merging it: medium. A wrong log line is silent until someone counts it, and it reaches `atelier log` and the Desk, but it is derived data and a revert restores the old walk.

The value is worth that. Round 1 approved at `7f71563da`. Since then this branch gained the round-1 review record, `git merge origin/master` (atelier-1n49: CI job filters and clippy), and the Report. The item's Rust, Desk route, generated types, and log docs are the same tree as round 1. The merge of `docs/design.md` kept this item's `plan` / `plan-merge` table and walk notes, and 1n49's CI-jobs paragraph.

I read `log.rs` (`Plan`, `PlanMerge`, the 24-hour rule, `mission_merges`, `history_until`, `Walker::commit`), the shared helpers in `plan.rs`, `listing_until` in `git.rs`, the Desk route, cache (`read::commits` is HEAD, base tip, and every local and remote-tracking ref), types, and the docs. `GET /api/history` and `/api/launch` have no diff against master; `t3-extension/` has no file change.

`cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk --locked`: 409 passed, including `types` and `history_is_read_from_commit_subjects`. `cargo clippy -p atelier-core -p atelier-cli -p atelier-desk --all-targets --locked -- -D warnings`, `cargo fmt -- --check`, and `git diff --check master...HEAD` passed. On this repository the built CLI gives no `plan` for atelier-g6rp or atelier-qr5s, atelier-osel has plan, plan-merge and an amendment, and 4a94d05c5 is `plan-merge` for atelier-kw8u under `log` and under `show`'s Rules passed. `atelier log atelier-ww7w` prints 46 lines now (the Report's 34, then round 1's 42); it gains a line as the mission's items move.

I did not reproduce the Report's hung `git cat-file` 15.00s 500; `listing_until` and `bounded::within` have unit tests. I did not re-run `desk/` pnpm checks; `desk/` is unchanged since round 1 and the types test passed. I did not run `node t3-extension/run-tests.mjs`; `t3-extension/` is unchanged, and CI's `extension` job passed on `7f71563`.

Both of round 1's deferred findings still hold. I reproduced Finding 1 in a repository that held only `config.toml`, then `start --new`, a Report, and `merge --trivial`: `atelier log` printed the `start` line and no `merge`.

## Finding 1: `atelier log` drops an item's `merge` when the base had no records yet
Recommendation: defer
`Walker::commit` in `crates/atelier-core/src/log.rs` (the `commit.parents.len() > 1` branch) only emits `Happened::Merge` when the walk still sees a merge. The walk lists with `--parents`, so Git rewrites a merge to one parent when the first parent changed no record files. I reproduced the Report's case: a repository with only `config.toml`, `start --new`, a Report, `merge --trivial`; `atelier log` printed the `start` line and no `merge`. `plan-merge` is not this hole: it looks up the full hash in the second `git log`. This is older than this item; the Report already asks the lead to file it.

## Finding 2: a backlog merge past a rule still prints `merge` for every record it changed
Recommendation: defer
`Happened::Passed { merge: true }` is still named `merge` in `Happened::name` (`crates/atelier-core/src/log.rs`). After this change a plan's merge is `plan-merge`, so garden.md's `$4 == "merge"` no longer counts those. A backlog branch merged past a rule still goes through `records_of` and `Passed`, so each record it changed still looks like a merged item. The CLI test `a_proposed_plans_lead_and_merge_and_a_set_of_a_started_item_each_show_their_passed_rule` still asserts that line. The Report names this and suggests a name of its own, such as `records-merge`. Out of this item's Done when.
