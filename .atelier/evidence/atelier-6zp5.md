---
created_at: "2026-06-17T22:32:03.369969007+00:00"
id: "atelier-6zp5"
evidence_type: "validation"
captured_at: "2026-06-17T22:32:03.369959962+00:00"
target:
  kind: "issue"
  id: "atelier-mwup"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mwup"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Workflow epic validation: docs define workflow ownership boundaries; atelier-workflow now owns policy loading/parsing, schema validation, status category lookup, configured initial status lookup, branch template rendering, workflow status validation, transition lookup, and transitions-from-status APIs; atelier-app workflow_policy is DB-backed check and branch-owner resolution adapter; CLI transition options/execution, bundle issue status defaults, status/readiness surfaces, and workflow check consume those APIs. Search transcript over workflow_for_issue_type, parse_policy_text, parse_workflows, parse_statuses, validate_policy, transition_for_issue_type, transitions_from_status, configured_initial_status, workflow_allows_status, evaluate_policy_transition, and validators.get shows parsing/transition lookup only in atelier-workflow, with CLI retaining only validator execution as DB/Git/lint/evidence adapter. Validations passed: cargo fmt -- --check; cargo check -p atelier-cli; cargo test -p atelier-workflow; cli_integration tests test_issue_transition_options_and_successful_execution_follow_workflow_policy and test_workflow_check_reports_policy_and_issue_record_health; target/debug/atelier lint atelier-mwup; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check. Child evidence: atelier-nhfu, atelier-kir2, atelier-ba9a."
updated_at: "2026-06-17T22:32:07.169054930+00:00"
---

Workflow epic validation: docs define workflow ownership boundaries; atelier-workflow now owns policy loading/parsing, schema validation, status category lookup, configured initial status lookup, branch template rendering, workflow status validation, transition lookup, and transitions-from-status APIs; atelier-app workflow_policy is DB-backed check and branch-owner resolution adapter; CLI transition options/execution, bundle issue status defaults, status/readiness surfaces, and workflow check consume those APIs. Search transcript over workflow_for_issue_type, parse_policy_text, parse_workflows, parse_statuses, validate_policy, transition_for_issue_type, transitions_from_status, configured_initial_status, workflow_allows_status, evaluate_policy_transition, and validators.get shows parsing/transition lookup only in atelier-workflow, with CLI retaining only validator execution as DB/Git/lint/evidence adapter. Validations passed: cargo fmt -- --check; cargo check -p atelier-cli; cargo test -p atelier-workflow; cli_integration tests test_issue_transition_options_and_successful_execution_follow_workflow_policy and test_workflow_check_reports_policy_and_issue_record_health; target/debug/atelier lint atelier-mwup; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check. Child evidence: atelier-nhfu, atelier-kir2, atelier-ba9a.
