---
schema: atelier.review
schema_version: 1
id: atelier-f4b2
title: 'low: .atelier/issues/atelier-ftqn.md:66,75 the closed item''s Close Reason now contradicts the shipped code after 1d2f67df: it gives the row as ''<status>[, merged|, not merged]'' and says ''Rows no longer use the ✓ status mark, following the issue''s examples'', while rows now print ''✓ done, merged into <base>'' / ''⚠ blocked'' per the atelier-9g48 decision (finding atelier-qjin). The close reason is the durable record of what ftqn delivered and atelier-tciw builds on it, so a reader will take the wrong format as intended. Fix: amend the Close Reason (or add a dated note on atelier-ftqn) giving the current format and pointing to the 9g48 decision.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: ce188e134bf80ab4ed03dfd2e30562d2b714ee23
created_at: 2026-10-22T20:49:02.590814453Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/atelier-ftqn.md:66,75 the closed item's Close Reason now contradicts the shipped code after 1d2f67df: it gives the row as '<status>[, merged|, not merged]' and says 'Rows no longer use the ✓ status mark, following the issue's examples', while rows now print '✓ done, merged into <base>' / '⚠ blocked' per the atelier-9g48 decision (finding atelier-qjin). The close reason is the durable record of what ftqn delivered and atelier-tciw builds on it, so a reader will take the wrong format as intended. Fix: amend the Close Reason (or add a dated note on atelier-ftqn) giving the current format and pointing to the 9g48 decision.
