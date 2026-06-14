---
created_at: "2026-06-11T23:22:07.354752088+00:00"
id: "atelier-i3no"
evidence_type: "validation"
captured_at: "2026-06-11T23:22:07.354698765+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3es3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Storage layout boundary validated: introduced StorageLayout for canonical .atelier/.atelier-state discovery and runtime db resolution; command dispatch, workflow, work, telemetry, init, doctor, activity/evidence/mission/plan paths now route through layout helpers; rg scan confirmed remaining .atelier-state/.atelier state.db references are tests, display strings, or layout constants. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test rebuild_round_trips_canonical_issue_state -- --nocapture; cargo test --test cli_integration test_first_class_records_export_rebuild_and_validate -- --nocapture; cargo test --test cli_integration test_projection_index_rebuilds_changed_sources_before_issue_queries -- --nocapture; git diff --check; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-3es3."
updated_at: "2026-06-11T23:22:13.672720060+00:00"
---

Storage layout boundary validated: introduced StorageLayout for canonical .atelier/.atelier-state discovery and runtime db resolution; command dispatch, workflow, work, telemetry, init, doctor, activity/evidence/mission/plan paths now route through layout helpers; rg scan confirmed remaining .atelier-state/.atelier state.db references are tests, display strings, or layout constants. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test rebuild_round_trips_canonical_issue_state -- --nocapture; cargo test --test cli_integration test_first_class_records_export_rebuild_and_validate -- --nocapture; cargo test --test cli_integration test_projection_index_rebuilds_changed_sources_before_issue_queries -- --nocapture; git diff --check; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-3es3.
