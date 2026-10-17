---
schema: atelier.review
schema_version: 1
id: atelier-f9vp
title: 'crates/atelier-cli/src/commands/work.rs:764-780 and crates/atelier-app/src/criteria.rs:258: a pass pinned before any later code is still ''✓ proven'', and the ''Criteria:'' summary line drops the age entirely. In a scratch clone a receipt pinned 782 code commits before the feature existed rendered ''✓ proven ... | 782 code commits since'' and ''Criteria: 8: 8 proven''. The row is honest about age, but the mark and the count present it as passing without any freshness judgement, which the strategy''s Shape forbids for reports, and debrief (S4) will reuse this lookup. Fix: keep the three S8 marks but make the count line say how many proven criteria have code commits since their proof (e.g. ''8 criteria: 8 proven, 8 with code commits since''), and record a decision on the freshness rule debrief must apply before atelier-rx22 builds on mission_proof.'
target: atelier-ho7o
identity: Independent reviewer (Claude subagent)
commit: 9d56d55fc73d6ac4a9dd5b30848f5f364f5eb0e2
created_at: 2026-10-17T18:21:22.865720825Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/work.rs:764-780 and crates/atelier-app/src/criteria.rs:258: a pass pinned before any later code is still '✓ proven', and the 'Criteria:' summary line drops the age entirely. In a scratch clone a receipt pinned 782 code commits before the feature existed rendered '✓ proven ... | 782 code commits since' and 'Criteria: 8: 8 proven'. The row is honest about age, but the mark and the count present it as passing without any freshness judgement, which the strategy's Shape forbids for reports, and debrief (S4) will reuse this lookup. Fix: keep the three S8 marks but make the count line say how many proven criteria have code commits since their proof (e.g. '8 criteria: 8 proven, 8 with code commits since'), and record a decision on the freshness rule debrief must apply before atelier-rx22 builds on mission_proof.
