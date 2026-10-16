---
schema: atelier.review
schema_version: 1
id: atelier-as4e
title: 'A conflicted in-scope record is checked only with allowed_edge against the branch side (crates/atelier-app/src/review_gate.rs:212-227; union_preserving is skipped when in_scope_issue). So taking ''ours'' silently drops the base''s change to the owner''s own record and keeps the approval. Reproduced: the owner has a done child on its branch and is approved. On the base, atelier issue update C --parent <owner> adds an open child C. The base merge conflicts only in the owner''s record (children). git checkout --ours, commit. C is gone from the owner''s children, and atelier merge --check passes review.current and work.terminal, so a fast-forward merge would orphan C on the base. A clean or union merge of the same base change correctly stales the approval. The same route drops a base ''blocked_by <owner>'' link stored in the owner''s blocks. docs/product/reviews-and-merge.md:65-68 claims each conflicted record keeps every line either side added, which is false for in-scope records. Fix: require union preservation for in-scope conflicts too, plus status-only against the branch side, and add the in-scope ''ours'' case to the test.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: acffbe36af6bb949038f0313feedb2a5c15affd2
created_at: 2026-10-16T20:09:44.913758619Z
event:
  event: finding
  recommendation: fix_now
---

A conflicted in-scope record is checked only with allowed_edge against the branch side (crates/atelier-app/src/review_gate.rs:212-227; union_preserving is skipped when in_scope_issue). So taking 'ours' silently drops the base's change to the owner's own record and keeps the approval. Reproduced: the owner has a done child on its branch and is approved. On the base, atelier issue update C --parent <owner> adds an open child C. The base merge conflicts only in the owner's record (children). git checkout --ours, commit. C is gone from the owner's children, and atelier merge --check passes review.current and work.terminal, so a fast-forward merge would orphan C on the base. A clean or union merge of the same base change correctly stales the approval. The same route drops a base 'blocked_by <owner>' link stored in the owner's blocks. docs/product/reviews-and-merge.md:65-68 claims each conflicted record keeps every line either side added, which is false for in-scope records. Fix: require union preservation for in-scope conflicts too, plus status-only against the branch side, and add the in-scope 'ours' case to the test.
