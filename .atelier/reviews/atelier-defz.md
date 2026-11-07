---
schema: atelier.review
schema_version: 2
id: atelier-defz
title: 'crates/atelier-cli/src/commands/workflow.rs:172-177 — the unknown-transition error no longer names the issue''s current status when transitions exist. Master: ''Unknown transition nope for issue X; available from todo are: block, defer, start, supersede''; HEAD: ''Invalid transition nope. Valid values: block, defer, start, supersede''. The status is kept only in the empty case hint. An agent that expected start but finds the issue in review must run issue show to learn why. Fix: a hint such as ''Transitions from status todo.'' in both branches.'
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:31.575535416Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 5f00943ce60e1c6259d43098c548683140d5b06a
    recorded_at: 2026-10-17T09:55:35.650634784Z
    resolution_commit: efee9e953d9329f07a6c463676efc8b47c0b7b79
    reason: 'Fixed now: the unknown-transition error names the current status.'
    migrated_from: atelier-r41r
---

crates/atelier-cli/src/commands/workflow.rs:172-177 — the unknown-transition error no longer names the issue's current status when transitions exist. Master: 'Unknown transition nope for issue X; available from todo are: block, defer, start, supersede'; HEAD: 'Invalid transition nope. Valid values: block, defer, start, supersede'. The status is kept only in the empty case hint. An agent that expected start but finds the issue in review must run issue show to learn why. Fix: a hint such as 'Transitions from status todo.' in both branches.
