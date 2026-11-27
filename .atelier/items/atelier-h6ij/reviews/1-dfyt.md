---
schema: atelier.review
schema_version: 3
round: 1
commit: ba2287063e6af8c5e339c78ec9734ff309996986
reviewers:
- 01a2022c-0534-7bc2-a59f-b28beca3e569
model: grok-4.6
verdict: approve
created_at: 2026-11-22T07:33:00.212035197Z
---

## Summary

The change does what the item asks: later plan rounds open on the plan's own diff (or say the plan is unchanged), two trailing split rounds on one plan change stop review for a human ruling, a code item at changes-requested in round 3 or later points at the lead, and a code delta is omitted or scoped when the last round's commit cannot be read as a clean range.

Complexity: medium. The new moving parts are `plan_delta` / `plan_paths`, trailing split counting on the change's own review files, `git::changed_on_branch`, and the status/submit/merge wording; each is local and the tests name the cases.

Value: high. Split plan loops and full re-reviews on round 2+ are in current use on mission and amendment branches.

Risk of merging it: medium. A wrong path list could hide a plan edit from a later round, and a wrong split count could stop a merge that should go or allow another loop. Both show in `atelier review`, `submit`, `status`, and `merge`, and the tests cover the specified cases, including a taken-back amendment edit and an amendment of a plan that already merged after two splits.

Checked by reading the diff at `ba2287063e6af8c5e339c78ec9734ff309996986` against the Done when lines, the Report, and `roles/reviewer.md` (unchanged). Walked `plan_delta`, `plan_paths`, `trailing_splits` / `Approvals::stopped`, `review::previous`, `say::needed` / `cap_for_lead`, `plan_unapproved`, `plan_submitted`, and the role and product docs. Ran `cargo nextest run --locked -j 2 -p atelier-cli --test rounds` (12 passed), `plan::tests::only_completed_split_rounds_at_the_end_count_toward_the_exit`, the later-round and round-3 status tests in `scenarios.rs`, and `evals/agent-factory/test.sh` (all fixtures passed).

Could not check: a real reviewer agent working from the new briefs (the Report also leaves that undriven); the full `cargo nextest run --locked` suite of 416 tests; `cargo fmt` / `clippy` as claimed in the Report; reproducing `validation/real-history.txt` in a fresh clone.
