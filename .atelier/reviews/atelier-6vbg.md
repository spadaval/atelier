---
schema: atelier.review
schema_version: 2
id: atelier-6vbg
title: finding on atelier-h1nc @b29d7717
target: atelier-h1nc
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: b29d77173838f0e67a446fd90643a9d7ff6666a8
created_at: 2026-10-23T15:08:08.956890637Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: no_action
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 21174abf904abac519eece4eb62664a228f85068
    recorded_at: 2026-10-23T15:08:37.623644576Z
    reason: An open blocker is separate work; starting it is the honest next step and no one-step command can make it done. The line labels it as such.
    migrated_from: atelier-godr
---

low: crates/atelier-cli/src/gate_report.rs blocker_fix: for an open todo blocker the fix 'atelier start <blocker>' switches the checkout to a new task/<blocker> branch based on the owner branch and leaves the blocker in_progress, still open, so blockers.none_open stays failed; for a backlog blocker the fix is only the labelled transition listing. Honest labelling, but neither moves the check to passing without further steps. Consider printing the full path (plan/start/close) or only the unlink.
