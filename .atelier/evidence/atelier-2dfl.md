---
created_at: "2026-06-16T18:44:51.322682296+00:00"
id: "atelier-2dfl"
evidence_type: "validation"
captured_at: "2026-06-16T18:44:46.461401474+00:00"
command: "bash -lc 'cargo build -p atelier-cli && target/debug/atelier lint atelier-kzfl && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-kzfl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kzfl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Build, tracker lint, doctor, formatting, and diff whitespace pass"
updated_at: "2026-06-16T18:44:55.170228117+00:00"
---

## Summary

Build, tracker lint, doctor, formatting, and diff whitespace pass

## Command

```console
bash -lc 'cargo build -p atelier-cli && target/debug/atelier lint atelier-kzfl && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'
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

Bytes: 139
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.66s
```
