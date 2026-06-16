---
created_at: "2026-06-16T17:56:09.486109290+00:00"
id: "atelier-27ng"
evidence_type: "test"
captured_at: "2026-06-16T17:56:06.951276062+00:00"
command: "cargo nextest run --cargo-quiet root_help role_guide export_check doctor --no-tests=pass"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-m1r7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-m1r7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "independent validator failing probe now passes"
updated_at: "2026-06-16T17:56:13.718998079+00:00"
---

## Summary

independent validator failing probe now passes

## Command

```console
cargo nextest run --cargo-quiet root_help role_guide export_check doctor --no-tests=pass
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1347
Truncated: no

```text
────────────
 Nextest run ID edf1dc38-95fa-4e13-bdc8-f61adeeb6be0 with nextest profile: default
    Starting 8 tests across 9 binaries (704 tests skipped)
        PASS [   0.014s] (1/8) atelier-cli::cli_integration setup_guidance::test_doctor_help_documents_fix_boundary
        PASS [   0.166s] (2/8) atelier-cli::cli_integration setup_guidance::test_doctor_reports_runtime_health_without_becoming_canonical_lint
        PASS [   0.172s] (3/8) atelier-cli::cli_integration setup_guidance::test_doctor_fix_refuses_to_modify_malformed_canonical_records
        PASS [   0.176s] (4/8) atelier-cli::cli_integration setup_guidance::test_doctor_human_separates_projection_and_runtime_state_health
        PASS [   0.223s] (5/8) atelier-cli::cli_integration setup_guidance::test_doctor_distinguishes_missing_runtime_projection_database
        PASS [   0.260s] (6/8) atelier-cli::smoke_tests smoke::cli_data::test_canonical_export_check_cli
        PASS [   0.419s] (7/8) atelier-cli::cli_integration setup_guidance::test_doctor_fix_repairs_missing_and_stale_local_projection_state
        PASS [   0.443s] (8/8) atelier-cli::cli_integration mission_projection_worktree::test_rebuild_temp_files_are_ignored_by_query_lint_and_doctor
────────────
     Summary [   0.444s] 8 tests run: 8 passed, 704 skipped
```
