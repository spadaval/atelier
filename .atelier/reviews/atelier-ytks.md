---
schema: atelier.review
schema_version: 1
id: atelier-ytks
title: finding on atelier-imlh @64749673
target: atelier-imlh
identity: Independent reviewer (Claude Sonnet subagent)
commit: 6474967393d9266b33fd623e8af454b946073e2b
created_at: 2026-10-23T15:41:26.813178322Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-records/src/lib.rs:2070 evidence_summary_text takes the first '## Summary' section. A receipt made by the old merge --sync (12 such in .atelier/evidence, title starts '## Summary') nests '## Summary' inside its Summary, so the first section is empty and the next rerun gets Summary ' (rerun after merging main)' with a leading space, losing the text. Reproduced with master's binary sync then this branch's sync. Also a Summary containing its own '## Notes' heading is cut at it, and an empty Summary yields a leading-space suffix. Suggest taking the last Summary section or trimming and skipping an empty one.
