---
schema: atelier.review
schema_version: 1
id: atelier-byp9
title: 'Low: record_index.rs:289-303 drops an evidence or review record whose ID collides with another record (cross-kind collisions pass the path check), then every reference to it is reported as missing, e.g. ''atelier-ev02 has related reference to missing evidence X'' next to ''Duplicate record ID in record files: X''. That is a spurious cascade error; the old code and ba7512b7 reported only the duplicate. The first error, and so fail-fast loading, is unchanged, and random IDs make the collision unlikely. Fix: remember the dropped duplicate IDs and suppress missing-target errors for them in ensure_record/exists. Recommend DEFER.'
target: atelier-1ioo
identity: independent-reviewer-3-2026-10-15
commit: 8f13d015eda0d0d702e77bf318fb686565e62955
created_at: 2026-10-15T19:30:04.012422523Z
event:
  event: finding
  recommendation: defer
---

Low: record_index.rs:289-303 drops an evidence or review record whose ID collides with another record (cross-kind collisions pass the path check), then every reference to it is reported as missing, e.g. 'atelier-ev02 has related reference to missing evidence X' next to 'Duplicate record ID in record files: X'. That is a spurious cascade error; the old code and ba7512b7 reported only the duplicate. The first error, and so fail-fast loading, is unchanged, and random IDs make the collision unlikely. Fix: remember the dropped duplicate IDs and suppress missing-target errors for them in ensure_record/exists. Recommend DEFER.
