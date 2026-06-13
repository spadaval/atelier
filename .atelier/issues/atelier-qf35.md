---
created_at: "2026-06-13T02:33:41.341571624+00:00"
id: "atelier-qf35"
issue_type: "epic"
labels:
- "process"
- "proof"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-b2vi"
  - kind: "issue"
    id: "atelier-cbru"
  - kind: "issue"
    id: "atelier-ovs0"
  - kind: "issue"
    id: "atelier-rc1v"
  - kind: "issue"
    id: "atelier-z80r"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T16:09:15.927464357+00:00"
status: "done"
title: "Define strong proof and contract-first workflow"
updated_at: "2026-06-13T16:09:15.927464357+00:00"
---

## Description

Define what strong proof means in Atelier and make contract-first work the default for public behavior, workflow policy, and closeout rules. This epic should turn weak-proof lessons from atelier-tcmr into durable product and Agent Factory guidance.
- Strong proof is defined in product and quality vocabulary as claim-specific, reproducible, attached, classified, scoped, and independently validated when risk requires it.
- Weak proof is defined as broad, summary-only, unattached, unverifiable, or not mapped to a claim.
- Validation policy distinguishes qualitative judgment from quantitative measurement: subjective claims can pass through inspectable expert judgment, while numerical claims use hard metrics whenever practical.
- Docs-first and test-first expectations are explicit for public CLI behavior, tracker transitions, evidence policy, and Agent Factory workflow changes.
- Validation placement guidance tells authors what belongs at the mission, epic, issue, and validation-item layers without duplicating detail across all of them.
- Work-item authoring guidance tells agents when broad checks support proof and when scenario proof is required.
- Anti-red-tape guidance prevents parent validation from becoming a second implementation spec.
- Documentation diff updates CONTEXT and validation/process guidance with the strong-proof vocabulary.
- Focused examples show strong and weak proof for ordinary issues, parent closeout, CLI behavior, subjective information-hierarchy validation, performance validation, validation placement, and validation items.
- Tracker lint and export checks pass after the guidance changes.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
