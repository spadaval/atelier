---
created_at: "2026-06-11T23:48:56.021525275+00:00"
id: "atelier-9m68"
evidence_type: "test"
captured_at: "2026-06-11T23:48:56.021470109+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: null
agent_identity: "codex"
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cd1l"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-cd1l validation: projection query paths share the central freshness check and rebuild safely from changed/deleted/unindexed .atelier Markdown before issue and dependency queries; export no longer treats local .atelier runtime files as stale canonical output. Passed: cargo fmt -- --check; cargo test test_projection_index_rebuilds --test cli_integration -- --nocapture; cargo test --no-run; atelier export --check; atelier lint; atelier doctor; atelier workflow validate issue atelier-cd1l."
updated_at: "2026-06-11T23:49:01.405172294+00:00"
---

atelier-cd1l validation: projection query paths share the central freshness check and rebuild safely from changed/deleted/unindexed .atelier Markdown before issue and dependency queries; export no longer treats local .atelier runtime files as stale canonical output. Passed: cargo fmt -- --check; cargo test test_projection_index_rebuilds --test cli_integration -- --nocapture; cargo test --no-run; atelier export --check; atelier lint; atelier doctor; atelier workflow validate issue atelier-cd1l.
