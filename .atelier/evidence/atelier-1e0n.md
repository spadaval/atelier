---
created_at: "2026-06-14T07:00:17.049839259+00:00"
id: "atelier-1e0n"
evidence_type: "validation"
captured_at: "2026-06-14T07:00:17.049799984+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-hf3f"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-hf3f"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Focused proof coverage checks passed: cargo test --test cli_integration test_validation_issue_closeout_reports_blocked_requirement_coverage -- --exact; cargo test --test cli_integration test_mission_audit_reports_parent_proof_coverage_classifications -- --exact; git diff --check; atelier lint"
updated_at: "2026-06-14T07:00:19.051438843+00:00"
---

Focused proof coverage checks passed: cargo test --test cli_integration test_validation_issue_closeout_reports_blocked_requirement_coverage -- --exact; cargo test --test cli_integration test_mission_audit_reports_parent_proof_coverage_classifications -- --exact; git diff --check; atelier lint
