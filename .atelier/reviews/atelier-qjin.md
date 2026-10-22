---
schema: atelier.review
schema_version: 1
id: atelier-qjin
title: 'low: crates/atelier-cli/src/commands/git.rs:483-495 BranchOwner::line prints the bare status word, so a blocked branch owner reads ''task/x  atelier-x blocked  Title'' with no ⚠. human-cli-output.md:12-15 requires ⚠ beside a status that needs attention such as blocked, and the ↳ in-flight mark for the same item (git.rs:566-575, unchanged) still says ''⚠ blocked on task/x'' / ''✓ done on task/x, not merged'' / ''merged into main'' while the row says ''blocked'' / ''done, not merged'' / ''merged''; atelier-ftqn''s Context asks the two views to share branch wording. The test at tests/session.rs:4353-4354 pins the unmarked form. Fix: build the status word with self.cue() (or at least keep ⚠ for attention statuses) and align the merged wording with the in-flight mark, or record a decision exempting Branches rows and document it with R1.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: 57aaf3090f1be8100a33932f28e2c6aea6376cf3
created_at: 2026-10-22T20:43:04.522021310Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/src/commands/git.rs:483-495 BranchOwner::line prints the bare status word, so a blocked branch owner reads 'task/x  atelier-x blocked  Title' with no ⚠. human-cli-output.md:12-15 requires ⚠ beside a status that needs attention such as blocked, and the ↳ in-flight mark for the same item (git.rs:566-575, unchanged) still says '⚠ blocked on task/x' / '✓ done on task/x, not merged' / 'merged into main' while the row says 'blocked' / 'done, not merged' / 'merged'; atelier-ftqn's Context asks the two views to share branch wording. The test at tests/session.rs:4353-4354 pins the unmarked form. Fix: build the status word with self.cue() (or at least keep ⚠ for attention statuses) and align the merged wording with the in-flight mark, or record a decision exempting Branches rows and document it with R1.
