---
created_at: "2026-06-19T02:12:45.940752104+00:00"
id: "atelier-oz5b"
evidence_type: "validation"
captured_at: "2026-06-19T02:12:45.940745580+00:00"
target:
  kind: "issue"
  id: "atelier-bilp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bilp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Updated Agent Factory guidance, CLI help/status/mission prompts, validation docs, command-surface fixtures, and import evidence text so ordinary agents use lint/status/mission surfaces while doctor/export remain admin or targeted diagnostics. Checks: cargo fmt -- --check; cargo test --test cli_integration test_top_level_help_only_shows_core_commands; cargo test --test cli_integration test_root_status_summarizes_checkout_orientation; cargo test --test cli_integration test_mission_status_cli_reports_control_state; cargo test --test cli_integration workflow_check; atelier lint atelier-bilp; atelier lint; git diff --check."
updated_at: "2026-06-19T02:12:48.447733738+00:00"
---

Updated Agent Factory guidance, CLI help/status/mission prompts, validation docs, command-surface fixtures, and import evidence text so ordinary agents use lint/status/mission surfaces while doctor/export remain admin or targeted diagnostics. Checks: cargo fmt -- --check; cargo test --test cli_integration test_top_level_help_only_shows_core_commands; cargo test --test cli_integration test_root_status_summarizes_checkout_orientation; cargo test --test cli_integration test_mission_status_cli_reports_control_state; cargo test --test cli_integration workflow_check; atelier lint atelier-bilp; atelier lint; git diff --check.
