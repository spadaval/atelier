---
schema: atelier.review
schema_version: 2
id: atelier-qt4c
title: finding on atelier-ml3r @0ac85467
target: atelier-ml3r
identity: independent reviewer subagent (claude-opus-5-5), atelier-ml3r
commit: 0ac8546728725631c173eec494bc745cfe33fc37
created_at: 2026-11-07T14:01:15.736828074Z
event:
  event: finding
  recommendation: fix_now
---

low: CONTEXT.md:17 and docs/product/validation.md:67-68 define a validation item as an independent assessment of a mission's criteria only, while the skill (spadaval/.agents@fc5e3977 references/atelier.md:138-140, references/tracker-records.md:169-171, procedures/plan.md:20-21, procedures/orchestrate.md:100-101) also files one for an increment's user-facing outcome. The loose-end rule in CONTEXT.md, AGENTS.md and root help now turns on 'an independent validation is a validation item', so a reader of Atelier's docs would file a post-merge validation of an increment as a loose end and a reader of the skill as a validation item. Misses Done when line 7 (docs in the same words as the skill). Fix: define it in CONTEXT.md and validation.md as an independent assessment of a mission's criteria or an increment's outcome, which advances its mission.
