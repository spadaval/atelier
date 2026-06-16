---
created_at: "2026-06-14T07:26:30.524381105+00:00"
id: "atelier-cmy9"
evidence_type: "validation"
captured_at: "2026-06-14T07:26:30.524271012+00:00"
target:
  kind: "issue"
  id: "atelier-ayr6"
  role: "validates"
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
status: "recorded"
title: "Validation closeout fixture now records claim-specific proof for every Evidence requirement before expecting closeout to pass, while broad unrelated pass evidence is still rejected. Proof: cargo nextest run --test cli_integration -E 'test(test_validation_issue_closeout_requires_contract_audit_evidence)'; cargo fmt -- --check; git diff --check; atelier lint atelier-ayr6."
updated_at: "2026-06-14T07:26:33.059346373+00:00"
---

Validation closeout fixture now records claim-specific proof for every Evidence requirement before expecting closeout to pass, while broad unrelated pass evidence is still rejected. Proof: cargo nextest run --test cli_integration -E 'test(test_validation_issue_closeout_requires_contract_audit_evidence)'; cargo fmt -- --check; git diff --check; atelier lint atelier-ayr6.
