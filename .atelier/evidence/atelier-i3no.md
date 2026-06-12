---
created_at: "2026-06-11T23:22:07.354752088+00:00"
id: "atelier-i3no"
data: "{\"captured_at\":\"2026-06-11T23:22:07.354698765+00:00\",\"kind\":\"validation\",\"path\":null,\"producer\":null,\"result\":\"pass\",\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3es3"
    type: "validates"
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Storage layout boundary validated: introduced StorageLayout for canonical .atelier/.atelier-state discovery and runtime db resolution; command dispatch, workflow, work, telemetry, init, doctor, activity/evidence/mission/plan paths now route through layout helpers; rg scan confirmed remaining .atelier-state/.atelier state.db references are tests, display strings, or layout constants. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test rebuild_round_trips_canonical_issue_state -- --nocapture; cargo test --test cli_integration test_first_class_records_export_rebuild_and_validate -- --nocapture; cargo test --test cli_integration test_projection_index_rebuilds_changed_sources_before_issue_queries -- --nocapture; git diff --check; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-3es3."
updated_at: "2026-06-11T23:22:13.672720060+00:00"
---

Storage layout boundary validated: introduced StorageLayout for canonical .atelier/.atelier-state discovery and runtime db resolution; command dispatch, workflow, work, telemetry, init, doctor, activity/evidence/mission/plan paths now route through layout helpers; rg scan confirmed remaining .atelier-state/.atelier state.db references are tests, display strings, or layout constants. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test rebuild_round_trips_canonical_issue_state -- --nocapture; cargo test --test cli_integration test_first_class_records_export_rebuild_and_validate -- --nocapture; cargo test --test cli_integration test_projection_index_rebuilds_changed_sources_before_issue_queries -- --nocapture; git diff --check; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-3es3.
