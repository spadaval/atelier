---
created_at: "2026-06-16T18:05:03.135623697+00:00"
id: "atelier-csj4"
evidence_type: "validation"
captured_at: "2026-06-16T18:05:02.192953941+00:00"
command: "target/debug/atelier doctor"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-kswx"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 633
    summary: "Database: /root/atelier/.atelier/runtime/state.db\nState: /root/atelier/.atelier\nInstall health:\n  config: ok\n  ignored_runtime_paths: ok\nProjection rebuild:\n  state_dir: ok\n  rebuild_ready: ok\n  projection_fresh: ok\n  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources\nCache health:\n  cache_dir: missing (optional)\n  projection_metadata: ok\nRuntime state:\n  directory: ok\n  database: ok\n  local_tables: ok\n  diagnostics: enabled\nCompatibility:\n  tables: \nLegacy health:\nconfig: ok\ndatabase: ok\nignore_rules: ok\nprojection_fresh: ok\nrebuild_ready: ok\nruntime_state: ok\nruntime_tables: ok\n"
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
    id: "atelier-kswx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Tracker doctor health passes after implementing shared ordering."
updated_at: "2026-06-16T18:05:06.663559580+00:00"
---

Tracker doctor health passes after implementing shared ordering.

Command: target/debug/atelier doctor
Exit status: 0

Stdout summary:
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

Stderr summary:
(none)

