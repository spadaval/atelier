---
created_at: "2026-06-16T17:47:23.926320562+00:00"
id: "atelier-tr2b"
evidence_type: "test"
captured_at: "2026-06-16T17:47:21.255745873+00:00"
command: "bash -lc 'target/debug/atelier lint atelier-a7gd && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-a7gd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a7gd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "a7gd lint doctor formatting and whitespace checks pass"
updated_at: "2026-06-16T17:47:27.542321866+00:00"
---

## Summary

a7gd lint doctor formatting and whitespace checks pass

## Command

```console
bash -lc 'target/debug/atelier lint atelier-a7gd && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 646
Truncated: no

```text
Lint passed.
Database: /root/atelier/.atelier/runtime/state.db
State: /root/atelier/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: ok
Runtime state:
  directory: ok
  database: ok
  local_tables: ok
  diagnostics: enabled
Compatibility:
  tables: 
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok
```

## Stderr

Bytes: 0
Truncated: no

```text
```
