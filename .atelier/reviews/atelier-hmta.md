---
schema: atelier.review
schema_version: 1
id: atelier-hmta
title: 'crates/atelier-cli/src/commands/workflow.rs:71: with neither flag, effective_close_reason reuses whatever Close Reason the record holds, and apply_transition never clears Close Reason when a record leaves done. A record reopened by editing its status (or by a policy transition out of done) and closed again with no flag now silently reuses the old Outcome and report, where before the close was refused for a missing --reason. Similarly --reason on supersede keeps report parts written for a close. Impact is a stale report reaching debrief (S4). Fix later: clear Close Reason when a transition leaves a done status, or have the transition print which Close Reason it used.'
target: atelier-sfsq
identity: Independent reviewer (Claude subagent)
commit: 184469851584ee81ba2b163fb54071bd9ccabd7b
created_at: 2026-10-17T18:04:30.467604974Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/workflow.rs:71: with neither flag, effective_close_reason reuses whatever Close Reason the record holds, and apply_transition never clears Close Reason when a record leaves done. A record reopened by editing its status (or by a policy transition out of done) and closed again with no flag now silently reuses the old Outcome and report, where before the close was refused for a missing --reason. Similarly --reason on supersede keeps report parts written for a close. Impact is a stale report reaching debrief (S4). Fix later: clear Close Reason when a transition leaves a done status, or have the transition print which Close Reason it used.
