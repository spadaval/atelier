---
created_at: "2026-06-13T19:48:06.321570581+00:00"
id: "atelier-eo94"
evidence_type: "validation"
captured_at: "2026-06-13T19:48:06.321493802+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-eovw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "atelier-eovw validation: new issue creation after workflow init+migrate writes configured initial status todo; transition options immediately show start allowed; root start moves todo -> in_progress. Regression test confirms legacy open/closed/archived workflow migrate-statuses mappings remain available. Commands: cargo fmt -- --check; cargo build; cargo test --test cli_integration test_issue_create_after_workflow_migration_uses_configured_initial_status -- --nocapture; cargo test --test cli_integration test_workflow_migrate_statuses_rewrites_legacy_issue_statuses_and_preserves_close_metadata -- --nocapture; target/debug/atelier workflow check; target/debug/atelier lint; target/debug/atelier export --check; git diff --check."
updated_at: "2026-06-13T19:48:08.395950631+00:00"
---

atelier-eovw validation: new issue creation after workflow init+migrate writes configured initial status todo; transition options immediately show start allowed; root start moves todo -> in_progress. Regression test confirms legacy open/closed/archived workflow migrate-statuses mappings remain available. Commands: cargo fmt -- --check; cargo build; cargo test --test cli_integration test_issue_create_after_workflow_migration_uses_configured_initial_status -- --nocapture; cargo test --test cli_integration test_workflow_migrate_statuses_rewrites_legacy_issue_statuses_and_preserves_close_metadata -- --nocapture; target/debug/atelier workflow check; target/debug/atelier lint; target/debug/atelier export --check; git diff --check.
