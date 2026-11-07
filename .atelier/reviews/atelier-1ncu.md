---
schema: atelier.review
schema_version: 2
id: atelier-1ncu
title: 'Hand edits to records in a base merge keep the approval even when nothing conflicted. merge_resolution (crates/atelier-app/src/git_freshness.rs:797-814) returns every record path where the commit differs from merge-tree''s result, not only the paths merge-tree reported as conflicted, and allowed_resolution (crates/atelier-app/src/review_gate.rs:192-198) skips every out-of-scope .atelier/issues path whatever the change (status, relationships, deletion, mode). Reproduced: approve, then ''git merge --no-commit main'' of a base code commit with no conflicts, run ''atelier issue transition <outside> start'', commit. ''atelier merge --check'' passes review.current. The same edit as an ordinary commit stales the approval (out_of_scope_statuses_relationship_changes_and_existing_proof_edits_stale_approval), and the docs still say a hand-edited merge needs renewed review. docs/product/reviews-and-merge.md:66 now says records outside scope ''may be resolved freely''. Fix: count only merge-tree''s conflicted paths as a resolution. Any other differing record path should make the merge an ordinary edge, or be checked with allowed_edge.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: 961e69d6cffe8612e03d743f7efcf5a158dec220
created_at: 2026-10-16T19:55:05.438977442Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: acffbe36af6bb949038f0313feedb2a5c15affd2
    recorded_at: 2026-10-16T20:09:44.664780368Z
    resolution_commit: 2f1af2e6d028dff57b520366a77d53d78fba4f0c
    reason: 'Resolved. merge_resolution (git_freshness.rs) now returns None when the commit differs from merge-tree''s result at any path that did not conflict, so the merge becomes an ordinary edge checked by allowed_edge. Replayed: approve, then git merge --no-commit of a base code commit with no conflicts, atelier issue transition <outside> start, commit; atelier merge --check now fails review.current. The test''s edit_while_merging case covers this and passes.'
    migrated_from: atelier-7x9c
---

Hand edits to records in a base merge keep the approval even when nothing conflicted. merge_resolution (crates/atelier-app/src/git_freshness.rs:797-814) returns every record path where the commit differs from merge-tree's result, not only the paths merge-tree reported as conflicted, and allowed_resolution (crates/atelier-app/src/review_gate.rs:192-198) skips every out-of-scope .atelier/issues path whatever the change (status, relationships, deletion, mode). Reproduced: approve, then 'git merge --no-commit main' of a base code commit with no conflicts, run 'atelier issue transition <outside> start', commit. 'atelier merge --check' passes review.current. The same edit as an ordinary commit stales the approval (out_of_scope_statuses_relationship_changes_and_existing_proof_edits_stale_approval), and the docs still say a hand-edited merge needs renewed review. docs/product/reviews-and-merge.md:66 now says records outside scope 'may be resolved freely'. Fix: count only merge-tree's conflicted paths as a resolution. Any other differing record path should make the merge an ordinary edge, or be checked with allowed_edge.
