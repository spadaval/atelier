---
schema: atelier.review
schema_version: 2
id: atelier-6yx1
title: finding on atelier-bg7w @f4175923
target: atelier-bg7w
identity: gpt-6.1-sol reviewer
commit: f41759232002d63dd6ff417fd4e7ded782faa78e
created_at: 2026-11-09T04:38:09.558483243Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-bg7w.md:34 R5. The required saying-no matrix includes known invalid proof: cross-repo-regression and vacuous-test-review use non-Atelier fixtures where the skill steps aside (atelier-wl6u), and rare-edge-chain is explicitly inconclusive because unplanted defects dominate its outcomes (atelier-pju0; 2026-11-08-saying-no.md). Comparing pass counts cannot establish this rewrite preserves the intended guidance. Six rare-edge runs also repeat a costly inconclusive test (the fixture allows $8 and 30 minutes per run). Require the repairs before using these cells as gates, or omit the invalid cells with an explicit limitation and substitute focused valid probes. Keep the valid regression scenarios and permit reuse of identical pinned baseline runs.
