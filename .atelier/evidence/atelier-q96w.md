---
created_at: "2026-06-14T00:15:16.366808125+00:00"
id: "atelier-q96w"
evidence_type: "validation"
captured_at: "2026-06-14T00:15:16.366694506+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wj05"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Refactored workflow parser/validator and transition adapter boundaries. Parser helpers split workflow entry parsing, transition parsing, status selector parsing, and required-field validation in src/workflow_policy.rs. Validator helpers split issue-type mappings, workflow statuses, terminal statuses, transition status checks, and transition reference checks. Command adapter helpers split transition lookup, availability checks, blocker evaluation, blocked reporting, canonical record updates, and activity logging in src/commands/workflow.rs. Checks passed: cargo nextest run test_issue_transition_options_and_successful_execution_follow_workflow_policy test_workflow_check_reports_policy_and_issue_record_health test_workflow_check_rejects_issue_status_outside_selected_workflow test_issue_create_after_workflow_init_uses_configured_initial_status; cargo fmt -- --check; target/debug/atelier workflow check; target/debug/atelier lint; target/debug/atelier export --check; git diff --check."
updated_at: "2026-06-14T00:15:19.108447346+00:00"
---

Refactored workflow parser/validator and transition adapter boundaries. Parser helpers split workflow entry parsing, transition parsing, status selector parsing, and required-field validation in src/workflow_policy.rs. Validator helpers split issue-type mappings, workflow statuses, terminal statuses, transition status checks, and transition reference checks. Command adapter helpers split transition lookup, availability checks, blocker evaluation, blocked reporting, canonical record updates, and activity logging in src/commands/workflow.rs. Checks passed: cargo nextest run test_issue_transition_options_and_successful_execution_follow_workflow_policy test_workflow_check_reports_policy_and_issue_record_health test_workflow_check_rejects_issue_status_outside_selected_workflow test_issue_create_after_workflow_init_uses_configured_initial_status; cargo fmt -- --check; target/debug/atelier workflow check; target/debug/atelier lint; target/debug/atelier export --check; git diff --check.
