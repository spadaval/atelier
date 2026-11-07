---
schema: atelier.review
schema_version: 2
id: atelier-h2i8
title: finding on atelier-ml3r @0ac85467
target: atelier-ml3r
identity: independent reviewer subagent (claude-opus-5-5), atelier-ml3r
commit: 0ac8546728725631c173eec494bc745cfe33fc37
created_at: 2026-11-07T14:01:15.231000408Z
event:
  event: finding
  recommendation: fix_now
---

low: spadaval/.agents@fc5e3977 skills/agent-factory/references/atelier.md:68-74,158-165: the rewrite dropped the old reference's rule 'Integrate only with atelier merge, never a plain git merge into the base branch, which skips every gate'. No runtime file of the skill says it now; only maintainers/skill-evaluation.md:101 does, which the eval harness does not stage, though that table still expects agents to resist the 'Ungated merge' pressure, and ADR 0028 decision 6 accepts that a plain git merge of an approved light-path branch passes land without running merge checks because atelier merge is the documented path. Fix: restore one clause in step 6 and in the light path: never a plain git merge into the base, which runs no gate and marks nothing done.
