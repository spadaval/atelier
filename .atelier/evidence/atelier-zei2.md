---
created_at: "2026-06-11T23:41:53.896535296+00:00"
id: "atelier-zei2"
evidence_type: "test"
captured_at: "2026-06-11T23:41:53.896485370+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: null
agent_identity: "codex"
independence_level: null
follow_up_ids: []
residual_risks: []
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t0s4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-t0s4 validation: command entrypoints now use explicit storage access modes for projection query, canonical mutation, runtime-only, and health/repair; shared setup centralizes fresh projection checks for query and mutation paths. Passed: cargo fmt -- --check; cargo test command_storage_tests::access_modes_declare_projection_freshness_policy -- --nocapture; cargo test --no-run; atelier export --check; atelier lint; atelier doctor; atelier workflow validate issue atelier-t0s4."
updated_at: "2026-06-11T23:41:59.277377351+00:00"
---

atelier-t0s4 validation: command entrypoints now use explicit storage access modes for projection query, canonical mutation, runtime-only, and health/repair; shared setup centralizes fresh projection checks for query and mutation paths. Passed: cargo fmt -- --check; cargo test command_storage_tests::access_modes_declare_projection_freshness_policy -- --nocapture; cargo test --no-run; atelier export --check; atelier lint; atelier doctor; atelier workflow validate issue atelier-t0s4.
