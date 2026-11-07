---
schema: atelier.review
schema_version: 2
id: atelier-7ifl
title: 'crates/atelier-cli/src/commands/workflow.rs:419 — in ''issue transition <id>'' options, a blocked transition''s ''Command:'' line now prints straight after the ''Failed Requirements'' sub-heading''s items, so it reads as one more failed requirement (''Failed Requirements / ---- / validator blockers.transitive_none_open / Command: atelier issue transition <id> start''). On master the separate ''Commands'' heading kept it apart. Scenario: any blocked transition with failed requirements, e.g. a start blocked by an open blocker or supersede missing --reason. Fix: push the Command: line with the transition''s own fields (after To:/Requirements, before the Failed Requirements/Description lists).'
target: atelier-onwh
identity: Independent reviewer (Claude subagent)
commit: 81e00ba8711cfead5a7dd7743deb6cf53d00af80
created_at: 2026-10-17T07:08:38.188253177Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: c5fa522e6015a1c9b6e7ebe6b466ceded5f7b34f
    recorded_at: 2026-10-17T07:12:53.802539110Z
    resolution_commit: ab0b9724d7ef9197a7138e2509fa8c216343fe92
    reason: 'Command: follows To: in each option; the test pins the order.'
    migrated_from: atelier-7k6v
---

crates/atelier-cli/src/commands/workflow.rs:419 — in 'issue transition <id>' options, a blocked transition's 'Command:' line now prints straight after the 'Failed Requirements' sub-heading's items, so it reads as one more failed requirement ('Failed Requirements / ---- / validator blockers.transitive_none_open / Command: atelier issue transition <id> start'). On master the separate 'Commands' heading kept it apart. Scenario: any blocked transition with failed requirements, e.g. a start blocked by an open blocker or supersede missing --reason. Fix: push the Command: line with the transition's own fields (after To:/Requirements, before the Failed Requirements/Description lists).
