---
created_at: "2026-10-17T00:31:28.261958765+00:00"
id: "atelier-et6q"
issue_type: "decision"
labels:
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-stsa"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Decide how evidence and issues name the success criterion they prove"
updated_at: "2026-10-17T00:31:28.261958765+00:00"
---

## Context

Mission atelier-r6bt. Strategy Risks and Unknowns name this as the mission's first decision. atelier-stsa's Unknowns asked the same question. Evidence record shape: `docs/product/validation.md`; strategy parsing today: the mission dashboard reads only the strategy path.

## Question

`debrief` (S4 of docs/strategies/review-and-introspection.md) shows the outcome against each success criterion with linked proof, and `work mission` (S8, atelier-stsa) marks each criterion proven, unproven, or failing. Both need one rule for how a strategy's criteria are identified and how an evidence receipt or issue says which criterion it proves. Nothing is built on criteria until this is decided, so both views share one parser.

## Options

- Criteria parsed from the strategy's `- S<n>:` bullets under Success Criteria (the convention every strategy in this repository already follows), with a criterion list on evidence receipts (`atelier evidence record --criterion S1 --criterion S3`). Proof is where the claim is checked; issues stay free of criterion bookkeeping. Validation receipts already name criteria in prose (atelier-xz11 recorded S1-S8), so this formalizes existing practice.
- The same parsing, with the criterion list on the issue (a `Proves: S1` line or a `criteria` field) and evidence inheriting it. Cheaper per receipt, but an issue's proof may cover only part of what the issue names.
- A free-form tag in the evidence summary (`[S1]`) matched by regex. No schema change, but `check` cannot refuse a typo and `prune` cannot reason about it.
