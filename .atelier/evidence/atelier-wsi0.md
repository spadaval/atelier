---
created_at: "2026-06-13T23:50:47.709738072+00:00"
id: "atelier-wsi0"
evidence_type: "validation"
captured_at: "2026-06-13T23:50:47.709629199+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ihz0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Validated typed DomainRecord/data_json reduction: cargo test record_store::tests --lib; cargo test test_first_class_records_export_rebuild_and_validate --test cli_integration; cargo fmt -- --check; git diff --check; target/debug/atelier rebuild; target/debug/atelier lint; target/debug/atelier export --check."
updated_at: "2026-06-13T23:50:50.153165819+00:00"
---

Validated typed DomainRecord/data_json reduction: cargo test record_store::tests --lib; cargo test test_first_class_records_export_rebuild_and_validate --test cli_integration; cargo fmt -- --check; git diff --check; target/debug/atelier rebuild; target/debug/atelier lint; target/debug/atelier export --check.
