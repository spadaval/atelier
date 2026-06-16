---
created_at: "2026-06-14T07:21:52.016697616+00:00"
id: "atelier-l816"
evidence_type: "validation"
captured_at: "2026-06-14T07:21:52.016589273+00:00"
target:
  kind: "issue"
  id: "atelier-liqk"
  role: "validates"
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
status: "recorded"
title: "Removed-command suggestions point to supported replacements without compatibility aliases. Proof: cargo nextest run --test cli_integration -E 'test(test_removed_commands_suggest_supported_replacements) or test(test_removed_aliases_fail_as_unknown_commands)'; cargo fmt -- --check; git diff --check; atelier lint; atelier lint atelier-liqk."
updated_at: "2026-06-14T07:21:54.433582366+00:00"
---

Removed-command suggestions point to supported replacements without compatibility aliases. Proof: cargo nextest run --test cli_integration -E 'test(test_removed_commands_suggest_supported_replacements) or test(test_removed_aliases_fail_as_unknown_commands)'; cargo fmt -- --check; git diff --check; atelier lint; atelier lint atelier-liqk.
