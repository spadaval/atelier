---
schema: atelier.review
schema_version: 1
id: atelier-q5uh
title: finding on atelier-0uaj @d0752d08
target: atelier-0uaj
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: d0752d08c118347563e06c6c7d38857b2cdad47e
created_at: 2026-10-23T16:09:54.984730111Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/gate_report.rs:33-47 Uncommitted::new keeps only .atelier/issues paths, so an uncommitted approval record (.atelier/reviews/<id>.md, new) is not treated as differing. Repro: done+closed owner with committed state, then 'atelier review approve' without committing; 'land'/'merge --check' print review.current 'no independent approval ... Next: atelier review request <owner>' beside '✓ 5 checks pass (... work.terminal)' and the records line. The review fix is wrong (the approval exists in the checkout; requesting a review is not needed) and passes are shown for an owner whose scope record differs: the same class as atelier-6ajc. Committing records (first line) does clear it, so it converges, but the printed review.current fix does not fix. Fix: include review (and any non-issue) record paths in Uncommitted, mapping .atelier/reviews/* to the issues they review, or treat review.current as differing whenever any review record differs, giving it the records-commit fix and dropping the owner passes; add a land/merge test with an uncommitted approval.
