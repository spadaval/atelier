---
created_at: "2026-06-14T06:57:55.758544102+00:00"
id: "atelier-54o9"
evidence_type: "validation"
captured_at: "2026-06-14T06:57:55.758504591+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-gh3m"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-gh3m"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "folded workflow setup/help guidance into init, lint, status, mission, doctor, and docs; proof: cargo test --test cli_integration test_top_level_help_only_shows_core_commands test_init_creates_atelier_directory test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_workflow_init_writes_starter_policy_in_fresh_repo test_workflow_check_reports_policy_and_issue_record_health test_root_status_summarizes_checkout_orientation test_mission_closeout_uses_contract_audit test_issue_create_after_workflow_init_uses_configured_initial_status test_workflow_configuration_docs_describe_internal_diagnostics; cargo fmt -- --check; git diff --check; atelier lint"
updated_at: "2026-06-14T06:57:57.451335403+00:00"
---

folded workflow setup/help guidance into init, lint, status, mission, doctor, and docs; proof: cargo test --test cli_integration test_top_level_help_only_shows_core_commands test_init_creates_atelier_directory test_workflow_help_is_scoped_as_advanced_internal_diagnostic test_workflow_init_writes_starter_policy_in_fresh_repo test_workflow_check_reports_policy_and_issue_record_health test_root_status_summarizes_checkout_orientation test_mission_closeout_uses_contract_audit test_issue_create_after_workflow_init_uses_configured_initial_status test_workflow_configuration_docs_describe_internal_diagnostics; cargo fmt -- --check; git diff --check; atelier lint
