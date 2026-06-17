---
created_at: "2026-06-17T23:29:11.403798598+00:00"
id: "atelier-nufa"
evidence_type: "validation"
captured_at: "2026-06-17T23:29:11.403791158+00:00"
target:
  kind: "issue"
  id: "atelier-nm00"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nm00"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Recorded app/CLI boundary audit in docs/architecture/quality/app-cli-boundary-audit-2026-06-17.md and linked it from the quality index. Search transcript proves the live contradiction: crates/atelier-cli/src/main.rs and command modules still call command_storage/projection_query_db/canonical_mutation_db/degraded_projection_query_db/existing_projection_db, RecordStore::new, Database::open, and refresh_projection for orchestration; crates/atelier-app/src has no println!/eprintln! hits. The audit defines stricter future closeout proof and points to atelier-j75d, atelier-uro5, and atelier-wpht. Validation passed: target/debug/atelier lint atelier-nm00 and git diff --check."
updated_at: "2026-06-17T23:29:15.399620174+00:00"
---

Recorded app/CLI boundary audit in docs/architecture/quality/app-cli-boundary-audit-2026-06-17.md and linked it from the quality index. Search transcript proves the live contradiction: crates/atelier-cli/src/main.rs and command modules still call command_storage/projection_query_db/canonical_mutation_db/degraded_projection_query_db/existing_projection_db, RecordStore::new, Database::open, and refresh_projection for orchestration; crates/atelier-app/src has no println!/eprintln! hits. The audit defines stricter future closeout proof and points to atelier-j75d, atelier-uro5, and atelier-wpht. Validation passed: target/debug/atelier lint atelier-nm00 and git diff --check.
