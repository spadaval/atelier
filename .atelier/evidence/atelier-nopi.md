---
created_at: "2026-06-19T04:38:17.456955970+00:00"
id: "atelier-nopi"
evidence_type: "validation"
captured_at: "2026-06-19T04:38:17.456954694+00:00"
target:
  kind: "issue"
  id: "atelier-j1i1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-j1i1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Canonical review room YAML records implemented. Record kind review is first-class under .atelier/reviews/<id>.yaml, renders plain YAML, parses room metadata plus ordered events, allocates IDs across canonical dirs, and rebuild indexes review records with the correct .yaml extension. Proof: cargo test -p atelier-records --lib passed; cargo test -p atelier-app rebuild --lib passed; atelier lint atelier-j1i1 passed; git diff --check passed."
updated_at: "2026-06-19T04:38:20.244264030+00:00"
---

Canonical review room YAML records implemented. Record kind review is first-class under .atelier/reviews/<id>.yaml, renders plain YAML, parses room metadata plus ordered events, allocates IDs across canonical dirs, and rebuild indexes review records with the correct .yaml extension. Proof: cargo test -p atelier-records --lib passed; cargo test -p atelier-app rebuild --lib passed; atelier lint atelier-j1i1 passed; git diff --check passed.
