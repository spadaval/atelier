---
created_at: "2026-06-16T18:24:37.251545280+00:00"
id: "atelier-4zck"
evidence_type: "validation"
captured_at: "2026-06-16T18:24:36.245686710+00:00"
command: "target/debug/atelier doctor"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-f7vd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-f7vd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Tracker doctor health passes after mission status ordering changes."
updated_at: "2026-06-16T18:24:40.919991776+00:00"
---

## Summary

Tracker doctor health passes after mission status ordering changes.

## Command

```console
target/debug/atelier doctor
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
