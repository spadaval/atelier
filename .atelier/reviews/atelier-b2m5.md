---
schema: atelier.review
schema_version: 2
id: atelier-b2m5
title: finding on atelier-dkul @859f41a2
target: atelier-dkul
identity: plan-reviewer-g6rp
commit: 859f41a2454c4278bc33d18868729b139e7e90f7
created_at: 2026-11-08T05:58:08.171002117Z
event:
  event: finding
  recommendation: fix_now
---

[low] Between dkul merging and atelier-6oxt deleting the old copy (6oxt waits on 9s9i, 45gr, 8bj8 and bxs7), two full copies of the skill exist. The live one is still spadaval/.agents' copy until bxs7. Nothing tells an agent working in ~/projects/agents-skills that its copy is frozen, so the strategy's 'never edited in two places' rests on memory. Fix: in the same step as the move, have dkul add a one-line 'moved to spadaval/atelier; do not edit' notice to spadaval/.agents' README and the skill's maintainers/ there, or run bxs7 and 6oxt straight after dkul.
