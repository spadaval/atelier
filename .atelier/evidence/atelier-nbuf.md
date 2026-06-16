---
created_at: "2026-06-13T04:16:17.526630037+00:00"
id: "atelier-nbuf"
evidence_type: "test"
captured_at: "2026-06-13T04:16:17.526603484+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-b7wl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Implemented mission status closeout drill-down: added  backed by contract audit, with missing-id rejection and help coverage. Checks: cargo fmt -- --check; git diff --check; cargo test --test cli_integration test_mission_status_help_exposes_closeout_drilldown; cargo test --test cli_integration test_mission_audit_reports_missing_partial_and_ready_proof; atelier mission status atelier-19mc shows Docs/Help Drift: clear; atelier lint; atelier export --check."
updated_at: "2026-06-13T04:16:26.443690472+00:00"
---

Implemented mission status closeout drill-down: added  backed by contract audit, with missing-id rejection and help coverage. Checks: cargo fmt -- --check; git diff --check; cargo test --test cli_integration test_mission_status_help_exposes_closeout_drilldown; cargo test --test cli_integration test_mission_audit_reports_missing_partial_and_ready_proof; atelier mission status atelier-19mc shows Docs/Help Drift: clear; atelier lint; atelier export --check.
