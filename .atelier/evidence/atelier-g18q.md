---
created_at: "2026-06-20T02:16:15.622870526+00:00"
id: "atelier-g18q"
evidence_type: "validation"
captured_at: "2026-06-20T02:16:15.622855781+00:00"
target:
  kind: "issue"
  id: "atelier-kmmv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kmmv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Parent epic validation for workflow policy cleanup: child issues atelier-zu0t, atelier-qx40, and atelier-ih2n are closed with evidence atelier-jif3, atelier-uo65, and atelier-yght. Independent review of PR 11 found four issues; addressed by switching readiness helpers and atelier next progress from hard-coded done/archived status strings to closed_at, adding a storage regression for custom terminal statuses, and updating stale docs around archived, review/validation categories, and review.linked_pr_merged starter semantics. Review PR 11 was approved and squash-merged in Forgejo. Validation passed: target/debug/atelier workflow check; target/debug/atelier lint atelier-kmmv; cargo fmt -- --check; cargo test -p atelier-workflow; cargo test -p atelier-app project_config --lib; cargo test -p atelier-app rebuild --lib; cargo test -p atelier-cli --test cli_integration issue_type; cargo test -p atelier-cli --test cli_integration test_issue_orientation_uses_workflow_categories_and_exact_statuses; cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation; cargo test -p atelier-cli --test cli_integration test_root_status_reports_active_mission_contract_fields; cargo test -p atelier-cli commands::agent_factory::tests::dependency_rows_include_context_and_open_blocker_marker --lib; cargo test -p atelier-sqlite test_dependency_readiness_uses_closed_at_not_literal_done_status; cargo test -p atelier-cli commands::next --lib; git diff --check."
updated_at: "2026-06-20T02:16:20.189518148+00:00"
---

Parent epic validation for workflow policy cleanup: child issues atelier-zu0t, atelier-qx40, and atelier-ih2n are closed with evidence atelier-jif3, atelier-uo65, and atelier-yght. Independent review of PR 11 found four issues; addressed by switching readiness helpers and atelier next progress from hard-coded done/archived status strings to closed_at, adding a storage regression for custom terminal statuses, and updating stale docs around archived, review/validation categories, and review.linked_pr_merged starter semantics. Review PR 11 was approved and squash-merged in Forgejo. Validation passed: target/debug/atelier workflow check; target/debug/atelier lint atelier-kmmv; cargo fmt -- --check; cargo test -p atelier-workflow; cargo test -p atelier-app project_config --lib; cargo test -p atelier-app rebuild --lib; cargo test -p atelier-cli --test cli_integration issue_type; cargo test -p atelier-cli --test cli_integration test_issue_orientation_uses_workflow_categories_and_exact_statuses; cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation; cargo test -p atelier-cli --test cli_integration test_root_status_reports_active_mission_contract_fields; cargo test -p atelier-cli commands::agent_factory::tests::dependency_rows_include_context_and_open_blocker_marker --lib; cargo test -p atelier-sqlite test_dependency_readiness_uses_closed_at_not_literal_done_status; cargo test -p atelier-cli commands::next --lib; git diff --check.
