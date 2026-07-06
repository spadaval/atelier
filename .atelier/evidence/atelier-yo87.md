---
created_at: "2026-06-30T20:08:58.680582125+00:00"
id: "atelier-yo87"
evidence_type: "test"
captured_at: "2026-06-30T20:08:58.680571002+00:00"
target:
  kind: "issue"
  id: "atelier-gayo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-gayo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run (709 passed, 4 skipped); cargo fmt -- --check; cargo build -p atelier-cli; git diff --check; cargo test -p atelier-app pr::tests --lib; cargo test -p atelier-app review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings --lib; cargo test -p atelier-cli --test cli_integration test_branch_integrate_action_failure_rolls_back_status_with_recovery -- --nocapture; cargo test -p atelier-cli --test cli_integration test_epic_close_integrates_into_recorded_mission_branch -- --nocapture; target/debug/atelier check atelier-gayo; target/debug/atelier check atelier-sszj"
updated_at: "2026-06-30T20:09:04.467906685+00:00"
---

cargo nextest run (709 passed, 4 skipped); cargo fmt -- --check; cargo build -p atelier-cli; git diff --check; cargo test -p atelier-app pr::tests --lib; cargo test -p atelier-app review_room::tests::room_merge_requires_current_approval_and_resolved_blocking_findings --lib; cargo test -p atelier-cli --test cli_integration test_branch_integrate_action_failure_rolls_back_status_with_recovery -- --nocapture; cargo test -p atelier-cli --test cli_integration test_epic_close_integrates_into_recorded_mission_branch -- --nocapture; target/debug/atelier check atelier-gayo; target/debug/atelier check atelier-sszj
