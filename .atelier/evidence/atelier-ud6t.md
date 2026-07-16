---
created_at: "2026-07-07T06:00:13.693111910+00:00"
id: "atelier-ud6t"
evidence_type: "test"
captured_at: "2026-07-07T06:00:11.237836364+00:00"
command: "cargo test -p atelier-cli --test cli_integration mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output -- --exact --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-dy3u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4fip"
    role: "validates"
  - kind: "issue"
    id: "atelier-dy3u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Response to atelier-lzxi: shared COLUMNS-aware row rendering keeps Mission Overview hierarchy, state, priority, title, progress, drill-downs, and exceptional accounting within 40 columns while preserving wide and NO_COLOR output"
updated_at: "2026-07-07T06:00:31.512772786+00:00"
---

## Summary

Response to atelier-lzxi: shared COLUMNS-aware row rendering keeps Mission Overview hierarchy, state, priority, title, progress, drill-downs, and exceptional accounting within 40 columns while preserving wide and NO_COLOR output

## Command

```console
cargo test -p atelier-cli --test cli_integration mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output -- --exact --nocapture
```

Exit status: 0

## Stdout

Bytes: 233
Truncated: no

```text

running 1 test
test mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 279 filtered out; finished in 0.47s
```

## Stderr

Bytes: 267
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-overview/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.92s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```
