---
created_at: "2026-11-07T05:00:24.620382782+00:00"
id: "atelier-4yqd"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Record the simpler lifecycle in ADR 0028, and add the no-regrowth principle"
updated_at: "2026-11-07T05:00:24.620382782+00:00"
---

## Goal

Sumeet decided the simpler lifecycle on 2026-11-06 (docs/strategies/simpler-lifecycle.md, Shape). Record those decisions in one ADR so later items and other repositories have one reference, mark what it supersedes in ADRs 0019, 0025, 0026 and 0027, and add the principle against regrowth to PRODUCT_INTENT.md now, since it governs the rest of this mission.

## Done when

- [ ] `docs/adr/0028-*.md` states each Shape decision of the strategy, with the transcript analysis summarised as context, and names exactly which decisions of ADRs 0019, 0025, 0026 and 0027 and which PRODUCT_INTENT.md principles it supersedes. Mechanics the strategy leaves to later items are named as open, not invented.
- [ ] The Status line of each superseded ADR points to ADR 0028 and says which part it replaces.
- [ ] PRODUCT_INTENT.md has the no-regrowth principle: a new gate or record type names the kind of defect it would have caught in real threads, and a rule that only patches another rule's edge case is a reason to remove the first rule. Other principles change only when the item that changes the behaviour lands.
- [ ] Sumeet has accepted the ADR (his approval recorded on this item).

## Scope

Docs only: the new ADR, Status lines of the superseded ADRs, one principle in PRODUCT_INTENT.md.

## Context

docs/adr/0019-verified-repo-native-tracker.md, 0025-findings-carry-their-disposition.md, 0026-evidence-records-only-what-nothing-else-records.md, 0027-proof-has-a-time.md; PRODUCT_INTENT.md. The analysis that motivated the mission is summarised in the strategy's Why.
