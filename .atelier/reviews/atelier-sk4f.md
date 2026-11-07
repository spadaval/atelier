---
schema: atelier.review
schema_version: 2
id: atelier-sk4f
title: finding on atelier-mgl0 @711e28e4
target: atelier-mgl0
identity: Independent code reviewer (Claude subagent, round 1)
commit: 711e28e44cd41e6a7382f337e0434b37de4b4b76
created_at: 2026-11-05T04:51:26.458897059Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: no_action
    identity: Orchestrator (Claude Opus 5.5)
    commit: df02b8aa35ff8c951439f6b5fe1a1c21aa9d5fb7
    recorded_at: 2026-11-05T04:51:56.305835107Z
    reason: 'Already fixed in 19210197 before this finding was recorded: nav hover uses --code and the contrast test checks code.'
    migrated_from: atelier-t8s7
---

The contrast test misses --line, the nav hover background, where muted, faint, warn and back fall below 4.5:1.
