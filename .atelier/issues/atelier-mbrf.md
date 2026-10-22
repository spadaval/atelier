---
created_at: "2026-10-17T19:00:36.352316978+00:00"
id: "atelier-mbrf"
issue_type: "feature"
labels:
- "land"
- "merge"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let a trivial change land on the base with a visible exemption, without an issue or work branch"
updated_at: "2026-10-22T22:49:02.392765092+00:00"
---

## Goal

A trivial change to a non-record file, such as a one-line `workflow.yaml` fix or a typo in an ADR, today needs an owner issue, a work branch, a `review trivial` event and `atelier merge`. `review trivial` pins a commit on a work branch, and `land` on the base accepts only record and strategy commits or commits covered by a merged owner's approval or exemption (`docs/product/reviews-and-merge.md`, `land_base_unreviewed`). So the exemption meant for changes whose correctness is evident costs a full cycle. Under the 2026-10-22 decision, review has one standard: non-trivial changes, prose included, get independent review, and only the trivial exemption skips it. This item removes the owner requirement from that exemption, and nothing else.

## Done when

- A trivial exemption can pin a base commit with no owner issue (for example `atelier review trivial --base <commit> --identity <who> "why correctness is evident"`), and `land` on the base counts that commit as covered.
- `land` still refuses an unexempted non-record commit with `land_base_unreviewed`, and an exemption pinned to one commit does not cover a later one.
- The exemption shows in the review history and in mission debriefs, as owner exemptions do today.
- `docs/product/reviews-and-merge.md` states the rule.
- Tests cover an exempted base commit, an unexempted one, and a later commit after an exemption.

## Context

The human rejected two alternatives on 2026-10-22: exempting planning paths from review, and a separate review standard for planning text. Records and mission strategies stay exempt as they are, because requiring an issue to edit issues would be recursive.

## Notes

- 2026-10-22 [note] From review of atelier-r28p (finding atelier-kotb): docs/product/reviews-and-merge.md:85-94, the comment at review_gate.rs:134-139 and atelier-r28p's decision note say a record 'filed on the base after the approval' stays editable; the code checks only that the record is absent from the approval's pinned tree, so a base record filed before the approval and merged in later is editable too. Reword all three to 'not in the approval's pinned tree' while editing this doc section.
