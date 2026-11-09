---
schema: atelier.review
schema_version: 2
id: atelier-af8h
title: finding on atelier-06jc @ad3625bf
target: atelier-06jc
identity: grok-4.6-plan-reviewer
commit: ad3625bff949661871ae77a2dd69fc48c733f5ea
created_at: 2026-11-09T05:21:19.938581112Z
event:
  event: finding
  recommendation: defer
---

medium: atelier-06jc.md The extract shows 189 reviewer issue show --full runs (34 of the item named in the prompt), not 65, and Sol review show/list/evidence is about 11% of review tokens, not 14% (14% is the broader read-records bucket). Truncation to 300 chars with no id, proof lines naming review show, and review show <item-id> refusing are real. Correct the counts so Scope in the brief is not sold as a 14% save; keep those three fixes.
