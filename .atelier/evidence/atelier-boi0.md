---
created_at: "2026-06-13T23:10:56.830653091+00:00"
id: "atelier-boi0"
evidence_type: "test"
captured_at: "2026-06-13T23:10:56.830568285+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-50tm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Large-function hotspot triage for atelier-50tm. Existing evidence atelier-dltk was inspected and found useful but truncated. Fresh command run: cargo clippy --message-format=json --all-targets -- -W clippy::too_many_lines | jq -r ...; duplicate reports across lib/bin/test targets were deduplicated to 26 unique hotspots.\n\nProduction hotspots and owners:\n- src/db/mod.rs init_schema (187 lines) -> atelier-ggls.\n- src/db/mod.rs migrate_issue_ids_to_text (210 lines) -> atelier-ggls.\n- src/workflow_policy.rs parse_workflows (119 lines) -> atelier-wj05.\n- src/workflow_policy.rs validate_policy (124 lines) -> atelier-wj05.\n- src/commands/workflow.rs transition_issue (117 lines) -> atelier-wj05.\n- src/commands/agent_factory.rs update (118), update_lifecycle (141), lint (160) -> atelier-d7lw, with workflow-orientation extraction overlap in atelier-4u5h where relevant.\n- src/commands/evidence.rs capture (106), src/commands/mission.rs status_one (135), src/commands/plan.rs validate_bulk_plan (105), apply_bulk_plan (220), src/commands/status.rs run (224), src/main.rs dispatch_issue (332), run (638), command_identity (115) -> atelier-d7lw.\n- src/commands/rebuild.rs load_projection (158) -> atelier-ggls.\n\nTest hotspots and owner:\n- tests/cli_integration.rs test_diagnostics_slow_summarizes_fixture_events, test_issue_mutations_create_activity_sidecars, test_command_result_json_mode_is_rejected_and_human_subset_works, test_first_class_records_export_rebuild_and_validate, test_evidence_capture_records_command_metadata_and_attaches_targets, test_orientation_commands_enter_degraded_mode_for_malformed_records, test_mission_list_human_overview_orders_and_summarizes, test_mission_status_cli_reports_control_state, test_work_lifecycle_human_output_and_guards -> atelier-kpm8.\n\nNo not-applicable hotspot was found: every current too_many_lines warning maps to an owner issue. No code refactor was performed in this validation slice."
updated_at: "2026-06-13T23:11:06.311747360+00:00"
---

Large-function hotspot triage for atelier-50tm. Existing evidence atelier-dltk was inspected and found useful but truncated. Fresh command run: cargo clippy --message-format=json --all-targets -- -W clippy::too_many_lines | jq -r ...; duplicate reports across lib/bin/test targets were deduplicated to 26 unique hotspots.

Production hotspots and owners:
- src/db/mod.rs init_schema (187 lines) -> atelier-ggls.
- src/db/mod.rs migrate_issue_ids_to_text (210 lines) -> atelier-ggls.
- src/workflow_policy.rs parse_workflows (119 lines) -> atelier-wj05.
- src/workflow_policy.rs validate_policy (124 lines) -> atelier-wj05.
- src/commands/workflow.rs transition_issue (117 lines) -> atelier-wj05.
- src/commands/agent_factory.rs update (118), update_lifecycle (141), lint (160) -> atelier-d7lw, with workflow-orientation extraction overlap in atelier-4u5h where relevant.
- src/commands/evidence.rs capture (106), src/commands/mission.rs status_one (135), src/commands/plan.rs validate_bulk_plan (105), apply_bulk_plan (220), src/commands/status.rs run (224), src/main.rs dispatch_issue (332), run (638), command_identity (115) -> atelier-d7lw.
- src/commands/rebuild.rs load_projection (158) -> atelier-ggls.

Test hotspots and owner:
- tests/cli_integration.rs test_diagnostics_slow_summarizes_fixture_events, test_issue_mutations_create_activity_sidecars, test_command_result_json_mode_is_rejected_and_human_subset_works, test_first_class_records_export_rebuild_and_validate, test_evidence_capture_records_command_metadata_and_attaches_targets, test_orientation_commands_enter_degraded_mode_for_malformed_records, test_mission_list_human_overview_orders_and_summarizes, test_mission_status_cli_reports_control_state, test_work_lifecycle_human_output_and_guards -> atelier-kpm8.

No not-applicable hotspot was found: every current too_many_lines warning maps to an owner issue. No code refactor was performed in this validation slice.
