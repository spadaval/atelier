---
created_at: "2026-07-06T20:46:09.876677366+00:00"
id: "atelier-oy2z"
evidence_type: "test"
captured_at: "2026-07-06T20:45:57.564834551+00:00"
command: "cargo nextest run -p atelier-cli --status-level fail --final-status-level fail"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-qqfe"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qqfe"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Full Atelier CLI command regression suite"
updated_at: "2026-07-06T20:46:09.879108765+00:00"
---

## Summary

Full Atelier CLI command regression suite

## Command

```console
cargo nextest run -p atelier-cli --status-level fail --final-status-level fail
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 419
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-xa9s/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.34s
────────────
 Nextest run ID e4d7aa79-cc6d-442a-920d-f5058f19ce86 with nextest profile: default
    Starting 448 tests across 4 binaries
────────────
     Summary [   9.647s] 448 tests run: 448 passed, 0 skipped
```

