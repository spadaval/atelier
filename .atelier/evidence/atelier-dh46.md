---
created_at: "2026-06-14T07:19:32.874677812+00:00"
id: "atelier-dh46"
evidence_type: "validation"
captured_at: "2026-06-14T07:19:32.874567829+00:00"
target:
  kind: "issue"
  id: "atelier-gsq1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-gsq1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Record-specific note commands are implemented for issue and mission activity. Help and docs show issue note and mission note, generic note add rejects with corrective guidance, and history reads issue and mission notes. Proof: cargo nextest run --test cli_integration focused note/help/history filters; cargo fmt -- --check; git diff --check; atelier lint; rg found no normal docs/tests references to --append-notes or atelier note add."
updated_at: "2026-06-14T07:19:35.413393342+00:00"
---

Record-specific note commands are implemented for issue and mission activity. Help and docs show issue note and mission note, generic note add rejects with corrective guidance, and history reads issue and mission notes. Proof: cargo nextest run --test cli_integration focused note/help/history filters; cargo fmt -- --check; git diff --check; atelier lint; rg found no normal docs/tests references to --append-notes or atelier note add.
