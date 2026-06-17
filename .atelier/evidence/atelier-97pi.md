---
created_at: "2026-06-17T23:37:16.760068594+00:00"
id: "atelier-97pi"
evidence_type: "validation"
captured_at: "2026-06-17T23:37:16.760055181+00:00"
agent_identity: "codex"
target:
  kind: "issue"
  id: "atelier-uro5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uro5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Moved migrated dispatch paths to atelier-app use_cases for storage selection and record/target resolution. Validation: cargo fmt -- --check; cargo check -p atelier-cli; cargo test -p atelier-app use_case; cargo test -p atelier-cli setup_guidance::test_root_status_summarizes_checkout_orientation; cargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail; cargo test -p atelier-cli setup_guidance::test_workflow_help_is_scoped_as_advanced_internal_diagnostic; cargo test -p atelier-cli records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets; cargo build -p atelier-cli; target/debug/atelier lint atelier-uro5; target/debug/atelier doctor --fix; git diff --check. Search evidence: migrated main.rs status, mission, evidence, and workflow check arms route through use_cases::*; app use_cases has no println!/eprintln!."
updated_at: "2026-06-17T23:37:21.156817581+00:00"
---

Moved migrated dispatch paths to atelier-app use_cases for storage selection and record/target resolution. Validation: cargo fmt -- --check; cargo check -p atelier-cli; cargo test -p atelier-app use_case; cargo test -p atelier-cli setup_guidance::test_root_status_summarizes_checkout_orientation; cargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail; cargo test -p atelier-cli setup_guidance::test_workflow_help_is_scoped_as_advanced_internal_diagnostic; cargo test -p atelier-cli records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets; cargo build -p atelier-cli; target/debug/atelier lint atelier-uro5; target/debug/atelier doctor --fix; git diff --check. Search evidence: migrated main.rs status, mission, evidence, and workflow check arms route through use_cases::*; app use_cases has no println!/eprintln!.
