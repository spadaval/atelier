---
schema: atelier.review
schema_version: 1
id: atelier-641q
title: 'The test covers only half of what the upstream fallback does. Changing ''.or_else(|| had_status(upstream))'' to ''.or_else(|| None)'' (treat an unparsable parent as a new claim) still passes base_done_claims_survive..., the epic-migration test and base_refuses_done_claims...; only changing it to ''already done'' fails the first half. The Close Reason''s ''replacing the upstream fallback ... fails it'' is true only for that direction. No test covers the case the fallback exists for: done upstream, broken locally, restored unchanged, which should land without land_base_done_claim (my probe shows it does).'
target: atelier-hyew
identity: Independent reviewer (Claude subagent)
commit: 387dbfad150d05d24d840cfefcc70e4154673ba4
created_at: 2026-10-16T20:46:11.288585539Z
event:
  event: finding
  recommendation: defer
---

The test covers only half of what the upstream fallback does. Changing '.or_else(|| had_status(upstream))' to '.or_else(|| None)' (treat an unparsable parent as a new claim) still passes base_done_claims_survive..., the epic-migration test and base_refuses_done_claims...; only changing it to 'already done' fails the first half. The Close Reason's 'replacing the upstream fallback ... fails it' is true only for that direction. No test covers the case the fallback exists for: done upstream, broken locally, restored unchanged, which should land without land_base_done_claim (my probe shows it does).
