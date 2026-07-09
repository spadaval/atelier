---
created_at: "2026-07-07T06:14:07.506035895+00:00"
id: "atelier-r8uf"
evidence_type: "test"
captured_at: "2026-07-07T06:14:05.103095482+00:00"
command: "cargo test -p atelier-cli --test cli_integration test_work_missions_ -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sjsz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sjsz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission Overview CLI regression: hidden done/default empty accounting, explicit --all inclusion, linked epics, collapsed leaves, direct/unassigned work, quiet and NO_COLOR output, narrow width, and drill-down guidance"
updated_at: "2026-07-07T06:14:11.417349476+00:00"
---

## Summary

Mission Overview CLI regression: hidden done/default empty accounting, explicit --all inclusion, linked epics, collapsed leaves, direct/unassigned work, quiet and NO_COLOR output, narrow width, and drill-down guidance

## Command

```console
cargo test -p atelier-cli --test cli_integration test_work_missions_ -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 356
Truncated: no

```text

running 2 tests
test mission_cache_worktree::test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work ... ok
test mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 280 filtered out; finished in 0.45s
```

## Stderr

Bytes: 267
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-overview/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.89s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```
