---
created_at: "2026-06-11T23:31:40.843179538+00:00"
id: "atelier-vo53"
evidence_type: "validation"
captured_at: "2026-06-11T23:31:40.843125362+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-pgkd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "One-shot markdown-first migration command validated: added atelier migrate markdown-first to move legacy .atelier-state canonical directories into .atelier, create .atelier/config.toml/root runtime ignore entries when missing, refresh projection while preserving runtime tables, and report next commands. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test commands::migrate::tests::markdown_first_moves_legacy_records_into_atelier -- --nocapture; target/debug/atelier migrate markdown-first reports already migrated in this checkout; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-pgkd."
updated_at: "2026-06-11T23:31:46.516482141+00:00"
---

One-shot markdown-first migration command validated: added atelier migrate markdown-first to move legacy .atelier-state canonical directories into .atelier, create .atelier/config.toml/root runtime ignore entries when missing, refresh projection while preserving runtime tables, and report next commands. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test commands::migrate::tests::markdown_first_moves_legacy_records_into_atelier -- --nocapture; target/debug/atelier migrate markdown-first reports already migrated in this checkout; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-pgkd.
