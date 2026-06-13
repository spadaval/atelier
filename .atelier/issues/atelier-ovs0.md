---
created_at: "2026-06-13T02:35:54.474582860+00:00"
id: "atelier-ovs0"
issue_type: "validation"
labels:
- "proof"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Validate strong-proof guidance against representative work"
updated_at: "2026-06-13T02:35:54.474582860+00:00"
---

## Description

Validate that the strong-proof guidance is operational by applying it to representative ordinary issue, CLI behavior, parent closeout, subjective validation, quantitative validation, and validation-item examples.

## Outcome

- Each representative example is classified as strong proof, weak proof, blocked, deferred, or not applicable with a reason.
- Subjective examples are judged against explicit evaluator role, scenario, comparison baseline, decision criteria, and captured evidence rather than pre-baked output.
- Quantitative examples are judged against hard metrics or an explicit reason why a metric is not practical.
- The validator identifies any wording that would let broad green checks masquerade as claim-specific proof.
- Follow-up issues are created for unclear or unenforceable guidance.

## Evidence

- Evidence record attached to this validation issue contains the classification table.
- Command transcripts cover tracker lint/export and any docs/help parity checks used in the validation.
