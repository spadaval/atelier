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
closed_at: "2026-10-17T15:57:12.116803728+00:00"
status: "done"
title: "Decide how evidence and issues name the success criterion they prove"
updated_at: "2026-10-17T15:57:12.116803728+00:00"
---

## Context

Mission atelier-r6bt. Strategy Risks and Unknowns name this as the mission's first decision. atelier-stsa's Unknowns asked the same question. Evidence record shape: `docs/product/validation.md`; strategy parsing today: the mission dashboard reads only the strategy path.

## Question

`debrief` (S4 of docs/strategies/review-and-introspection.md) shows the outcome against each success criterion with linked proof, and `work mission` (S8, atelier-stsa) marks each criterion proven, unproven, or failing. Both need one rule for how a strategy's criteria are identified and how an evidence receipt or issue says which criterion it proves. Nothing is built on criteria until this is decided, so both views share one parser.

## Options

- Criteria parsed from the strategy's `- S<n>:` bullets under Success Criteria (the convention every strategy in this repository already follows), with a criterion list on evidence receipts (`atelier evidence record --criterion S1 --criterion S3`). Proof is where the claim is checked; issues stay free of criterion bookkeeping. Validation receipts already name criteria in prose (atelier-xz11 recorded S1-S8), so this formalizes existing practice.
- The same parsing, with the criterion list on the issue (a `Proves: S1` line or a `criteria` field) and evidence inheriting it. Cheaper per receipt, but an issue's proof may cover only part of what the issue names.
- A free-form tag in the evidence summary (`[S1]`) matched by regex. No schema change, but `check` cannot refuse a typo and `prune` cannot reason about it.

## Decision

Decided by Sumeet, 2026-10-17. Criteria get semantic names. Proof names them on the evidence receipt, with one result for each criterion.

- **Identity.** A criterion is a bullet `- <slug>: <text>` under the success criteria heading. The heading lives in the mission's linked strategy, or in the mission's own Success criteria section when it has no strategy. The slug is lowercase kebab-case and unique within the mission. Views list criteria in document order. Existing `- S<n>:` bullets are accepted as slugs, so no mission or strategy needs migrating. New criteria use semantic slugs (for example `pipe-safe`, `quiet-tokens`).
- **Proof.** Evidence receipts name the criteria they judge, each with its own result: `atelier evidence record --criterion pipe-safe=pass --criterion quiet-tokens=fail`. One validation run can then pass some criteria and fail others, as atelier-pnha did, without marking every criterion failing. Issues carry no criterion bookkeeping, because proof is recorded where the claim is checked.
- **Checking.** `check` refuses a receipt that names a criterion its mission does not define. This also covers a slug renamed after proof exists: the orphaned proof is reported, so a rename is never silent. Criterion IDs therefore need no rule about staying stable once a mission starts.
- **One parser.** `debrief` (S4) and the mission dashboard (atelier-stsa) share one criteria parser and one proof lookup.
- **Not covered.** Proof still matches a criterion whose text was edited after the proof was recorded. The debrief may flag criteria whose text changed since their proof, working that out from Git history rather than storing new state. Existing receipts that name criteria only in prose stay as they are: their criteria show as unproven until new proof names them.

Rejected options:
- Positional `S<n>` IDs as the scheme. Inserting or reordering a criterion changes what a number means, and `S3=fail` says nothing on its own.
- The criterion list on the issue. An issue's proof may cover only part of what the issue names.
- A free-form tag in the summary. `check` cannot refuse a typo in it.

## Close Reason

Criteria are named by semantic slugs (S<n> still accepted), read from the strategy or the mission's own Success criteria, and evidence receipts name each criterion they judge with its own result; check refuses unknown criteria.
