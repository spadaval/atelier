---
created_at: "2026-06-20T04:59:32.933849768+00:00"
id: "atelier-wvor"
evidence_type: "validation"
captured_at: "2026-06-20T04:59:32.933848644+00:00"
target:
  kind: "issue"
  id: "atelier-2sma"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2sma"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed the special issue close subcommand and converted active guidance/tests/docs to issue transition close. Validation passed: cargo nextest run -p atelier-cli --test cli_integration -E 'test(test_issue_help_uses_reduced_lifecycle_surface) | test(test_removed_issue_close_command_rejects_to_and_reason_flags) | test(test_issue_transition_close_reports_blockers_and_records_blocked_activity)'; cargo fmt -- --check; cargo check -p atelier-cli; git diff --check; target/debug/atelier issue close --help is rejected as unrecognized; issue help omits close."
updated_at: "2026-06-20T04:59:35.886387045+00:00"
---

Removed the special issue close subcommand and converted active guidance/tests/docs to issue transition close. Validation passed: cargo nextest run -p atelier-cli --test cli_integration -E 'test(test_issue_help_uses_reduced_lifecycle_surface) | test(test_removed_issue_close_command_rejects_to_and_reason_flags) | test(test_issue_transition_close_reports_blockers_and_records_blocked_activity)'; cargo fmt -- --check; cargo check -p atelier-cli; git diff --check; target/debug/atelier issue close --help is rejected as unrecognized; issue help omits close.
