---
schema: atelier.review
schema_version: 2
id: atelier-49hb
title: 'crates/atelier-cli/src/commands/issue.rs:878 vs 906: `issue create/update --strategy <missing path>` succeeds, but any later `issue update <mission> --body ...` on that mission is refused with ''Invalid --body for <id>: Mission strategy <path> does not exist'', because the body check now runs issue_findings, which includes the new strategy-link findings. The error blames --body for something --body did not cause, and it blocks body edits for a mission whose strategy is linked before the file is written. Reproduced in /tmp: update --strategy docs/s/missing2.md exits 0; update --body ''## Outcome\no'' then fails. Fix: either check the strategy link when --strategy is set (refuse a missing file or a front matter that disagrees, at create and update), or leave the strategy-link findings out of the --body check so they are reported only by check. Add a test.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.676775027Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    recorded_at: 2026-10-15T23:59:22.191902999Z
    resolution_commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    reason: Fixed in 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    migrated_from: atelier-b5xk
---

crates/atelier-cli/src/commands/issue.rs:878 vs 906: `issue create/update --strategy <missing path>` succeeds, but any later `issue update <mission> --body ...` on that mission is refused with 'Invalid --body for <id>: Mission strategy <path> does not exist', because the body check now runs issue_findings, which includes the new strategy-link findings. The error blames --body for something --body did not cause, and it blocks body edits for a mission whose strategy is linked before the file is written. Reproduced in /tmp: update --strategy docs/s/missing2.md exits 0; update --body '## Outcome\no' then fails. Fix: either check the strategy link when --strategy is set (refuse a missing file or a front matter that disagrees, at create and update), or leave the strategy-link findings out of the --body check so they are reported only by check. Add a test.
