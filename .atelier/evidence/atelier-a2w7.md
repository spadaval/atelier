---
created_at: "2026-06-15T16:57:21.297105075+00:00"
id: "atelier-a2w7"
evidence_type: "validation"
captured_at: "2026-06-15T16:57:21.193929831+00:00"
command: "cargo check --manifest-path fuzz/Cargo.toml --bins"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7vfj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7vfj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Fuzz harnesses compile against atelier-sqlite ProjectionIndex and no longer import atelier::db::Database or atelier::models::Issue; rg old imports in fuzz returned no matches."
updated_at: "2026-06-15T16:57:23.697884214+00:00"
---

## Summary

Fuzz harnesses compile against atelier-sqlite ProjectionIndex and no longer import atelier::db::Database or atelier::models::Issue; rg old imports in fuzz returned no matches.

## Command

```console
cargo check --manifest-path fuzz/Cargo.toml --bins
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 72
Truncated: no

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```
