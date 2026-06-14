---
created_at: "2026-06-14T07:48:56.552327398+00:00"
id: "atelier-x52c"
evidence_type: "validation"
captured_at: "2026-06-14T07:48:56.552294284+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-vj08"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vj08"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Implemented atelier init --import-beads for .beads/issues.manual.jsonl, kept standalone import-beads hidden from root help but callable, and updated docs/help/tests. Passing proof: cargo fmt -- --check; git diff --check; atelier lint; cargo test --test cli_integration test_init_help_documents_import_beads_flag; cargo test --test cli_integration test_init_import_beads_requires_explicit_flag; cargo test --test cli_integration test_top_level_help_only_shows_core_commands; cargo test --test cli_integration test_import_beads_jsonl_fixture_round_trip; cargo test --test cli_integration test_import_beads_reports_mapping_without_tracker_provenance; cargo test --test cli_integration test_workflow_check_reports_policy_and_issue_record_health."
updated_at: "2026-06-14T07:48:58.482231963+00:00"
---

Implemented atelier init --import-beads for .beads/issues.manual.jsonl, kept standalone import-beads hidden from root help but callable, and updated docs/help/tests. Passing proof: cargo fmt -- --check; git diff --check; atelier lint; cargo test --test cli_integration test_init_help_documents_import_beads_flag; cargo test --test cli_integration test_init_import_beads_requires_explicit_flag; cargo test --test cli_integration test_top_level_help_only_shows_core_commands; cargo test --test cli_integration test_import_beads_jsonl_fixture_round_trip; cargo test --test cli_integration test_import_beads_reports_mapping_without_tracker_provenance; cargo test --test cli_integration test_workflow_check_reports_policy_and_issue_record_health.
