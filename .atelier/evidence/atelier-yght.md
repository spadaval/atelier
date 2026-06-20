---
created_at: "2026-06-20T02:03:06.590721917+00:00"
id: "atelier-yght"
evidence_type: "validation"
captured_at: "2026-06-20T02:03:06.590711486+00:00"
target:
  kind: "issue"
  id: "atelier-ih2n"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ih2n"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Made configured statuses primary in compact user-facing labels: issue headers, dependency labels, and mission active-work rows now render statuses like in_progress, todo, and done without category/status pairs; category remains in explicit detail lines and rollups. Validation passed: cargo fmt -- --check; cargo build -p atelier-cli; cargo test -p atelier-cli --test cli_integration test_issue_orientation_uses_workflow_categories_and_exact_statuses; cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation; cargo test -p atelier-cli --test cli_integration test_root_status_reports_active_mission_contract_fields; cargo test -p atelier-cli --test cli_integration test_root_status_guides_current_work_to_transition_and_worktree_status; cargo test -p atelier-cli --test cli_integration issues::test_issue_create_update_and_transition_use_custom_issue_type; cargo test -p atelier-cli commands::agent_factory::tests::dependency_rows_include_context_and_open_blocker_marker --lib; target/debug/atelier issue show atelier-ih2n; target/debug/atelier mission status atelier-v6et; target/debug/atelier lint atelier-ih2n; git diff --check. A broader mission_projection_worktree run exposed unrelated review-room/terminal-check failures and was not used as passing proof."
updated_at: "2026-06-20T02:03:11.198992703+00:00"
---

Made configured statuses primary in compact user-facing labels: issue headers, dependency labels, and mission active-work rows now render statuses like in_progress, todo, and done without category/status pairs; category remains in explicit detail lines and rollups. Validation passed: cargo fmt -- --check; cargo build -p atelier-cli; cargo test -p atelier-cli --test cli_integration test_issue_orientation_uses_workflow_categories_and_exact_statuses; cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation; cargo test -p atelier-cli --test cli_integration test_root_status_reports_active_mission_contract_fields; cargo test -p atelier-cli --test cli_integration test_root_status_guides_current_work_to_transition_and_worktree_status; cargo test -p atelier-cli --test cli_integration issues::test_issue_create_update_and_transition_use_custom_issue_type; cargo test -p atelier-cli commands::agent_factory::tests::dependency_rows_include_context_and_open_blocker_marker --lib; target/debug/atelier issue show atelier-ih2n; target/debug/atelier mission status atelier-v6et; target/debug/atelier lint atelier-ih2n; git diff --check. A broader mission_projection_worktree run exposed unrelated review-room/terminal-check failures and was not used as passing proof.
