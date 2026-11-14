---
schema: atelier.review
schema_version: 3
round: 1
commit: 2d0bed695a59ea4a1f93f60656bc1e087f88dbb7
reviewers:
- thread:delegated-task:command%3Amcp%3A524904cd-3475-4a13-9587-b5a6781d23a6%3Adelegate-task%3Ac07677b0-8b4e-48c8-ac7c-7b018cfaf200
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T06:14:43.000180999Z
---

## Summary

Complexity: low. The release change edits the workspace version, lockfile,
changelog and one cutover instruction; it introduces no runtime code.
Value: high. The published binary needs to include the workflow already in
use on this host.
Risk of merging: medium. Incorrect upgrade guidance reaches every installer,
although the release metadata and notes are cheap to correct.
The value warrants this scope; no smaller change supplies the release.

Reviewed commit 2d0bed695a59ea4a1f93f60656bc1e087f88dbb7 in a fresh detached
worktree at origin/atelier-6hb2. Compared its release changes with master and
its changelog claims with v0.5.0: command definitions, mission compatibility,
branch naming and discovery, record writes, merge gates and hooks, plus the
corresponding integration scenarios. The listed features are implemented.
0.6.0 is an appropriate pre-1.0 minor version for the removed commands and
changed workflow. The four versioned tracker packages agree at 0.6.0;
atelier-desk intentionally remains an unreleased 0.0.0 package. The candidate
binary prints `atelier 0.6.0`. No stale current-version declaration was found.

Validation: locked Cargo metadata, formatting and release-diff whitespace
checks pass. `cargo nextest run -p atelier-cli --test branches --test planning
--test disputes --test log --test hooks --test merge --test views` passed all
159 selected tests; one performance test is skipped by default. All tests
used TMPDIR=/home/supadava/.cache/atelier-review-6hb2. A separate scratch
repository reproduced the old-draft upgrade behavior below. Protected-base
merging was covered by the existing forge-stub tests; I did not perform a
live merge or build release archives. `atelier review` refused the detached
checkout and directed review-record writes to the item's branch checkout,
which is where this review and verdict are recorded.

## Finding 1: Explain how to continue an existing 0.5 draft mission
Recommendation: fix now

CHANGELOG.md:11-12 says there is no migration, but the entry never explains
what to do with a 0.5 mission already stored as `status: draft` on the base.
These records parse, yet their old continuation command, `ready <mission>
--by <name>`, is now refused. They remain proposed even on the base, so they
cannot be led or have their items started. The new `start --mission` example
creates a different mission and does not resume the existing ID.

This is implemented in `atelier-core/src/mission.rs` and
`atelier-app/src/plan.rs::agreed`, and exercised by
`an_old_draft_on_the_base_is_agreed_only_by_two_plan_reviews` in
`crates/atelier-cli/tests/planning.rs:2362`. I also reproduced it with a
0.5-shaped draft lacking `planners`: `check` passed, `ready` and `lead`
refused, then creating its named branch and running `submit` changed it to
`open` and requested plan reviews.

Keep the format-compatibility assurance, but add an explicit upgrade note:
old ready missions are already agreed; an existing draft must be continued
on a branch named for that mission's ID, submitted there, and merged after
two current plan approvals. State that `ready` now accepts items only. This
lets an upgrading user resume the existing plan instead of assuming that
no action is required or creating a replacement mission.
