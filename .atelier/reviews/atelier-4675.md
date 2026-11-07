---
schema: atelier.review
schema_version: 2
id: atelier-4675
title: 'crates/atelier-cli/src/commands/git.rs:500 — status''s Other Local Work Branches rows always add a ''| N active'' cue, including ''| 0 active'' (seen live: task/atelier-7w1g ... | not merged into master | 0 active). The new Counts rule says a list of counts leaves out its zeros. This predates the change and is a row cue, not a summary line, so it does not block. Follow-up: drop the cue at zero, or document why it stays.'
target: atelier-krf4
identity: Independent reviewer (Claude subagent)
commit: 23527f8026ade3ebd70c6740ddb995c23d25fe87
created_at: 2026-10-17T07:38:21.732008126Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 4e8e51630991ab8cf13516ff566284c5e05eb760
    recorded_at: 2026-10-17T07:48:33.166396848Z
    resolution_commit: 5927bc001a2354c3e2a5e1c54f0953843e78bc94
    reason: 'Fixed now: branch rows drop the active cue at zero; a session test covers both cases.'
    migrated_from: atelier-cm18
---

crates/atelier-cli/src/commands/git.rs:500 — status's Other Local Work Branches rows always add a '| N active' cue, including '| 0 active' (seen live: task/atelier-7w1g ... | not merged into master | 0 active). The new Counts rule says a list of counts leaves out its zeros. This predates the change and is a row cue, not a summary line, so it does not block. Follow-up: drop the cue at zero, or document why it stays.
