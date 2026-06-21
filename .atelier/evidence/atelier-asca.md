---
created_at: "2026-06-21T18:42:48.856698146+00:00"
id: "atelier-asca"
evidence_type: "test"
captured_at: "2026-06-21T18:42:46.394521694+00:00"
command: "cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-s43l"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-s43l"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy"
updated_at: "2026-06-21T18:42:51.754288473+00:00"
---

## Summary

cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy

## Command

```console
cargo nextest run -p atelier-cli test_issue_create_mission_type_requires_workflow_policy_declaration test_issue_create_mission_type_uses_declared_workflow_policy
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 686
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-ncq9/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.89s
────────────
 Nextest run ID 7d141a44-9ef1-4071-9b94-0cb8e82fc3c4 with nextest profile: default
    Starting 2 tests across 4 binaries (441 tests skipped)
        PASS [   0.107s] (1/2) atelier-cli::cli_integration issues::test_issue_create_mission_type_requires_workflow_policy_declaration
        PASS [   0.291s] (2/2) atelier-cli::cli_integration issues::test_issue_create_mission_type_uses_declared_workflow_policy
────────────
     Summary [   0.292s] 2 tests run: 2 passed, 441 skipped
```

