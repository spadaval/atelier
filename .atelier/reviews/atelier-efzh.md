---
schema: atelier.review
schema_version: 1
id: atelier-efzh
title: 'The test does not cover the claims. In base_merge_with_resolved_record_conflicts_keeps_approval_unless_scope_changes (crates/atelier-cli/tests/merge_gates.rs:862-920), the in_scope case makes the owner record conflict, and ''git add .'' (line 912) commits it with conflict markers in it. It goes stale only because git merge''s marker labels (HEAD/main) differ from merge-tree''s (commit IDs), not because an in-scope record was realistically resolved and refused for a non-status change. Missing cases: a non-conflicting record edit inside a base merge (would have caught the first finding); an out-of-scope blocks or relationship change pointing into scope; an evidence or review file edited in a resolution; a conflict in .atelier/workflow.yaml or config.toml staying stale. Add these, and resolve the in-scope record properly instead of committing markers.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: 961e69d6cffe8612e03d743f7efcf5a158dec220
created_at: 2026-10-16T19:55:05.697521320Z
event:
  event: finding
  recommendation: fix_now
---

The test does not cover the claims. In base_merge_with_resolved_record_conflicts_keeps_approval_unless_scope_changes (crates/atelier-cli/tests/merge_gates.rs:862-920), the in_scope case makes the owner record conflict, and 'git add .' (line 912) commits it with conflict markers in it. It goes stale only because git merge's marker labels (HEAD/main) differ from merge-tree's (commit IDs), not because an in-scope record was realistically resolved and refused for a non-status change. Missing cases: a non-conflicting record edit inside a base merge (would have caught the first finding); an out-of-scope blocks or relationship change pointing into scope; an evidence or review file edited in a resolution; a conflict in .atelier/workflow.yaml or config.toml staying stale. Add these, and resolve the in-scope record properly instead of committing markers.
