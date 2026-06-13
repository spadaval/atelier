---
created_at: "2026-06-13T23:51:22.269588433+00:00"
id: "atelier-ny3b"
evidence_type: "validation"
captured_at: "2026-06-13T23:51:22.269469753+00:00"
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
    id: "atelier-i9ob"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Work selection guidance moved into status surfaces: atelier mission status atelier-man9 prints Selectable Work rows with ready reason, parent, proof state, Blocked Work, and direct atelier start guidance; atelier status and atelier start route agents back to status surfaces. Checks passed: cargo fmt -- --check; cargo nextest run test_root_status_summarizes_checkout_orientation test_root_status_guides_active_work_to_transition_or_abandon test_mission_status_cli_reports_control_state test_mission_status_names_concrete_closeout_blockers test_mission_status_names_stale_and_malformed_record_blockers; git diff --check; git diff --check -- '*.md'; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor."
updated_at: "2026-06-13T23:51:27.952497759+00:00"
---

Work selection guidance moved into status surfaces: atelier mission status atelier-man9 prints Selectable Work rows with ready reason, parent, proof state, Blocked Work, and direct atelier start guidance; atelier status and atelier start route agents back to status surfaces. Checks passed: cargo fmt -- --check; cargo nextest run test_root_status_summarizes_checkout_orientation test_root_status_guides_active_work_to_transition_or_abandon test_mission_status_cli_reports_control_state test_mission_status_names_concrete_closeout_blockers test_mission_status_names_stale_and_malformed_record_blockers; git diff --check; git diff --check -- '*.md'; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor.
