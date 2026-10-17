---
schema: atelier.review
schema_version: 1
id: atelier-uign
title: 'crates/atelier-cli/src/commands/prune.rs:871-1059 — no test pins prune''s switch to the local day: reverting canonical_retention_cutoff or the issue/review/evidence comparisons to UTC date_naive() leaves every prune test passing, because their fixtures sit 0, 1, 30+ or 2000 days away from the cutoff and no test runs prune under a TZ whose day differs from UTC''s. The code change is mechanical and consistent (cutoff and record dates both use local_date, so no off-by-one), and the done-when requires a test only for notes, so this does not block. A test could set a record''s closed_at so that its local day under TZ=UTC-14 equals today while its UTC day is yesterday, and assert prune --retention-days 0 keeps it.'
target: atelier-7iyw
identity: Independent reviewer (Claude subagent)
commit: e65c1fb0c327d2e93144387b0e81d0e298acdd47
created_at: 2026-10-17T07:03:36.770228208Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/prune.rs:871-1059 — no test pins prune's switch to the local day: reverting canonical_retention_cutoff or the issue/review/evidence comparisons to UTC date_naive() leaves every prune test passing, because their fixtures sit 0, 1, 30+ or 2000 days away from the cutoff and no test runs prune under a TZ whose day differs from UTC's. The code change is mechanical and consistent (cutoff and record dates both use local_date, so no off-by-one), and the done-when requires a test only for notes, so this does not block. A test could set a record's closed_at so that its local day under TZ=UTC-14 equals today while its UTC day is yesterday, and assert prune --retention-days 0 keeps it.
