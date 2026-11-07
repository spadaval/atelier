---
schema: atelier.review
schema_version: 2
id: atelier-0ukf
title: finding on atelier-0uaj @46a269e9
target: atelier-0uaj
identity: Independent reviewer (Claude Sonnet subagent)
commit: 46a269e98ef40a08ce787c618d8dd0f7b5e42bee
created_at: 2026-10-23T15:58:52.754982557Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 0724301b807acd5a90df5d5e14cbbba57cea9a0b
    recorded_at: 2026-10-23T15:59:25.025850333Z
    reason: merge_checks are read-only by policy; running them before refusing has no side effects to guard against, and the refusal still precedes integration.
    migrated_from: atelier-2tm4
---

info: real merge now runs candidate/merge shell checks against a dirty, diverged or uncommitted state before refusing (previously refused first). Refusal, exit code and base ref unchanged in all 12 scenarios compared with master binary; only side-effect exposure of shell checks differs (not exercised: no shell.check in scratch policy).
