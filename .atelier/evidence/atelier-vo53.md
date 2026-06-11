---
created_at: "2026-06-11T23:31:40.843179538+00:00"
id: "atelier-vo53"
data: "{\"captured_at\":\"2026-06-11T23:31:40.843125362+00:00\",\"kind\":\"validation\",\"path\":null,\"producer\":null,\"result\":\"pass\",\"uri\":null}"
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
