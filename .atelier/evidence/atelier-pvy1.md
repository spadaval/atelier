---
created_at: "2026-06-14T07:29:38.726743011+00:00"
id: "atelier-pvy1"
evidence_type: "validation"
captured_at: "2026-06-14T07:29:38.726702744+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-oqtz"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-oqtz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Focused stale projection and invalid canonical diagnostics passed: cargo test freshness_problem_messages_are_bounded_and_actionable; cargo test --test cli_integration test_projection_index_rejects_invalid_markdown_without_rebuild; cargo test --test cli_integration test_projection_query_distinguishes_schema_drift_from_malformed_records; cargo test --test cli_integration test_projection_index_bounds_many_changed_sources_and_rebuilds; cargo test --test smoke_tests test_canonical_export_check_cli; cargo fmt -- --check; git diff --check; atelier lint atelier-oqtz; atelier export --check; atelier doctor."
updated_at: "2026-06-14T07:29:40.696571599+00:00"
---

Focused stale projection and invalid canonical diagnostics passed: cargo test freshness_problem_messages_are_bounded_and_actionable; cargo test --test cli_integration test_projection_index_rejects_invalid_markdown_without_rebuild; cargo test --test cli_integration test_projection_query_distinguishes_schema_drift_from_malformed_records; cargo test --test cli_integration test_projection_index_bounds_many_changed_sources_and_rebuilds; cargo test --test smoke_tests test_canonical_export_check_cli; cargo fmt -- --check; git diff --check; atelier lint atelier-oqtz; atelier export --check; atelier doctor.
