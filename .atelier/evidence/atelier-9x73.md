---
created_at: "2026-07-06T18:48:39.915011205+00:00"
id: "atelier-9x73"
evidence_type: "test"
captured_at: "2026-07-06T18:47:22.997185880+00:00"
command: "env HOME=/tmp/atelier-vqhi-full-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run --profile extended --run-ignored=only"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Extended ignored SQLite property-test audit; four known nonblocking tests"
updated_at: "2026-07-06T18:48:45.923840505+00:00"
---

## Summary

Extended ignored SQLite property-test audit; four known nonblocking tests

## Command

```console
env HOME=/tmp/atelier-vqhi-full-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run --profile extended --run-ignored=only
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 833
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.92s
────────────
 Nextest run ID 88fd87a5-9f8f-4063-bec8-9245e935e60a with nextest profile: extended
    Starting 4 tests across 9 binaries (713 tests skipped)
        PASS [  18.752s] (1/4) atelier-sqlite proptest_tests::prop_extended_cascade_deletes_children
        PASS [  18.852s] (2/4) atelier-sqlite proptest_tests::prop_extended_no_circular_deps
        PASS [  20.258s] (3/4) atelier-sqlite proptest_tests::prop_extended_ready_list_correctness
        PASS [  16.820s] (4/4) atelier-sqlite proptest_tests::prop_extended_search_wildcards_escaped
────────────
     Summary [  74.684s] 4 tests run: 4 passed, 713 skipped
```

