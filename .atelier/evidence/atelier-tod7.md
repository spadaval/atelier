---
created_at: "2026-06-15T16:49:16.835968361+00:00"
id: "atelier-tod7"
evidence_type: "validation"
captured_at: "2026-06-15T16:49:16.835923981+00:00"
target:
  kind: "issue"
  id: "atelier-5dgb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5dgb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "SQLite schema contract proof: docs/architecture/sqlite-runtime-schema.md defines rebuildable projection tables, runtime_metadata as local runtime table, one .atelier/runtime/state.db, rebuild as migration path, and excludes sessions/work_associations/hidden claims from target source-of-truth; atelier-sqlite tests schema_tables_have_explicit_ownership and removed_tables_are_not_part_of_target_schema passed; atelier lint and export --check passed."
updated_at: "2026-06-15T16:49:19.102718989+00:00"
---

SQLite schema contract proof: docs/architecture/sqlite-runtime-schema.md defines rebuildable projection tables, runtime_metadata as local runtime table, one .atelier/runtime/state.db, rebuild as migration path, and excludes sessions/work_associations/hidden claims from target source-of-truth; atelier-sqlite tests schema_tables_have_explicit_ownership and removed_tables_are_not_part_of_target_schema passed; atelier lint and export --check passed.
