---
created_at: "2026-06-14T07:11:02.906366509+00:00"
id: "atelier-ayr6"
issue_type: "bug"
labels:
- "evidence"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-bqau"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T07:30:39.163594472+00:00"
status: "done"
title: "Fix validation proof fixture coverage for contract-audit closeout"
updated_at: "2026-06-14T07:30:39.163594472+00:00"
---

## Description

Independent validation of the transcript-derived proof gates found that the focused closeout test for validation issues no longer proves all Evidence requirements. The scenario rejects broad pass evidence correctly, then records one contract-audit proof record, but closeout remains blocked because the second Evidence bullet is still uncovered.

## Outcome

The validation closeout scenario either records claim-specific proof for every Evidence bullet or the fixture is reshaped so the expected pass condition matches the intended product contract.

## Evidence

- Focused CLI integration test transcript shows `test_validation_issue_closeout_requires_contract_audit_evidence` passes after the fixture or implementation correction.
- Focused CLI integration test transcript for `test_validation_issue_closeout_requires_contract_audit_evidence` still shows broad unrelated pass evidence is rejected for validation issue closeout.
