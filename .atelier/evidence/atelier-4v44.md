---
created_at: "2026-06-19T05:11:33.820134535+00:00"
id: "atelier-4v44"
evidence_type: "validation"
captured_at: "2026-06-19T05:11:33.820133002+00:00"
target:
  kind: "issue"
  id: "atelier-swxv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-swxv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Provider mode parity and wrong-mode rejection were validated. Provider-backed behavior is covered by cargo test -p atelier-app pr::tests --lib and cargo test -p atelier-cli pr --lib. Installed-binary checks prove atelier pr is rejected as an unknown command, provider mode rejects room-only review resolve with review_mode_invalid, and room mode rejects provider-only review link with review_mode_invalid."
updated_at: "2026-06-19T05:11:36.457948834+00:00"
---

Provider mode parity and wrong-mode rejection were validated. Provider-backed behavior is covered by cargo test -p atelier-app pr::tests --lib and cargo test -p atelier-cli pr --lib. Installed-binary checks prove atelier pr is rejected as an unknown command, provider mode rejects room-only review resolve with review_mode_invalid, and room mode rejects provider-only review link with review_mode_invalid.
