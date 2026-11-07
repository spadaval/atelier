---
schema: atelier.review
schema_version: 2
id: atelier-qjin
title: 'low: crates/atelier-cli/src/commands/git.rs:483-495 BranchOwner::line prints the bare status word, so a blocked branch owner reads ''task/x  atelier-x blocked  Title'' with no ⚠. human-cli-output.md:12-15 requires ⚠ beside a status that needs attention such as blocked, and the ↳ in-flight mark for the same item (git.rs:566-575, unchanged) still says ''⚠ blocked on task/x'' / ''✓ done on task/x, not merged'' / ''merged into main'' while the row says ''blocked'' / ''done, not merged'' / ''merged''; atelier-ftqn''s Context asks the two views to share branch wording. The test at tests/session.rs:4353-4354 pins the unmarked form. Fix: build the status word with self.cue() (or at least keep ⚠ for attention statuses) and align the merged wording with the in-flight mark, or record a decision exempting Branches rows and document it with R1.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: 57aaf3090f1be8100a33932f28e2c6aea6376cf3
created_at: 2026-10-22T20:43:04.522021310Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: bca612b06974a12e3d5b1a73775266b03c9c0523
    recorded_at: 2026-10-22T20:47:02.157828481Z
    resolution_commit: 1d2f67df28ea9dfd13863c5f41e37302f069dbba
    reason: Rows use cue() for ✓/⚠ and the in-flight mark's merged wording; decision note on atelier-9g48.
    migrated_from: atelier-i6iq
---

low: crates/atelier-cli/src/commands/git.rs:483-495 BranchOwner::line prints the bare status word, so a blocked branch owner reads 'task/x  atelier-x blocked  Title' with no ⚠. human-cli-output.md:12-15 requires ⚠ beside a status that needs attention such as blocked, and the ↳ in-flight mark for the same item (git.rs:566-575, unchanged) still says '⚠ blocked on task/x' / '✓ done on task/x, not merged' / 'merged into main' while the row says 'blocked' / 'done, not merged' / 'merged'; atelier-ftqn's Context asks the two views to share branch wording. The test at tests/session.rs:4353-4354 pins the unmarked form. Fix: build the status word with self.cue() (or at least keep ⚠ for attention statuses) and align the merged wording with the in-flight mark, or record a decision exempting Branches rows and document it with R1.
