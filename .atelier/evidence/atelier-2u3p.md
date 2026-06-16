---
created_at: "2026-06-16T17:58:04.597374166+00:00"
id: "atelier-2u3p"
evidence_type: "test"
captured_at: "2026-06-16T17:57:58.470299315+00:00"
command: "bash -lc 'target/debug/atelier lint && target/debug/atelier doctor && cargo nextest run --cargo-quiet root_help role_guide export_check doctor --no-tests=pass && cargo fmt -- --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-c4b8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c4b8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "command-surface epic final checks pass"
updated_at: "2026-06-16T17:58:09.170919543+00:00"
---

## Summary

command-surface epic final checks pass

## Command

```console
bash -lc 'target/debug/atelier lint && target/debug/atelier doctor && cargo nextest run --cargo-quiet root_help role_guide export_check doctor --no-tests=pass && cargo fmt -- --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 657
Truncated: no

```text
Lint passed.
Database: /root/atelier/.atelier/runtime/state.db
State: /root/atelier/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: not ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: stale
Runtime state:
  directory: ok
  database: ok
  local_tables: ok
  diagnostics: enabled
Compatibility:
  tables: 
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: not ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok
```

## Stderr

Bytes: 1347
Truncated: no

```text
────────────
 Nextest run ID 5babdb60-763f-4c74-9d22-333c5c5d46e0 with nextest profile: default
    Starting 8 tests across 9 binaries (704 tests skipped)
        PASS [   0.017s] (1/8) atelier-cli::cli_integration setup_guidance::test_doctor_help_documents_fix_boundary
        PASS [   0.165s] (2/8) atelier-cli::cli_integration setup_guidance::test_doctor_reports_runtime_health_without_becoming_canonical_lint
        PASS [   0.170s] (3/8) atelier-cli::cli_integration setup_guidance::test_doctor_human_separates_projection_and_runtime_state_health
        PASS [   0.173s] (4/8) atelier-cli::cli_integration setup_guidance::test_doctor_fix_refuses_to_modify_malformed_canonical_records
        PASS [   0.221s] (5/8) atelier-cli::cli_integration setup_guidance::test_doctor_distinguishes_missing_runtime_projection_database
        PASS [   0.269s] (6/8) atelier-cli::smoke_tests smoke::cli_data::test_canonical_export_check_cli
        PASS [   0.396s] (7/8) atelier-cli::cli_integration mission_projection_worktree::test_rebuild_temp_files_are_ignored_by_query_lint_and_doctor
        PASS [   0.689s] (8/8) atelier-cli::cli_integration setup_guidance::test_doctor_fix_repairs_missing_and_stale_local_projection_state
────────────
     Summary [   0.691s] 8 tests run: 8 passed, 704 skipped
```
