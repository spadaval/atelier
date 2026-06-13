---
created_at: "2026-06-13T00:46:46.993232860+00:00"
id: "atelier-jspo"
issue_type: "task"
labels:
- "tests"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T01:17:16.783421376+00:00"
status: "done"
title: "Repair success-close integration fixtures"
updated_at: "2026-06-13T01:17:16.783421376+00:00"
---

## Description

Repair broad-suite failures from active success-close fixtures that still use
bare `issue close` calls. These fixtures exercise list, tree, next, mutation,
dependency, smoke, mission-list, and stress flows where close is expected to
succeed, but current closeout rules require attached issue proof before the
status transition can complete.

## Outcome

- Active success-close integration fixtures attach valid proof before closing
  issues.
- The tests still verify readiness propagation, list/tree status filters,
  next-command progress, mutation durability, smoke lifecycle behavior,
  mission-list summaries, and rapid close/reopen behavior.
- The default broad-suite fail-fast probe advances past the previous
  dependency-chain failure.

## Evidence

- `cargo nextest run test_dependency_chain test_list_filter_by_status test_list_all_statuses test_tree_with_status_filter test_next_with_subissue_progress test_next_only_subissues_ready test_stress_rapid_operations test_issue_mutations_are_durable_without_manual_export test_issue_mutations_create_activity_sidecars test_issue_show_json_recovers_activity_fields_after_rebuild test_mission_list_human_overview_orders_and_summarizes test_mission_list_default_current_empty_state test_issue_tree_status_filter test_dependency_chain_and_ready` passes.
- `cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet` advances past the previous dependency-chain failure.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.
