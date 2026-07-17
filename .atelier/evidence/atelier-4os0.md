---
created_at: "2026-07-17T00:13:47.286883282+00:00"
id: "atelier-4os0"
evidence_type: "test"
captured_at: "2026-07-17T00:13:44.628477212+00:00"
command: "cargo nextest run --profile extended --run-ignored=only"
exit_status: "4"
target:
  kind: "issue"
  id: "atelier-t876"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run --profile extended --run-ignored=only"
updated_at: "2026-07-17T00:13:47.288691375+00:00"
---

## Summary

cargo nextest run --profile extended --run-ignored=only

## Command

```console
cargo nextest run --profile extended --run-ignored=only
```

Exit status: 4

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 501
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p4z2/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.26s
────────────
 Nextest run ID c3eea533-cdce-4093-8d10-f9d06310a1ab with nextest profile: extended
    Starting 0 tests across 9 binaries (781 tests skipped)
────────────
     Summary [   0.002s] 0 tests run: 0 passed, 781 skipped
error: no tests to run
(hint: use `--no-tests` to customize)
```

