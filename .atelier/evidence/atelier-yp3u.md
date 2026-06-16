---
created_at: "2026-06-14T08:04:58.376752422+00:00"
id: "atelier-yp3u"
evidence_type: "validation"
captured_at: "2026-06-14T08:04:58.376638294+00:00"
target:
  kind: "issue"
  id: "atelier-uy4o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uy4o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Implemented checkout-scoped active work enforcement. Passing checks: cargo fmt -- --check; git diff --check; cargo build; cargo test --test cli_integration test_root_start_rejects_different_active_issue_in_same_worktree; cargo test --test cli_integration test_root_start_same_issue_refreshes_single_active_association; cargo test --test cli_integration test_abandon_clears_scoped_active_issue_and_allows_switching; cargo test --test cli_integration test_separate_worktrees_can_have_different_active_issues; target/debug/atelier lint; target/debug/atelier export --check."
updated_at: "2026-06-14T08:05:01.153997494+00:00"
---

Implemented checkout-scoped active work enforcement. Passing checks: cargo fmt -- --check; git diff --check; cargo build; cargo test --test cli_integration test_root_start_rejects_different_active_issue_in_same_worktree; cargo test --test cli_integration test_root_start_same_issue_refreshes_single_active_association; cargo test --test cli_integration test_abandon_clears_scoped_active_issue_and_allows_switching; cargo test --test cli_integration test_separate_worktrees_can_have_different_active_issues; target/debug/atelier lint; target/debug/atelier export --check.
