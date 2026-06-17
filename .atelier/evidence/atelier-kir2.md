---
created_at: "2026-06-17T22:28:07.684744416+00:00"
id: "atelier-kir2"
evidence_type: "validation"
captured_at: "2026-06-17T22:28:07.684736111+00:00"
target:
  kind: "issue"
  id: "atelier-7uug"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7uug"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Workflow extraction validation: atelier-workflow now owns policy loading/parsing, schema validation, status category lookup, configured initial status lookup, branch template rendering, transition lookup, and transition-from-status APIs. atelier-app workflow_policy is reduced to DB-backed check and branch-owner resolution adapters; CLI transition options/execution and bundle initial status now consume the workflow APIs. Search transcript for workflow_for_issue_type, parse_policy_text, parse_workflows, transitions_from_status, transition_for_issue_type, evaluate_policy_transition, and validators.get shows parsing/transition lookup only in atelier-workflow, with CLI retaining validator built-in execution as the DB/Git/lint/evidence adapter. Validation: cargo fmt -- --check; cargo check -p atelier-app; cargo check -p atelier-cli; cargo test -p atelier-workflow; cli_integration tests test_issue_transition_options_and_successful_execution_follow_workflow_policy, test_root_start_applies_workflow_transition_and_records_active_work, test_workflow_check_reports_policy_and_issue_record_health, test_workflow_check_rejects_legacy_issue_statuses_without_migration_path, and test_bundle_apply_accepts_partial_issue_key_refs; target/debug/atelier lint atelier-7uug; target/debug/atelier lint; git diff --check."
updated_at: "2026-06-17T22:28:11.483775833+00:00"
---

Workflow extraction validation: atelier-workflow now owns policy loading/parsing, schema validation, status category lookup, configured initial status lookup, branch template rendering, transition lookup, and transition-from-status APIs. atelier-app workflow_policy is reduced to DB-backed check and branch-owner resolution adapters; CLI transition options/execution and bundle initial status now consume the workflow APIs. Search transcript for workflow_for_issue_type, parse_policy_text, parse_workflows, transitions_from_status, transition_for_issue_type, evaluate_policy_transition, and validators.get shows parsing/transition lookup only in atelier-workflow, with CLI retaining validator built-in execution as the DB/Git/lint/evidence adapter. Validation: cargo fmt -- --check; cargo check -p atelier-app; cargo check -p atelier-cli; cargo test -p atelier-workflow; cli_integration tests test_issue_transition_options_and_successful_execution_follow_workflow_policy, test_root_start_applies_workflow_transition_and_records_active_work, test_workflow_check_reports_policy_and_issue_record_health, test_workflow_check_rejects_legacy_issue_statuses_without_migration_path, and test_bundle_apply_accepts_partial_issue_key_refs; target/debug/atelier lint atelier-7uug; target/debug/atelier lint; git diff --check.
