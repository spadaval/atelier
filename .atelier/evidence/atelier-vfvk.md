---
created_at: "2026-07-16T18:54:28.555983448+00:00"
id: "atelier-vfvk"
evidence_type: "test"
captured_at: "2026-07-16T18:54:24.440786309+00:00"
command: "cargo nextest run -p atelier-cli --test cli_integration -E 'test(/dependency_closure/)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-amfw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-amfw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli --test cli_integration -E 'test(/dependency_closure/)'"
updated_at: "2026-07-16T18:54:28.558302795+00:00"
---

## Summary

cargo nextest run -p atelier-cli --test cli_integration -E 'test(/dependency_closure/)'

## Command

```console
cargo nextest run -p atelier-cli --test cli_integration -E 'test(/dependency_closure/)'
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 751
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.89s
────────────
 Nextest run ID 84841d36-032b-46c4-a3c9-f8069c3c070b with nextest profile: default
    Starting 2 tests across 1 binary (297 tests skipped)
        PASS [   0.679s] (1/2) atelier-cli::cli_integration mission_cache_worktree::test_issue_ready_work_and_direct_start_require_transitive_dependency_closure
        PASS [   1.928s] (2/2) atelier-cli::cli_integration mission_cache_worktree::test_mission_start_requires_cycle_safe_transitive_dependency_closure
────────────
     Summary [   1.929s] 2 tests run: 2 passed, 297 skipped
```
