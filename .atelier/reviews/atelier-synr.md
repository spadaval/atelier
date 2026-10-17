---
schema: atelier.review
schema_version: 1
id: atelier-synr
title: 'The closure in touched_claims follows parent_id, blocks and children, but work.terminal on a mission also follows ''advances'' links (issue_descendant_ids walks get_typed_relations for missions), and those are stored under relates. Under a valid policy that adds work.terminal to the objective close transition, reopening an issue that advances a done mission (or reopening a child of such an issue) edits only that issue''s record. The closure never reaches the mission, so its claim is skipped. Reproduced in a scratch repo with that one-line policy change, on the base with an upstream: land passes; doctor reports ''<mission> work.terminal: open work: <issue> [todo]''. Today''s default policy has no work.terminal on missions, so this repository is not affected yet. Fix: in linked(), also follow relates entries with the advances role (both directions, like get_typed_relations), and add a test with a mission policy that uses work.terminal.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: 438be41715aeeb6757c796ca3318d8dc956876f3
created_at: 2026-10-16T23:44:57.840821490Z
event:
  event: finding
  recommendation: fix_now
---

The closure in touched_claims follows parent_id, blocks and children, but work.terminal on a mission also follows 'advances' links (issue_descendant_ids walks get_typed_relations for missions), and those are stored under relates. Under a valid policy that adds work.terminal to the objective close transition, reopening an issue that advances a done mission (or reopening a child of such an issue) edits only that issue's record. The closure never reaches the mission, so its claim is skipped. Reproduced in a scratch repo with that one-line policy change, on the base with an upstream: land passes; doctor reports '<mission> work.terminal: open work: <issue> [todo]'. Today's default policy has no work.terminal on missions, so this repository is not affected yet. Fix: in linked(), also follow relates entries with the advances role (both directions, like get_typed_relations), and add a test with a mission policy that uses work.terminal.
