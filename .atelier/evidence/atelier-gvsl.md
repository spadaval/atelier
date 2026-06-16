---
created_at: "2026-06-16T17:43:41.975426334+00:00"
id: "atelier-gvsl"
evidence_type: "test"
captured_at: "2026-06-16T17:43:39.869631583+00:00"
command: "bash -lc 'target/debug/atelier lint atelier-jezn && target/debug/atelier doctor && git diff --check && git -C /root/.agents diff --check -- skills/agent-factory'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-jezn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jezn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "jezn lint doctor and whitespace checks pass"
updated_at: "2026-06-16T17:43:51.809966166+00:00"
---

## Summary

jezn lint doctor and whitespace checks pass

## Command

```console
bash -lc 'target/debug/atelier lint atelier-jezn && target/debug/atelier doctor && git diff --check && git -C /root/.agents diff --check -- skills/agent-factory'
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
