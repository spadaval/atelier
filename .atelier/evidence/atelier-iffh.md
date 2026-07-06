---
created_at: "2026-06-30T18:27:58.057659312+00:00"
id: "atelier-iffh"
evidence_type: "test"
captured_at: "2026-06-30T18:27:58.057651527+00:00"
target:
  kind: "issue"
  id: "atelier-qu06"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qu06"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Implemented git.* workflow validator/action names for mission branches; focused proof passed: cargo check -p atelier-cli; cargo fmt -- --check; cargo test -p atelier-workflow; cargo test -p atelier-cli commands::workflow::tests::default_validators_are_target_and_transition_aware; cargo test -p atelier-cli commands::workflow_planning::tests::branch_prepare_is_explicit_planned_action; cargo test -p atelier-cli --test cli_integration test_mission_start_prepares_mission_branch_from_base; cargo test -p atelier-cli --test cli_integration test_epic_start_from_mission_branch_uses_current_branch_base; cargo test -p atelier-cli --test cli_integration test_epic_start_rejects_wrong_branch_for_mission_scope; cargo test -p atelier-cli --test cli_integration test_off_base_branch_blocks_mission_closeout; cargo test -p atelier-cli --test cli_integration test_epic_start_requires_base_branch; cargo build -p atelier-cli; git diff --check; target/debug/atelier check atelier-qu06; target/debug/atelier check atelier-sszj."
updated_at: "2026-06-30T18:28:04.387306817+00:00"
---

Implemented git.* workflow validator/action names for mission branches; focused proof passed: cargo check -p atelier-cli; cargo fmt -- --check; cargo test -p atelier-workflow; cargo test -p atelier-cli commands::workflow::tests::default_validators_are_target_and_transition_aware; cargo test -p atelier-cli commands::workflow_planning::tests::branch_prepare_is_explicit_planned_action; cargo test -p atelier-cli --test cli_integration test_mission_start_prepares_mission_branch_from_base; cargo test -p atelier-cli --test cli_integration test_epic_start_from_mission_branch_uses_current_branch_base; cargo test -p atelier-cli --test cli_integration test_epic_start_rejects_wrong_branch_for_mission_scope; cargo test -p atelier-cli --test cli_integration test_off_base_branch_blocks_mission_closeout; cargo test -p atelier-cli --test cli_integration test_epic_start_requires_base_branch; cargo build -p atelier-cli; git diff --check; target/debug/atelier check atelier-qu06; target/debug/atelier check atelier-sszj.
