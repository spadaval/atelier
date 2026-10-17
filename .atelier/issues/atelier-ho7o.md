---
created_at: "2026-10-17T17:25:59.150801823+00:00"
id: "atelier-ho7o"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-ho7o
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-stsa"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T18:44:19.953794631+00:00"
status: "done"
title: "Parse mission criteria and let evidence receipts name the criteria they judge"
updated_at: "2026-10-17T18:44:19.953794631+00:00"
---

## Goal

Implement decision atelier-et6q so `work mission` (atelier-stsa, S8) and `debrief` (S4) can show each success criterion with its proof. Criteria are parsed from the strategy, and evidence receipts name the criteria they judge, each with its own result.

## Done when

- One parser reads a mission's criteria: `- <slug>: <text>` bullets under the Success Criteria heading of the linked strategy, or under the mission's own Success criteria section when it has no strategy. Slugs are lowercase kebab-case or `S<n>` and unique within the mission, kept in document order. Continuation lines belong to the bullet before them.
- `atelier evidence record --criterion <slug>=<result>` (repeatable) stores per-criterion results on the receipt, and `evidence show` prints them.
- `check` refuses a receipt that names a criterion its target's mission does not define (the target is the mission or an item advancing it), and one that names a criterion with no mission. The refusal names the slug and the receipt.
- A proof lookup returns, for each criterion, the receipts naming it with their results and freshness.
- Unit tests cover the parser (this repository's strategies parse without error), and CLI tests cover recording, showing and the `check` refusals. validation.md and mission-authoring.md describe criterion naming.

## Context

The decision record atelier-et6q holds the rules. Evidence record shape: docs/product/validation.md and crates/atelier-records. Freshness: crates/atelier-app/src/evidence_freshness.rs.

## Notes

- 2026-10-17 [decision one-way] Store a receipt's criteria as a front-matter list of slug and result pairs, slug first
  Reason: One validation run can pass some criteria and fail others, as et6q decided, and older binaries ignore the unknown key
  Rejected: a criteria body section
- 2026-10-17 [decision] For command capture, a bare --criterion <slug> takes the command's result and slug=result is refused
  Reason: A captured receipt's result comes from the exit status; letting a caller set a different per-criterion result would make the receipt claim what the command did not show
- 2026-10-17 [decision] Mark a criterion from the newest receipt whose commit is in HEAD's history, and report code commits since the proof instead of applying item-level freshness
  Reason: Item freshness fails on any later code commit, so every mission criterion would read unproven soon after its proof; the count keeps the age visible
  Rejected: apply evidence_is_fresh per receipt
- 2026-10-17 [decision] Show strategy criteria parse problems in views, not as check findings
  Reason: Existing missions may hold prose bullets in Success criteria, and check failing on them would block unrelated work; receipts naming undefined criteria are still check findings

## Close Reason

Mission criteria are parsed from the strategy or the mission's own section by one parser, evidence receipts name the criteria they judge with a result each, check and evidence record refuse slugs the target's single mission does not define, and a proof lookup gives each criterion its receipts, mark and age.

### Deviations
Review atelier-dqty showed that matching proof by slug let shared S<n> slugs prove another mission's criteria; receipts now name criteria only through a target in one mission, which et6q did not spell out. The proof lookup reports age as code commits since the deciding receipt instead of applying item-level freshness.

### Decisions
See this item's decision notes: criteria stored as slug/result pairs (one-way), bare slugs on command capture, marks from the newest receipt in HEAD's history with age reported, and parse problems shown in views rather than check.

### Surprises
Every strategy here already used `- S<n>:` bullets, so the parser accepted them unchanged. Item-level evidence freshness turned out unusable for mission criteria, since any later code commit breaks it.

### Open items
- atelier-rx22 (debrief) must decide how it presents proof that predates later code; the strategy forbids presenting a receipt as passing without checking freshness.
