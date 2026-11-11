---
schema: atelier.review
schema_version: 2
id: atelier-5yj2
title: finding on atelier-5zre @6d606a2c
target: atelier-5zre
identity: GPT-6.1 Sol independent reviewer, round 9
commit: 6d606a2cb985849dbd4b2222e477a39bb4a1108e
created_at: 2026-11-11T05:54:31.161257277Z
event:
  event: finding
  recommendation: fix_now
---

R2 low: .atelier/issues/atelier-jcki.md:30 Shrink: keep the surviving mission-title requirement in this rewrite. At fc0fe395 the first Done when explicitly requires Mission: atelier-1w8c followed by its title, from any branch. The rewrite retains only stale-approval reporting and says the new show already prints the mission. On a fresh conversion of 6d606a2c, atelier-next-7b06ed12 show atelier-jcki prints Mission: atelier-4pzj (draft), without that mission's title; show atelier-4pzj is still needed to learn it. The same omission exists in the current view renderer, which formats only the mission ID and status. Missions and their titles survive the redesign, so removing old issue show and parent cases does not remove this requirement. Restore the mission-title clause and its CLI test, mapped to show <id>, alongside the retained approval-freshness requirement; correct the rewrite explanation here and in atelier-5zre's Report. This asks only to preserve the work in the records. Proof output is ~/.cache/atelier-qr5s/5zre-review9/probes.txt. Recommendation: fix_now because the fourth Done when requires rewrites to keep the surviving need as stated.
