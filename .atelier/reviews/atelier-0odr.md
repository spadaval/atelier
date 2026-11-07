---
schema: atelier.review
schema_version: 2
id: atelier-0odr
title: '.atelier/issues/atelier-ermg.md:43 and docs/product/work-model.md:43: both say prune keeps decision records (citing atelier-r7cd), so the won''t-do list survives. The code does not do this. atelier-r7cd (''Decide what pruning keeps'') is still todo on master and in this branch. crates/atelier-cli/src/commands/prune.rs issue_candidates (around line 877) makes every terminal record past the retention cutoff a candidate, with no check for issue_type or a Background section. Failure: a won''t-do item recorded as a decided (done) decision record is deleted by ''atelier prune --apply'' after the retention window (7 days by default) unless an active record happens to link to it, so the won''t-do list quietly disappears. Repro: read issue_candidates and the protect_* helpers; nothing protects by type. Fix: either implement keeping decision records here (with a test), or change the decision and the doc to say won''t-do decisions are pruned like other closed records until atelier-r7cd lands, and link atelier-r7cd as the follow-up.'
target: atelier-b7hb
identity: Independent reviewer (Claude subagent)
commit: 316cc70ef3a1117026263abb3c68de001aba148f
created_at: 2026-10-16T01:36:43.065278502Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 9f26e3b8d2675960c51d317ea3023dc8be2ca9ed
    recorded_at: 2026-10-16T01:40:52.685370622Z
    resolution_commit: 9f26e3b8d2675960c51d317ea3023dc8be2ca9ed
    reason: atelier-r7cd (prune keeps decision records) merged to master and is merged here in 9f26e3b8d2675960c51d317ea3023dc8be2ca9ed, so the statement now holds
    migrated_from: atelier-twba
---

.atelier/issues/atelier-ermg.md:43 and docs/product/work-model.md:43: both say prune keeps decision records (citing atelier-r7cd), so the won't-do list survives. The code does not do this. atelier-r7cd ('Decide what pruning keeps') is still todo on master and in this branch. crates/atelier-cli/src/commands/prune.rs issue_candidates (around line 877) makes every terminal record past the retention cutoff a candidate, with no check for issue_type or a Background section. Failure: a won't-do item recorded as a decided (done) decision record is deleted by 'atelier prune --apply' after the retention window (7 days by default) unless an active record happens to link to it, so the won't-do list quietly disappears. Repro: read issue_candidates and the protect_* helpers; nothing protects by type. Fix: either implement keeping decision records here (with a test), or change the decision and the doc to say won't-do decisions are pruned like other closed records until atelier-r7cd lands, and link atelier-r7cd as the follow-up.
