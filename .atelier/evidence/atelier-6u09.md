---
created_at: "2026-06-13T22:45:02.786719036+00:00"
id: "atelier-6u09"
evidence_type: "validation"
captured_at: "2026-06-13T22:45:02.786649134+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ja3o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed hidden work start bypass. Proof: cargo fmt -- --check passed; cargo nextest run --test cli_integration root_start passed (test_root_start_refuses_structurally_invalid_issue, test_root_start_reports_workflow_validator_failure, test_root_start_applies_workflow_transition_and_records_active_work); cargo nextest run --test cli_integration hidden_work_start work_lifecycle_human_output_and_guards active_mission_focus_guides_status_and_work mission_start_requires_explicit_switch_and_warns_for_outside_work passed (4 tests); target/debug/atelier work start atelier-z1p8 now rejects with unrecognized subcommand 'start'; rg residue: start_lifecycle remains only at src/main.rs root start dispatch and src/commands/work.rs, commands::work::start residue is only the start_lifecycle dispatch site in src/main.rs, and work start residue is limited to tests/docs classification plus historical reference; target/debug/atelier lint and target/debug/atelier export --check passed."
updated_at: "2026-06-13T22:45:05.055395861+00:00"
---

Removed hidden work start bypass. Proof: cargo fmt -- --check passed; cargo nextest run --test cli_integration root_start passed (test_root_start_refuses_structurally_invalid_issue, test_root_start_reports_workflow_validator_failure, test_root_start_applies_workflow_transition_and_records_active_work); cargo nextest run --test cli_integration hidden_work_start work_lifecycle_human_output_and_guards active_mission_focus_guides_status_and_work mission_start_requires_explicit_switch_and_warns_for_outside_work passed (4 tests); target/debug/atelier work start atelier-z1p8 now rejects with unrecognized subcommand 'start'; rg residue: start_lifecycle remains only at src/main.rs root start dispatch and src/commands/work.rs, commands::work::start residue is only the start_lifecycle dispatch site in src/main.rs, and work start residue is limited to tests/docs classification plus historical reference; target/debug/atelier lint and target/debug/atelier export --check passed.
