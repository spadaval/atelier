---
created_at: "2026-06-12T23:54:57.951589202+00:00"
id: "atelier-8tx7"
data: "{\"captured_at\":\"2026-06-12T23:54:57.298202980+00:00\",\"command\":\"bash -lc 'atelier export --check && atelier lint && atelier doctor'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":0,\"summary\":\"\",\"truncated\":false},\"stdout\":{\"bytes\":704,\"summary\":\"Canonical export is current\\nState: /root/atelier/.atelier\\nLint passed.\\nDatabase: /root/atelier/.atelier/state.db\\nState: /root/atelier/.atelier\\nInstall health:\\n  config: ok\\n  ignored_runtime_paths: ok\\nProjection rebuild:\\n  state_dir: ok\\n  rebuild_ready: ok\\n  projection_fresh: ok\\n  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources\\nCache health:\\n  cache_dir: missing (optional)\\n  projection_metadata: ok\\nRuntime state:\\n  directory: ok\\n  database: ok\\n  local_tables: ok\\n  diagnostics: enabled\\nCompatibility:\\n  tables: comments\\nLegacy health:\\nconfig: ok\\ndatabase: ok\\nignore_rules: ok\\nprojection_fresh: ok\\nrebuild_ready: ok\\nruntime_state: ok\\nruntime_tables: ok\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-nzy1\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nzy1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Epic closeout health for nzy1: focused closeout proof matrix is attached separately; export check, lint, and doctor all pass on the current tracker state."
updated_at: "2026-06-12T23:54:58.886263002+00:00"
---

Epic closeout health for nzy1: focused closeout proof matrix is attached separately; export check, lint, and doctor all pass on the current tracker state.

Command: bash -lc 'atelier export --check && atelier lint && atelier doctor'
Exit status: 0

Stdout summary:
Canonical export is current
State: /root/atelier/.atelier
Lint passed.
Database: /root/atelier/.atelier/state.db
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
  tables: comments
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

