---
created_at: "2026-06-18T20:33:44.241719959+00:00"
id: "atelier-918i"
issue_type: "task"
labels:
- "pr"
- "schema"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T21:06:00.953585252+00:00"
status: "done"
title: "Implement workflow configuration schema v3 simplification"
updated_at: "2026-06-18T21:06:00.953585252+00:00"
---

## Description

Implement schema version 3 workflow configuration simplification: workflows own
issue type applicability, branch policy is explicit, validators are inline, and
PR links use the built-in `pull_request` field.

## Outcome

The repository parses and uses schema v3 workflow policy only; normal
`atelier init` writes v3 starter config; canonical PR links store
`pull_request: <number>`; docs describe only the v3 contract; and the migrated
repository passes parser, PR, canonical-state, and health checks.

## Evidence

- Command output: `cargo test -p atelier-workflow --lib`
- Command output: `cargo test -p atelier-app -p atelier-records -p atelier-sqlite -p atelier-cli --lib`
- Command output: `cargo test -p atelier-cli --test cli_integration test_workflow_init_is_removed_and_root_init_owns_starter_policy`
- Command output: `cargo test -p atelier-cli --test cli_integration test_branch_lifecycle_context_surfaces_on_status_issue_transition_and_mission_status`
- Repository health checks: `atelier lint`, `atelier doctor`, `atelier export --check`, and `git diff --check`
