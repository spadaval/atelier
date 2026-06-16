---
created_at: "2026-06-15T17:10:41.476136487+00:00"
id: "atelier-kr21"
evidence_type: "validation"
captured_at: "2026-06-15T17:10:37.787527962+00:00"
command: "cargo nextest run -p atelier-cli commands::import::tests::test_import_beads_fixture_preserves_counts_and_links test_work_lifecycle_human_output_and_guards"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yo9i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yo9i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "SQLite comments, sessions, and work_associations are removed from fresh schema, legacy migration drops them, rebuilt doctor reports only projection tables and no compatibility tables, Beads import preservation still passes, and worktree orientation works without runtime associations."
updated_at: "2026-06-15T17:12:13.189910036+00:00"
---

## Summary

SQLite comments, sessions, and work_associations are removed from fresh schema, legacy migration drops them, rebuilt doctor reports only projection tables and no compatibility tables, Beads import preservation still passes, and worktree orientation works without runtime associations.

## Command

```console
cargo nextest run -p atelier-cli commands::import::tests::test_import_beads_fixture_preserves_counts_and_links test_work_lifecycle_human_output_and_guards
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 641
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.29s
────────────
 Nextest run ID 136b6d2f-0989-4a5f-a09b-aac57cc081fc with nextest profile: default
    Starting 2 tests across 4 binaries (675 tests skipped)
        PASS [   0.049s] (1/2) atelier-cli commands::import::tests::test_import_beads_fixture_preserves_counts_and_links
        PASS [   1.102s] (2/2) atelier-cli::cli_integration test_work_lifecycle_human_output_and_guards
────────────
     Summary [   1.103s] 2 tests run: 2 passed, 675 skipped
```
