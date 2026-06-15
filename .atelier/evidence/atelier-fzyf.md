---
created_at: "2026-06-15T16:49:08.479900555+00:00"
id: "atelier-fzyf"
evidence_type: "validation"
captured_at: "2026-06-15T16:49:08.479855445+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-nbni"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nbni"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Core extraction proof: atelier-core owns RecordId, RecordRelation, and pure non-empty value validation without filesystem, SQLite, Clap, or telemetry dependencies; crate-local tests record_id_rejects_empty_values, record_id_preserves_valid_text, and relation_requires_kind_and_role passed under cargo nextest before the broader stale-test failures."
updated_at: "2026-06-15T16:49:10.607035006+00:00"
---

Core extraction proof: atelier-core owns RecordId, RecordRelation, and pure non-empty value validation without filesystem, SQLite, Clap, or telemetry dependencies; crate-local tests record_id_rejects_empty_values, record_id_preserves_valid_text, and relation_requires_kind_and_role passed under cargo nextest before the broader stale-test failures.
