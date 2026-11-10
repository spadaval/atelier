---
schema: atelier.review
schema_version: 2
id: atelier-l6gt
title: finding on atelier-fcb7 @dab1f733
target: atelier-fcb7
identity: grok-4.6-amend-r1
commit: dab1f733ffb6d2e4bfc406ed27ccda58563b272d
created_at: 2026-11-10T02:48:41.651905280Z
event:
  event: finding
  recommendation: fix_now
---

high: docs/strategies/redesign-architecture.md:386-401 (also redesign-3-cutover.md Authority): cutover runs atelier setup for Git hooks, then plain git push of conversion commits that change AGENTS.md, the skill path, and workflows. The new pre-push rule allows only an Atelier merge or paths under .atelier/missions/ and .atelier/items/, so those pushes are refused and the runbook stops at step 4. Smallest fix: Transition names ATELIER_SKIP="cutover conversion" on those pushes, which is the bypass this amendment adds for that one exception.
