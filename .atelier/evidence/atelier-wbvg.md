---
created_at: "2026-06-16T17:56:09.918683249+00:00"
id: "atelier-wbvg"
evidence_type: "test"
captured_at: "2026-06-16T17:56:07.143907049+00:00"
command: "bash -lc 'target/debug/atelier lint atelier-m1r7 && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-m1r7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-m1r7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "post-cleanup lint doctor fmt and whitespace pass"
updated_at: "2026-06-16T17:56:19.345723173+00:00"
---

## Summary

post-cleanup lint doctor fmt and whitespace pass

## Command

```console
bash -lc 'target/debug/atelier lint atelier-m1r7 && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 657
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
  projection_fresh: not ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: stale
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
projection_fresh: not ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok
```

## Stderr

Bytes: 0
Truncated: no

```text
```
