---
created_at: "2026-07-06T20:28:51.349086369+00:00"
id: "atelier-y1zi"
evidence_type: "test"
captured_at: "2026-07-06T20:28:51.158719689+00:00"
command: "bash -lc 'target/debug/atelier check --help | rg \"runtime/cache state; never edits tracked record files\" && target/debug/atelier rebuild --help | rg \"Advanced domain-cache diagnostic\" && target/debug/atelier export --help | rg \"renderer/cache freshness\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-u7wi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u7wi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'target/debug/atelier check --help | rg \"runtime/cache state; never edits tracked record files\" && target/debug/atelier rebuild --help | rg \"Advanced domain-cache diagnostic\" && target/debug/atelier export --help | rg \"renderer/cache freshness\"'"
updated_at: "2026-07-06T20:28:56.022795445+00:00"
---

## Summary

bash -lc 'target/debug/atelier check --help | rg "runtime/cache state; never edits tracked record files" && target/debug/atelier rebuild --help | rg "Advanced domain-cache diagnostic" && target/debug/atelier export --help | rg "renderer/cache freshness"'

## Command

```console
bash -lc 'target/debug/atelier check --help | rg "runtime/cache state; never edits tracked record files" && target/debug/atelier rebuild --help | rg "Advanced domain-cache diagnostic" && target/debug/atelier export --help | rg "renderer/cache freshness"'
```

Exit status: 0

## Stdout

Bytes: 291
Truncated: no

```text
      --fix                      Repair ignored local runtime/cache state; never edits tracked record files
Advanced domain-cache diagnostic; explicit local repair uses check --fix
      --check                    Check deterministic renderer/cache freshness without writing tracked records
```

## Stderr

Bytes: 0
Truncated: no

```text
```

