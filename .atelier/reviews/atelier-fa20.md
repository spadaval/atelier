---
schema: atelier.review
schema_version: 2
id: atelier-fa20
title: finding on atelier-mgl0 @711e28e4
target: atelier-mgl0
identity: Independent code reviewer (Claude subagent, round 1)
commit: 711e28e44cd41e6a7382f337e0434b37de4b4b76
created_at: 2026-11-05T04:51:26.674427758Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude Opus 5.5)
    commit: df02b8aa35ff8c951439f6b5fe1a1c21aa9d5fb7
    recorded_at: 2026-11-05T04:51:57.117182322Z
    follow_up: atelier-g21n
    reason: 'Deferred: accuracy first; leaving pointer-only parts out is its own change.'
    migrated_from: atelier-jhxg
---

Pointer-only Decisions parts now repeat the decision notes they point at.
