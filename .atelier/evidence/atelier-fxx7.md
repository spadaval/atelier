---
created_at: "2026-06-13T23:33:23.764932848+00:00"
id: "atelier-fxx7"
evidence_type: "validation"
captured_at: "2026-06-13T23:33:23.764835021+00:00"
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
    id: "atelier-nqp4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Implemented scoped issue body ownership fix. Proof: cargo fmt -- --check; cargo nextest run --cargo-quiet test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild test_update_description_flag_is_removed test_issue_show_reads_detail_body_from_record_store record_store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays (5 passed); target/debug/atelier rebuild; target/debug/atelier issue update --help with no --description; rg residue classified; target/debug/atelier lint atelier-nqp4; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check."
updated_at: "2026-06-13T23:33:25.904388903+00:00"
---

Implemented scoped issue body ownership fix. Proof: cargo fmt -- --check; cargo nextest run --cargo-quiet test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild test_update_description_flag_is_removed test_issue_show_reads_detail_body_from_record_store record_store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays (5 passed); target/debug/atelier rebuild; target/debug/atelier issue update --help with no --description; rg residue classified; target/debug/atelier lint atelier-nqp4; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check.
