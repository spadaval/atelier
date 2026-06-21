---
created_at: "2026-06-10T22:22:35.183562484+00:00"
id: atelier-zr8x
evidence_type: test
captured_at: "2026-06-10T22:22:35.183373516+00:00"
agent_identity: codex
relationships:
  blocks: []
  children: []
  attachments:
  - kind: issue
    id: atelier-g28t
    role: validates
  relates: []
schema: atelier.evidence
schema_version: 1
status: recorded
title: "Final human CLI output validation passed: cargo nextest run; cargo nextest
  run --profile extended --run-ignored=only; cargo fmt -- --check; git diff --check;
  atelier export --check; atelier lint; atelier doctor."
updated_at: "2026-06-10T22:22:35.183562484+00:00"
---

Final human CLI output validation passed: cargo nextest run; cargo nextest run --profile extended --run-ignored=only; cargo fmt -- --check; git diff --check; atelier export --check; atelier lint; atelier doctor.
