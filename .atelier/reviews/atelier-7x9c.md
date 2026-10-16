---
schema: atelier.review
schema_version: 1
id: atelier-7x9c
title: 'Resolved. merge_resolution (git_freshness.rs) now returns None when the commit differs from merge-tree''s result at any path that did not conflict, so the merge becomes an ordinary edge checked by allowed_edge. Replayed: approve, then git merge --no-commit of a base code commit with no conflicts, atelier issue transition <outside> start, commit; atelier merge --check now fails review.current. The test''s edit_while_merging case covers this and passes.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: acffbe36af6bb949038f0313feedb2a5c15affd2
created_at: 2026-10-16T20:09:44.664780368Z
event:
  event: disposition
  finding: atelier-1ncu
  decision: fix_now
  supersedes: null
  resolution_commit: 2f1af2e6d028dff57b520366a77d53d78fba4f0c
  follow_up: null
---

Resolved. merge_resolution (git_freshness.rs) now returns None when the commit differs from merge-tree's result at any path that did not conflict, so the merge becomes an ordinary edge checked by allowed_edge. Replayed: approve, then git merge --no-commit of a base code commit with no conflicts, atelier issue transition <outside> start, commit; atelier merge --check now fails review.current. The test's edit_while_merging case covers this and passes.
