---
created_at: "2026-07-06T20:45:57.372295289+00:00"
id: "atelier-vj0b"
evidence_type: "test"
captured_at: "2026-07-06T20:45:51.117514235+00:00"
command: "cargo nextest run -p atelier-cli cache --status-level fail --final-status-level fail"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-m7za"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-m7za"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Focused cache regression suite"
updated_at: "2026-07-06T20:45:57.374922404+00:00"
---

## Summary

Focused cache regression suite

## Command

```console
cargo nextest run -p atelier-cli cache --status-level fail --final-status-level fail
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 438
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-xa9s/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.09s
────────────
 Nextest run ID bf475a37-8bff-473c-bcaf-0bcee1dcb753 with nextest profile: default
    Starting 67 tests across 4 binaries (381 tests skipped)
────────────
     Summary [   3.812s] 67 tests run: 67 passed, 381 skipped
```
