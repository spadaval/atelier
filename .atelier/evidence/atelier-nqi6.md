---
created_at: "2026-06-14T07:12:02.230483490+00:00"
id: "atelier-nqi6"
evidence_type: "validation"
captured_at: "2026-06-14T07:12:02.230446022+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-liqk"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-liqk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Removed-command suggestion behavior validated: cargo test --test cli_integration test_removed_commands_suggest_supported_replacements -- --nocapture; cargo test --test cli_integration test_removed_aliases_fail_as_unknown_commands -- --nocapture; cargo fmt -- --check; git diff --check; atelier lint atelier-liqk; atelier lint."
updated_at: "2026-06-14T07:12:03.948277709+00:00"
---

Removed-command suggestion behavior validated: cargo test --test cli_integration test_removed_commands_suggest_supported_replacements -- --nocapture; cargo test --test cli_integration test_removed_aliases_fail_as_unknown_commands -- --nocapture; cargo fmt -- --check; git diff --check; atelier lint atelier-liqk; atelier lint.
