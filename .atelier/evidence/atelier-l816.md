---
created_at: "2026-06-14T07:21:52.016697616+00:00"
id: "atelier-l816"
evidence_type: "validation"
captured_at: "2026-06-14T07:21:52.016589273+00:00"
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
title: "Removed-command suggestions point to supported replacements without compatibility aliases. Proof: cargo nextest run --test cli_integration -E 'test(test_removed_commands_suggest_supported_replacements) or test(test_removed_aliases_fail_as_unknown_commands)'; cargo fmt -- --check; git diff --check; atelier lint; atelier lint atelier-liqk."
updated_at: "2026-06-14T07:21:54.433582366+00:00"
---

Removed-command suggestions point to supported replacements without compatibility aliases. Proof: cargo nextest run --test cli_integration -E 'test(test_removed_commands_suggest_supported_replacements) or test(test_removed_aliases_fail_as_unknown_commands)'; cargo fmt -- --check; git diff --check; atelier lint; atelier lint atelier-liqk.
