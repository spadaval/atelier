---
schema: atelier.review
schema_version: 1
id: atelier-defz
title: 'crates/atelier-cli/src/commands/workflow.rs:172-177 — the unknown-transition error no longer names the issue''s current status when transitions exist. Master: ''Unknown transition nope for issue X; available from todo are: block, defer, start, supersede''; HEAD: ''Invalid transition nope. Valid values: block, defer, start, supersede''. The status is kept only in the empty case hint. An agent that expected start but finds the issue in review must run issue show to learn why. Fix: a hint such as ''Transitions from status todo.'' in both branches.'
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:31.575535416Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/workflow.rs:172-177 — the unknown-transition error no longer names the issue's current status when transitions exist. Master: 'Unknown transition nope for issue X; available from todo are: block, defer, start, supersede'; HEAD: 'Invalid transition nope. Valid values: block, defer, start, supersede'. The status is kept only in the empty case hint. An agent that expected start but finds the issue in review must run issue show to learn why. Fix: a hint such as 'Transitions from status todo.' in both branches.
