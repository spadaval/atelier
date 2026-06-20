---
created_at: "2026-06-20T00:01:43.118680657+00:00"
id: "atelier-jif3"
evidence_type: "validation"
captured_at: "2026-06-20T00:01:43.118679310+00:00"
target:
  kind: "issue"
  id: "atelier-zu0t"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zu0t"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed compatibility_state_root and committed runtime/cache/database path settings from tracked config and init template; project config parser now accepts only [paths].state_root and rejects committed runtime/cache/database or compatibility path keys; fresh init still creates .atelier/runtime/state.db from StorageLayout defaults while generated .atelier/config.toml omits those keys. Validation passed: cargo fmt -- --check; cargo test -p atelier-app project_config --lib; cargo test -p atelier-cli commands::init --lib; cargo nextest run test_init_creates_atelier_directory; target/debug/atelier lint; atelier lint atelier-zu0t; git diff --check."
updated_at: "2026-06-20T00:01:45.881211453+00:00"
---

Removed compatibility_state_root and committed runtime/cache/database path settings from tracked config and init template; project config parser now accepts only [paths].state_root and rejects committed runtime/cache/database or compatibility path keys; fresh init still creates .atelier/runtime/state.db from StorageLayout defaults while generated .atelier/config.toml omits those keys. Validation passed: cargo fmt -- --check; cargo test -p atelier-app project_config --lib; cargo test -p atelier-cli commands::init --lib; cargo nextest run test_init_creates_atelier_directory; target/debug/atelier lint; atelier lint atelier-zu0t; git diff --check.
