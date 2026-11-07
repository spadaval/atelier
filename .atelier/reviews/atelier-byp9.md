---
schema: atelier.review
schema_version: 2
id: atelier-byp9
title: 'Low: record_index.rs:289-303 drops an evidence or review record whose ID collides with another record (cross-kind collisions pass the path check), then every reference to it is reported as missing, e.g. ''atelier-ev02 has related reference to missing evidence X'' next to ''Duplicate record ID in record files: X''. That is a spurious cascade error; the old code and ba7512b7 reported only the duplicate. The first error, and so fail-fast loading, is unchanged, and random IDs make the collision unlikely. Fix: remember the dropped duplicate IDs and suppress missing-target errors for them in ensure_record/exists. Recommend DEFER.'
target: atelier-1ioo
identity: independent-reviewer-3-2026-10-15
commit: 8f13d015eda0d0d702e77bf318fb686565e62955
created_at: 2026-10-15T19:30:04.012422523Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: independent-reviewer-3-2026-10-15
    commit: 268945b756e90d567a66c3cadd64e3339947c0fa
    recorded_at: 2026-10-15T19:30:54.392714064Z
    follow_up: atelier-ip9e
    reason: 'Deferred to atelier-ip9e, which covers the cascade and its fixture. The bug is low impact: it only happens when an evidence or review ID collides with a record of another kind, which random IDs make unlikely. check still fails closed, and the first error and fail-fast loading are unchanged.'
    migrated_from: atelier-i15x
---

Low: record_index.rs:289-303 drops an evidence or review record whose ID collides with another record (cross-kind collisions pass the path check), then every reference to it is reported as missing, e.g. 'atelier-ev02 has related reference to missing evidence X' next to 'Duplicate record ID in record files: X'. That is a spurious cascade error; the old code and ba7512b7 reported only the duplicate. The first error, and so fail-fast loading, is unchanged, and random IDs make the collision unlikely. Fix: remember the dropped duplicate IDs and suppress missing-target errors for them in ensure_record/exists. Recommend DEFER.
