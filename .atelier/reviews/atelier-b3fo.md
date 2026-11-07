---
schema: atelier.review
schema_version: 2
id: atelier-b3fo
title: 'low: crates/atelier-cli/src/commands/record_commit.rs:66-69 A hand edit that is staged (git add) onto a record of a pending --stage-records group is still committed with the group, since the group is tracked by path and the index cannot tell Atelier''s content from the hand edit (reproduced in a scratch repo). It is the same record file, nothing unrelated is swept in and nothing is lost, so this is within the intent; recording a blob hash per path would close it if wanted. Also noted: staged records follow a branch switch (git carries the index) and the group then commits on the new branch; harmless.'
target: atelier-zuow
identity: Independent reviewer (Claude subagent)
commit: 7cc73063f699089b7c61a7f547dc92a51e6694e7
created_at: 2026-10-22T21:51:27.963986341Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: ce2ae87512c10f2e3cc0f42ea3bc001eacde961c
    recorded_at: 2026-10-22T21:51:45.271003349Z
    reason: Same record file, nothing unrelated swept in and nothing lost.
    migrated_from: atelier-6bid
---

low: crates/atelier-cli/src/commands/record_commit.rs:66-69 A hand edit that is staged (git add) onto a record of a pending --stage-records group is still committed with the group, since the group is tracked by path and the index cannot tell Atelier's content from the hand edit (reproduced in a scratch repo). It is the same record file, nothing unrelated is swept in and nothing is lost, so this is within the intent; recording a blob hash per path would close it if wanted. Also noted: staged records follow a branch switch (git carries the index) and the group then commits on the new branch; harmless.
