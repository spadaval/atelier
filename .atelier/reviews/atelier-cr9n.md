---
schema: atelier.review
schema_version: 1
id: atelier-cr9n
title: 'G10: The new cat-file --batch -Z invocation introduces an undocumented Git>=2.42 requirement. Independent valid-land reproduction with an extracted Git2.39.5 binary fails unknown switch Z, while host Git2.47 passes the same fixture. Preserve the documented Git prerequisite and exact binary object framing; no system installation was used for the reproduction.'
target: atelier-1ioo
identity: session-reviewer
commit: 6f5188dd5bc990c88733a0027ee703eaaef9f737
created_at: 2026-10-02T00:43:05.724449506Z
event:
  event: finding
  recommendation: fix_now
---

G10: The new cat-file --batch -Z invocation introduces an undocumented Git>=2.42 requirement. Independent valid-land reproduction with an extracted Git2.39.5 binary fails unknown switch Z, while host Git2.47 passes the same fixture. Preserve the documented Git prerequisite and exact binary object framing; no system installation was used for the reproduction.
