---
schema: atelier.review
schema_version: 2
id: atelier-ed7b
title: 'crates/atelier-cli/src/commands/issue.rs:878: --strategy can set or change a path but never clear it, and workflow validation rejects a strategy on any non-mission (''strategy is only valid on issue_type mission''). So `issue update <mission> --issue-type task` cannot succeed once a strategy is linked, and the only way to unlink is a hand edit of the record. Not required by this record. Suggested follow-up: add a --no-strategy flag, as --no-parent works for parents.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.907028541Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    recorded_at: 2026-10-15T23:59:22.384429740Z
    resolution_commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    reason: Fixed in 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    migrated_from: atelier-77n1
---

crates/atelier-cli/src/commands/issue.rs:878: --strategy can set or change a path but never clear it, and workflow validation rejects a strategy on any non-mission ('strategy is only valid on issue_type mission'). So `issue update <mission> --issue-type task` cannot succeed once a strategy is linked, and the only way to unlink is a hand edit of the record. Not required by this record. Suggested follow-up: add a --no-strategy flag, as --no-parent works for parents.
