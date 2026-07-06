---
created_at: "2026-07-06T18:19:39.308977550+00:00"
id: "atelier-eoec"
evidence_type: "test"
captured_at: "2026-07-06T18:19:38.274312189+00:00"
command: "cargo nextest run -p atelier-sqlite"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-nxq9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nxq9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-sqlite"
updated_at: "2026-07-06T18:19:43.029212106+00:00"
---

## Summary

cargo nextest run -p atelier-sqlite

## Command

```console
cargo nextest run -p atelier-sqlite
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1419
Truncated: no

```text
   Compiling atelier-sqlite v0.2.0 (/root/.codex/worktrees/mska-ckca/crates/atelier-sqlite)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
────────────
 Nextest run ID fe5fef5e-f94a-40c2-9446-532ecb4aa807 with nextest profile: default
    Starting 9 tests across 1 binary
        PASS [   0.008s] (1/9) atelier-sqlite record_id::tests::validates_project_scoped_ids
        PASS [   0.009s] (2/9) atelier-sqlite record_id::tests::legacy_ids_are_project_scoped_base36
        PASS [   0.014s] (3/9) atelier-sqlite cache::tests::foreign_or_unversioned_layout_is_classified_for_rebuild
        PASS [   0.025s] (4/9) atelier-sqlite cache::tests::incompatible_cache_version_is_classified_without_migration
        PASS [   0.074s] (5/9) atelier-sqlite cache::tests::new_database_has_only_domain_tables_and_explicit_indexes
        PASS [   0.078s] (6/9) atelier-sqlite cache::tests::schema_has_no_body_or_generic_payload_columns
        PASS [   0.084s] (7/9) atelier-sqlite cache::tests::review_room_rows_add_replace_query_and_delete_atomically
        PASS [   0.090s] (8/9) atelier-sqlite cache::tests::evidence_rows_add_replace_reverse_query_and_delete_atomically
        PASS [   0.093s] (9/9) atelier-sqlite cache::tests::issue_rows_add_replace_query_and_delete_atomically
────────────
     Summary [   0.094s] 9 tests run: 9 passed, 0 skipped
```

