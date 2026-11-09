---
schema: atelier.review
schema_version: 2
id: atelier-d0vo
title: finding on atelier-8ep4 @f4175923
target: atelier-8ep4
identity: gpt-6.1-sol reviewer
commit: f41759232002d63dd6ff417fd4e7ded782faa78e
created_at: 2026-11-09T04:38:08.984283380Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-8ep4.md:38 R3. The new orchestrator-arranges-review acceptance gate requires only new passes >= old passes. Both arms can self-review in all three runs (0/3 versus 0/3) and the Done when line still holds, although the scenario's target behavior never succeeds. Keep the relative comparison as a regression check, but also require an absolute success bar for independent reviewer dispatch in the new arm; define what to do when the baseline already passes or runs are inconclusive.
