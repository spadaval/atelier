---
created_at: "2026-06-14T07:26:30.524381105+00:00"
id: "atelier-cmy9"
evidence_type: "validation"
captured_at: "2026-06-14T07:26:30.524271012+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-ayr6"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ayr6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Validation closeout fixture now records claim-specific proof for every Evidence requirement before expecting closeout to pass, while broad unrelated pass evidence is still rejected. Proof: cargo nextest run --test cli_integration -E 'test(test_validation_issue_closeout_requires_contract_audit_evidence)'; cargo fmt -- --check; git diff --check; atelier lint atelier-ayr6."
updated_at: "2026-06-14T07:26:33.059346373+00:00"
---

Validation closeout fixture now records claim-specific proof for every Evidence requirement before expecting closeout to pass, while broad unrelated pass evidence is still rejected. Proof: cargo nextest run --test cli_integration -E 'test(test_validation_issue_closeout_requires_contract_audit_evidence)'; cargo fmt -- --check; git diff --check; atelier lint atelier-ayr6.
