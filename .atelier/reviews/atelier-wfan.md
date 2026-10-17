---
schema: atelier.review
schema_version: 1
id: atelier-wfan
title: 'crates/atelier-cli/src/commands/work.rs:566-575 — ScopedWork::sections still appends '' | blockers: <ids>'' to the title field and also sets blockers, so work mission/work parent rows read ''... in status | blockers: atelier-ftqn | 1 blocker'' (seen: work mission atelier-k8h8 --all, atelier-tciw): the same fact twice, and a cue stored in the title, which breaks the doc''s fixed-field model (human-cli-output.md Item Line; atelier-e95f constraint), since a declarative view could not choose or drop that cue. Pre-existing on master; atelier-tf9n replaces the count with the direct blockers, so fold it in there by making the id list an ItemLine cue (IssueListRow needs a cue field).'
target: atelier-i2qq
identity: Independent reviewer (Claude subagent)
commit: c4b06e91043b5f0b07fce4a2cb9a0ba9e6cd2945
created_at: 2026-10-17T06:18:09.781092499Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/work.rs:566-575 — ScopedWork::sections still appends ' | blockers: <ids>' to the title field and also sets blockers, so work mission/work parent rows read '... in status | blockers: atelier-ftqn | 1 blocker' (seen: work mission atelier-k8h8 --all, atelier-tciw): the same fact twice, and a cue stored in the title, which breaks the doc's fixed-field model (human-cli-output.md Item Line; atelier-e95f constraint), since a declarative view could not choose or drop that cue. Pre-existing on master; atelier-tf9n replaces the count with the direct blockers, so fold it in there by making the id list an ItemLine cue (IssueListRow needs a cue field).
