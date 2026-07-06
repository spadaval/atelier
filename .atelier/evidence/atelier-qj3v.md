---
created_at: "2026-07-06T21:36:24.772739527+00:00"
id: "atelier-qj3v"
evidence_type: "test"
captured_at: "2026-07-06T21:36:22.244543277+00:00"
command: "cargo nextest run -p atelier-cli -E 'test(test_import_beads_late_invalid_record_is_failure_atomic_and_retryable) or test(import::tests)'"
exit_status: "0"
agent_identity: "agent-factory-implementer"
target:
  kind: "issue"
  id: "atelier-mska"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mska"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS: resolves the Beads import half of failing review evidence atelier-umyf. A late invalid second record and a forced late activity-stage failure leave no imported record/activity files or staging artifacts, and clean retry succeeds; successful import remains record-first with lazy cache repair. Portable filesystems lack atomic directory exchange: in-process install failures roll back, while a process/filesystem crash between backup and install renames can leave the ignored backup for recovery."
updated_at: "2026-07-06T21:36:29.080371072+00:00"
---

## Summary

PASS: resolves the Beads import half of failing review evidence atelier-umyf. A late invalid second record and a forced late activity-stage failure leave no imported record/activity files or staging artifacts, and clean retry succeeds; successful import remains record-first with lazy cache repair. Portable filesystems lack atomic directory exchange: in-process install failures roll back, while a process/filesystem crash between backup and install renames can leave the ignored backup for recovery.

## Command

```console
cargo nextest run -p atelier-cli -E 'test(test_import_beads_late_invalid_record_is_failure_atomic_and_retryable) or test(import::tests)'
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1378
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-import-atomic/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.07s
────────────
 Nextest run ID c77e797e-f3e1-4505-864f-3481f970d8bc with nextest profile: default
    Starting 7 tests across 4 binaries (445 tests skipped)
        PASS [   0.011s] (1/7) atelier-cli commands::import::tests::test_import_beads_fixture_preserves_counts_and_links
        PASS [   0.011s] (2/7) atelier-cli commands::import::tests::test_import_beads_preserves_notes_as_activity_records
        PASS [   0.012s] (3/7) atelier-cli commands::import::tests::test_import_staging_failure_leaves_no_records_or_activities_and_allows_retry
        PASS [   0.013s] (4/7) atelier-cli commands::import::tests::test_imported_beads_description_uses_current_issue_sections
        PASS [   0.025s] (5/7) atelier-cli commands::import::tests::test_import_rejects_late_invalid_record_without_partial_files_and_allows_retry
        PASS [   0.094s] (6/7) atelier-cli::cli_integration issues::test_import_beads_late_invalid_record_is_failure_atomic_and_retryable
        PASS [   0.124s] (7/7) atelier-cli commands::import::tests::test_import_writes_records_without_cache_then_next_query_repairs
────────────
     Summary [   0.125s] 7 tests run: 7 passed, 445 skipped
```
