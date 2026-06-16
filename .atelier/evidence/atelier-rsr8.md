---
created_at: "2026-06-16T17:50:43.037292353+00:00"
id: "atelier-rsr8"
evidence_type: "test"
captured_at: "2026-06-16T17:50:40.178064332+00:00"
command: "bash -lc 'target/debug/atelier lint && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-m1r7"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 657
    summary: "Lint passed.\nDatabase: /root/atelier/.atelier/runtime/state.db\nState: /root/atelier/.atelier\nInstall health:\n  config: ok\n  ignored_runtime_paths: ok\nProjection rebuild:\n  state_dir: ok\n  rebuild_ready: ok\n  projection_fresh: not ok\n  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources\nCache health:\n  cache_dir: missing (optional)\n  projection_metadata: stale\nRuntime state:\n  directory: ok\n  database: ok\n  local_tables: ok\n  diagnostics: enabled\nCompatibility:\n  tables: \nLegacy health:\nconfig: ok\ndatabase: ok\nignore_rules: ok\nprojection_fresh: not ok\nrebuild_ready: ok\nruntime_state: ok\nruntime_tables: ok\n"
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
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
status: "pass"
title: "epic validation lint doctor fmt and whitespace pass"
updated_at: "2026-06-16T17:50:49.935082539+00:00"
---

epic validation lint doctor fmt and whitespace pass

Command: bash -lc 'target/debug/atelier lint && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'
Exit status: 0

Stdout summary:
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

Stderr summary:
(none)

