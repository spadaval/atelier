---
created_at: "2026-06-20T05:02:50.430283465+00:00"
id: "atelier-pkc9"
evidence_type: "validation"
captured_at: "2026-06-20T05:02:50.430281726+00:00"
target:
  kind: "issue"
  id: "atelier-o5a9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-o5a9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed validator attempts from derived session tracking and evidence attachment activity. Validation passed: cargo nextest run -p atelier-records issue_attempts_are_derived_by_issue_role_and_serial; cargo nextest run -p atelier-cli --test cli_integration -E 'test(workflow_milestones_emit_issue_attempt_metadata_without_session_records)'; cargo fmt -- --check; cargo check -p atelier-cli; git diff --check. Local target/debug/atelier session list --active shows no /validator/ rows, and target/debug/atelier status --quiet reports work=1 with tracker=current."
updated_at: "2026-06-20T05:02:53.269352737+00:00"
---

Removed validator attempts from derived session tracking and evidence attachment activity. Validation passed: cargo nextest run -p atelier-records issue_attempts_are_derived_by_issue_role_and_serial; cargo nextest run -p atelier-cli --test cli_integration -E 'test(workflow_milestones_emit_issue_attempt_metadata_without_session_records)'; cargo fmt -- --check; cargo check -p atelier-cli; git diff --check. Local target/debug/atelier session list --active shows no /validator/ rows, and target/debug/atelier status --quiet reports work=1 with tracker=current.
