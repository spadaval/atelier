---
created_at: "2026-06-17T23:57:34.158088077+00:00"
id: "atelier-u0md"
evidence_type: "validation"
captured_at: "2026-06-17T23:57:34.158078726+00:00"
agent_identity: "codex"
target:
  kind: "issue"
  id: "atelier-y31v"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-y31v"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Added canonical session record support: atelier-core SessionRecord/SessionRecordData/SessionTarget, canonical session record kind under .atelier/sessions, RecordStore create/load session APIs, session parse/render/validation, and bounded session activity summaries. Validation: cargo test -p atelier-records session; cargo check -p atelier-core; cargo check -p atelier-records; cargo check -p atelier-cli; cargo fmt -- --check; target/debug/atelier lint atelier-y31v; git diff --check."
updated_at: "2026-06-17T23:57:38.181664203+00:00"
---

Added canonical session record support: atelier-core SessionRecord/SessionRecordData/SessionTarget, canonical session record kind under .atelier/sessions, RecordStore create/load session APIs, session parse/render/validation, and bounded session activity summaries. Validation: cargo test -p atelier-records session; cargo check -p atelier-core; cargo check -p atelier-records; cargo check -p atelier-cli; cargo fmt -- --check; target/debug/atelier lint atelier-y31v; git diff --check.
