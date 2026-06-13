---
created_at: "2026-06-13T20:08:48.798223763+00:00"
id: "atelier-9ypo"
issue_type: "task"
labels:
- "cleanup"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Remove legacy workflow migration and hard-coded statuses"
updated_at: "2026-06-13T21:04:10.297185984+00:00"
---

## Description

Remove obsolete migration command surfaces and legacy issue status code paths now
that issue lifecycle is driven by `.atelier/workflow.yaml`.

## Outcome

- Legacy workflow migration command paths and implementation helpers are removed
  from the current workflow code.
- Issue creation, listing, dependency, tree, and import paths use workflow
  statuses such as `todo`, `in_progress`, `validation`, and `done` instead of
  hard-coded `open` and `closed` issue states.
- Direct issue status updates and reopen shortcuts are removed in favor of
  `atelier issue transition`.
- Tests prove active workflow behavior without command-absence coverage for
  removed migrate commands.

## Evidence

- `rg -n "migrate-statuses|migrate_status|workflow migrate|record_status_changed|input.status|pub status: Option|issue update .*--status|--status open|--status closed|status: \"open\"|status: \"closed\"|Some\\(\"open\"\\)|Some\\(\"closed\"\\)" src tests AGENTFACTORY.md CONTEXT.md SPEC.md docs` leaves only mission-status/current-external-import references.
- `cargo fmt -- --check`
- `cargo build`
- Focused workflow integration tests:
  `test_workflow_help_is_scoped_as_advanced_internal_diagnostic`,
  `test_issue_transition_rejects_unmigrated_issue_status`,
  `test_workflow_check_rejects_legacy_issue_statuses_without_migration_path`,
  `test_issue_create_after_workflow_init_uses_configured_initial_status`,
  `test_workflow_check_reports_policy_and_issue_record_health`, and
  `test_workflow_check_rejects_issue_status_outside_selected_workflow`.
- Targeted module and smoke tests: `commands::tree::tests`, `db::tests`,
  `db::proptest_tests`, `models::tests`, `record_store::tests`,
  `commands::agent_factory::tests`, and
  `cargo test --test smoke_tests test_issue_tree_status_filter -- --nocapture`.
- `target/debug/atelier workflow check`
- `target/debug/atelier lint`
- `target/debug/atelier export --check`
- `target/debug/atelier doctor`
- `git diff --check`
