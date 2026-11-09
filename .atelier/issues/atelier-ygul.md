---
created_at: "2026-11-09T07:05:20.547021291+00:00"
id: "atelier-ygul"
issue_type: "task"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: atelier-ygul
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-v3nm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Stop land refusing a done validation item whose planning approval predates unrelated code"
updated_at: "2026-11-09T17:52:30.704210575+00:00"
---

## Goal

Marking validation item atelier-v3nm done made `atelier land` refuse on master for every thread: findings.disposed said its three planning fix_now findings came after the current approval, though approval atelier-4102 at dc3a18e4 followed them. The likely cause is that for a done item land requires no code to change between the approval and the done commit (`approval_is_current_to`, crates/atelier-app/src/review_gate.rs), and the mission's own merges changed code after dc3a18e4 while the item waited on them. A validation or planning-only item that changes no code should not need a fresh approval because unrelated code merged first.

## Done when

- [x] A done item or closed mission is not refused by land because another mission's strategy was edited on the base after its planning approval, judged as a new claim or from history; a regression test reproduces the atelier-v3nm case (planning fix_now, approval after it, another mission's replan committed on the base, item marked done there) and the atelier-fl12 case (the same for a mission close). Any other code change on the base after the approval, including one to the item's own mission strategy, still makes it stale.
- [x] The refusal message, where it still applies, names the code change that made the approval stale rather than saying the findings came after the approval.

## Notes

- 2026-11-09 [note] 2026-11-09: Closing mission atelier-fl12 hit the same refusal: findings.disposed said 4 fix_now findings came after the current approval. The approval it meant was Sumeet's planning approval of the mission, atelier-op8x at 2a7a5f14; the findings were the items' own code-review findings (atelier-vjuy, atelier-lfkn, atelier-huh0 and others), each already resolved by the item's later approval. So land also judges a mission's rolled-up item findings against the mission's planning approval. The fix should cover both the done validation item and the closed mission.

## Report

Both Done when lines hold. `base_done_claims_keep_planning_approvals_through_other_missions_strategy_edits` (crates/atelier-cli/tests/session.rs) builds both histories on main: a validation item and a mission, each with a planning fix_now finding, a fix, an approval, then another mission's strategy created and edited on main, then the done claim. Before the fix it failed with "1 fix_now finding came after the current approval"; now land passes, both for the new claim and for a later land that judges it from history (with the old rule restored in `original_approval_valid` alone, that later land fails). The same test edits the item's own mission strategy instead and asserts land still refuses, naming the commit ("replan on the base") and not saying "came after the current approval". `retained_stale_approvals_never_age_into_acceptance` (249ce829's case: code changed directly on main after the approval) still refuses. `cargo nextest run -p atelier-app -p atelier-cli` passes 986 tests, and 996 with `--features test-hooks`. In a scratch clone of this repository, closing atelier-fl12 on master is refused by the installed binary and passes with this branch's build; master checked out at atelier-v3nm's done commit 26186c06 behaves the same. `scripts/smoke-repos.sh` reports no regression across 5 repositories.

### Deviations

Rewrote the first Done when line after diagnosis. The guessed cause (the mission's own merges changing code after the approval) was wrong: merges into master were already excused as clean base merges. In both real cases the commits that made the approval stale were direct commits on master to mission atelier-yfqu's strategy (docs/strategies/fleet-dashboard.md: 2580f29e and 552a7a09 for v3nm, and three more by the fl12 close). Case 2's findings were also not the items' code-review findings but fl12's own planning findings (atelier-2nlf, 42n2, qhve, z246), judged against fl12's approvals atelier-op8x and tdff. "Code elsewhere changed" could not be excused in general without undoing 249ce829, so the line now names the case that is safe to excuse.

### Decisions

- Only strategy files count as excusable, and only those linked solely by missions unrelated to the approval's target (not the target, not holding it in scope, not in its scope). The merge policy allows those commits on the base, and their own mission's planning review covers them. Other direct code commits on the base still make an approval stale, since land has no other check on them yet (atelier-1nlr).
- The allowance applies only to commits the base already holds and only to done claims judged on the base (`base_stale_edge` in review_gate.rs). A work branch's approval must still cover every commit it brings.

### Open items

- The fallback message for a fix_now finding that no approval saw ("came after the current approval") is unchanged.

