---
created_at: "2026-07-06T18:19:26.082557868+00:00"
id: "atelier-2pkp"
evidence_type: "test"
captured_at: "2026-07-06T18:19:21.964932122+00:00"
command: "cargo nextest run -p atelier-cli test_cache_"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-2jse"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2jse"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "CLI cache dispatch repairs stale sources through the new boundary"
updated_at: "2026-07-06T18:19:30.233537357+00:00"
---

## Summary

CLI cache dispatch repairs stale sources through the new boundary

## Command

```console
cargo nextest run -p atelier-cli test_cache_
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1034
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-x7lq/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.03s
────────────
 Nextest run ID f392adc5-b254-44b7-9376-5239a13a0f8b with nextest profile: default
    Starting 4 tests across 4 binaries (448 tests skipped)
        PASS [   0.478s] (1/4) atelier-cli::cli_integration mission_projection_worktree::test_cache_rebuilds_changed_sources_before_issue_queries
        PASS [   0.736s] (2/4) atelier-cli::cli_integration mission_projection_worktree::test_cache_repairs_deleted_and_unindexed_sources_before_issue_queries
        PASS [   0.932s] (3/4) atelier-cli::cli_integration mission_projection_worktree::test_cache_rebuilds_dep_list_and_lint_but_ignores_derived_files
        PASS [   1.785s] (4/4) atelier-cli::cli_integration mission_projection_worktree::test_cache_bounds_many_changed_sources_and_rebuilds
────────────
     Summary [   1.786s] 4 tests run: 4 passed, 448 skipped
```

