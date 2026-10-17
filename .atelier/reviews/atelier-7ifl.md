---
schema: atelier.review
schema_version: 1
id: atelier-7ifl
title: 'crates/atelier-cli/src/commands/workflow.rs:419 — in ''issue transition <id>'' options, a blocked transition''s ''Command:'' line now prints straight after the ''Failed Requirements'' sub-heading''s items, so it reads as one more failed requirement (''Failed Requirements / ---- / validator blockers.transitive_none_open / Command: atelier issue transition <id> start''). On master the separate ''Commands'' heading kept it apart. Scenario: any blocked transition with failed requirements, e.g. a start blocked by an open blocker or supersede missing --reason. Fix: push the Command: line with the transition''s own fields (after To:/Requirements, before the Failed Requirements/Description lists).'
target: atelier-onwh
identity: Independent reviewer (Claude subagent)
commit: 81e00ba8711cfead5a7dd7743deb6cf53d00af80
created_at: 2026-10-17T07:08:38.188253177Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/workflow.rs:419 — in 'issue transition <id>' options, a blocked transition's 'Command:' line now prints straight after the 'Failed Requirements' sub-heading's items, so it reads as one more failed requirement ('Failed Requirements / ---- / validator blockers.transitive_none_open / Command: atelier issue transition <id> start'). On master the separate 'Commands' heading kept it apart. Scenario: any blocked transition with failed requirements, e.g. a start blocked by an open blocker or supersede missing --reason. Fix: push the Command: line with the transition's own fields (after To:/Requirements, before the Failed Requirements/Description lists).
