---
schema: atelier.review
schema_version: 2
id: atelier-0sem
title: finding on atelier-8bh4 @70b45d46
target: atelier-8bh4
identity: independent reviewer subagent (claude-opus-5-5), atelier-8bh4
commit: 70b45d464142a56eba268b9270dc764ea7e13de8
created_at: 2026-11-07T09:40:05.702825885Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 83e9a3a91f34fb82d9281a01aba87137f9d86089
    recorded_at: 2026-11-07T09:46:06.953430894Z
    resolution_commit: 83e9a3a91f34fb82d9281a01aba87137f9d86089
    reason: Fixed in 83e9a3a9.
---

low: crates/atelier-cli/src/commands/workflow.rs:178 the start_done_on_base / merge_done_on_base refusal says an item 'is done on <base> by its own transition once what it waits on is merged'. Judging by workflow puts every decision in all five repos under it, and a decision waits on a recorded Decision, not a merge; the old workflow_branch_decision told the agent to record the Decision first (seen on astromech-sd0k: old vs new start). Fix: drop or generalise 'once what it waits on is merged' (for example, 'by its <transition> transition on <base>'), so the message is true for decisions too.
