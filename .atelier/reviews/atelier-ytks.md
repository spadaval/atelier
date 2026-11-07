---
schema: atelier.review
schema_version: 2
id: atelier-ytks
title: finding on atelier-imlh @64749673
target: atelier-imlh
identity: Independent reviewer (Claude Sonnet subagent)
commit: 6474967393d9266b33fd623e8af454b946073e2b
created_at: 2026-10-23T15:41:26.813178322Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 2cdc0ba6354841d71d6edfc86ee88ab3e074e986
    recorded_at: 2026-10-23T15:46:27.167685142Z
    resolution_commit: e52310719a7fa54f70f69632d3b9ad301f7ce855
    reason: Legacy rerun receipts on master would lose their summary on the next sync, defeating atelier-3xq4.
    migrated_from: atelier-ui05
---

low: crates/atelier-records/src/lib.rs:2070 evidence_summary_text takes the first '## Summary' section. A receipt made by the old merge --sync (12 such in .atelier/evidence, title starts '## Summary') nests '## Summary' inside its Summary, so the first section is empty and the next rerun gets Summary ' (rerun after merging main)' with a leading space, losing the text. Reproduced with master's binary sync then this branch's sync. Also a Summary containing its own '## Notes' heading is cut at it, and an empty Summary yields a leading-space suffix. Suggest taking the last Summary section or trimming and skipping an empty one.
