---
created_at: "2026-06-21T18:55:28.687687369+00:00"
id: "atelier-iope"
evidence_type: "validation"
captured_at: "2026-06-21T18:55:21.042442205+00:00"
command: "bash -lc 'cargo fmt -- --check && git diff --check && target/debug/atelier lint && cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && cargo nextest run -p atelier-workflow rejects_unknown_inline_validator rejects_obsolete_flat_validator_names parses_custom_issue_type_registry'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-m2ql"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-m2ql"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo fmt -- --check && git diff --check && target/debug/atelier lint && cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && cargo nextest run -p atelier-workflow rejects_unknown_inline_validator rejects_obsolete_flat_validator_names parses_custom_issue_type_registry'"
updated_at: "2026-06-21T18:55:33.732174558+00:00"
---

## Summary

bash -lc 'cargo fmt -- --check && git diff --check && target/debug/atelier lint && cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && cargo nextest run -p atelier-workflow rejects_unknown_inline_validator rejects_obsolete_flat_validator_names parses_custom_issue_type_registry'

## Command

```console
bash -lc 'cargo fmt -- --check && git diff --check && target/debug/atelier lint && cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && cargo nextest run -p atelier-workflow rejects_unknown_inline_validator rejects_obsolete_flat_validator_names parses_custom_issue_type_registry'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 1612
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-ncq9/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.01s
────────────
 Nextest run ID 46f48ab7-93d8-4e7c-a8c8-418d962427b7 with nextest profile: default
    Starting 4 tests across 4 binaries (441 tests skipped)
        PASS [   0.139s] (1/4) atelier-cli::cli_integration issues::test_issue_create_mission_type_requires_workflow_policy_declaration
        PASS [   0.332s] (2/4) atelier-cli::cli_integration issues::test_issue_create_mission_type_uses_declared_workflow_policy
        PASS [   0.835s] (3/4) atelier-cli::cli_integration mission_projection_worktree::test_mission_terminal_status_and_options_use_configured_objective_validators
        PASS [   2.669s] (4/4) atelier-cli::cli_integration mission_projection_worktree::test_mission_close_uses_configured_objective_validators
────────────
     Summary [   2.670s] 4 tests run: 4 passed, 441 skipped
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
────────────
 Nextest run ID f2bfda9d-e336-49e6-bd22-5afd0548f304 with nextest profile: default
    Starting 3 tests across 1 binary (28 tests skipped)
        PASS [   0.011s] (1/3) atelier-workflow tests::parses_custom_issue_type_registry
        PASS [   0.015s] (2/3) atelier-workflow tests::rejects_unknown_inline_validator
        PASS [   0.036s] (3/3) atelier-workflow tests::rejects_obsolete_flat_validator_names
────────────
     Summary [   0.037s] 3 tests run: 3 passed, 28 skipped
```

