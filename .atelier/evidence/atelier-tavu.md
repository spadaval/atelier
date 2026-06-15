---
created_at: "2026-06-15T06:09:23.631682282+00:00"
id: "atelier-tavu"
evidence_type: "validation"
captured_at: "2026-06-15T06:09:23.631635312+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-okz2"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-okz2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Runtime active issue association removed from normal current-work flows: atelier start only applies workflow status and writes no active work_association; atelier status derives current work from in_progress Markdown; deleting .atelier/runtime/state.db plus rebuild preserves current-work status; stale runtime work_associations do not affect status; rebuild clears session active_issue_id while preserving worktree metadata. Proof: cargo test --test cli_integration root_start -- --nocapture; cargo test --test cli_integration status_preserves_current_work_after_runtime_database_rebuild -- --nocapture; cargo test rebuild::tests::refresh_projection_preserves_worktree_metadata_but_clears_active_session_pointer -- --nocapture; cargo test --test cli_integration removed -- --nocapture; atelier lint atelier-okz2; atelier export --check; git diff --check."
updated_at: "2026-06-15T06:09:25.515221149+00:00"
---

Runtime active issue association removed from normal current-work flows: atelier start only applies workflow status and writes no active work_association; atelier status derives current work from in_progress Markdown; deleting .atelier/runtime/state.db plus rebuild preserves current-work status; stale runtime work_associations do not affect status; rebuild clears session active_issue_id while preserving worktree metadata. Proof: cargo test --test cli_integration root_start -- --nocapture; cargo test --test cli_integration status_preserves_current_work_after_runtime_database_rebuild -- --nocapture; cargo test rebuild::tests::refresh_projection_preserves_worktree_metadata_but_clears_active_session_pointer -- --nocapture; cargo test --test cli_integration removed -- --nocapture; atelier lint atelier-okz2; atelier export --check; git diff --check.
