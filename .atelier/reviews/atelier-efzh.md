---
schema: atelier.review
schema_version: 2
id: atelier-efzh
title: 'The test does not cover the claims. In base_merge_with_resolved_record_conflicts_keeps_approval_unless_scope_changes (crates/atelier-cli/tests/merge_gates.rs:862-920), the in_scope case makes the owner record conflict, and ''git add .'' (line 912) commits it with conflict markers in it. It goes stale only because git merge''s marker labels (HEAD/main) differ from merge-tree''s (commit IDs), not because an in-scope record was realistically resolved and refused for a non-status change. Missing cases: a non-conflicting record edit inside a base merge (would have caught the first finding); an out-of-scope blocks or relationship change pointing into scope; an evidence or review file edited in a resolution; a conflict in .atelier/workflow.yaml or config.toml staying stale. Add these, and resolve the in-scope record properly instead of committing markers.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: 961e69d6cffe8612e03d743f7efcf5a158dec220
created_at: 2026-10-16T19:55:05.697521320Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: decb65a8e086ca314995b9534104ce64d67261eb
    recorded_at: 2026-10-16T20:21:12.803444201Z
    resolution_commit: 9c48a595e3a3a3799103820a7a2609c30488fd24
    reason: Resolved. Resolutions are now realistic records written through the records API (union, keep_blocker), not committed conflict markers; edit_while_merging covers a non-conflicting record edit in a base merge, drop_blocker covers an out-of-scope blocks link into scope, drop_child and drop_side cover take-ours, base_scope_edit covers in-scope non-status edits. Evidence, review and policy-file edits during a merge fall to the generic merge-tree equality check exercised by the edited and conflict cases of the clean-merge test. merge_gates passes 43/43.
    migrated_from: atelier-kqt4
---

The test does not cover the claims. In base_merge_with_resolved_record_conflicts_keeps_approval_unless_scope_changes (crates/atelier-cli/tests/merge_gates.rs:862-920), the in_scope case makes the owner record conflict, and 'git add .' (line 912) commits it with conflict markers in it. It goes stale only because git merge's marker labels (HEAD/main) differ from merge-tree's (commit IDs), not because an in-scope record was realistically resolved and refused for a non-status change. Missing cases: a non-conflicting record edit inside a base merge (would have caught the first finding); an out-of-scope blocks or relationship change pointing into scope; an evidence or review file edited in a resolution; a conflict in .atelier/workflow.yaml or config.toml staying stale. Add these, and resolve the in-scope record properly instead of committing markers.
