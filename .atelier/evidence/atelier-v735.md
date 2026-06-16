---
created_at: "2026-06-16T18:38:35.810241418+00:00"
id: "atelier-v735"
evidence_type: "validation"
captured_at: "2026-06-16T18:38:33.730805009+00:00"
command: "bash -lc 'cargo fmt -- --check && git diff --check && target/debug/atelier doctor'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-qh52"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qh52"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Formatting, diff whitespace, and tracker doctor checks pass"
updated_at: "2026-06-16T18:38:39.652539131+00:00"
---

## Summary

Formatting, diff whitespace, and tracker doctor checks pass

## Command

```console
bash -lc 'cargo fmt -- --check && git diff --check && target/debug/atelier doctor'
```

Exit status: 0

## Stdout

Bytes: 633
Truncated: no

```text
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
