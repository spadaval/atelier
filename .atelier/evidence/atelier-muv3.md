---
created_at: "2026-07-16T18:54:16.541340293+00:00"
id: "atelier-muv3"
evidence_type: "test"
captured_at: "2026-07-16T18:54:14.701151208+00:00"
command: "cargo nextest run -p atelier-app -p atelier-workflow -E 'test(/dependency_closure/) | test(parses_valid_policy)'"
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
title: "cargo nextest run -p atelier-app -p atelier-workflow -E 'test(/dependency_closure/) | test(parses_valid_policy)'"
updated_at: "2026-07-16T18:54:16.544470474+00:00"
---

## Summary

cargo nextest run -p atelier-app -p atelier-workflow -E 'test(/dependency_closure/) | test(parses_valid_policy)'

## Command

```console
cargo nextest run -p atelier-app -p atelier-workflow -E 'test(/dependency_closure/) | test(parses_valid_policy)'
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 774
Truncated: no

```text
   Compiling atelier-app v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.53s
────────────
 Nextest run ID f72032c4-e0ce-408a-bcfa-0fff105c0673 with nextest profile: default
    Starting 3 tests across 2 binaries (147 tests skipped)
        PASS [   0.009s] (1/3) atelier-app objective_graph::tests::dependency_closure_reports_complete_direct_and_transitive_paths
        PASS [   0.009s] (2/3) atelier-app objective_graph::tests::dependency_closure_fails_safely_with_an_actionable_cycle_path
        PASS [   0.012s] (3/3) atelier-workflow tests::parses_valid_policy
────────────
     Summary [   0.012s] 3 tests run: 3 passed, 147 skipped
```
