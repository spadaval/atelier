---
created_at: "2026-11-05T04:30:29.919889687+00:00"
id: "atelier-mgl0"
issue_type: "bug"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-mgl0
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lfs7"
    type: "discovered_from"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Fix the debrief defects the S4 and S7 reviews found"
updated_at: "2026-11-05T04:30:37.210222565+00:00"
---

## Goal

S4 and S7 of docs/strategies/review-and-introspection.md: fix the debrief defects that the S4 validation and the S7 agent review of atelier-vt8q's page marked FIX NOW.

## Done when

- A delivered item's and a done child's non-empty report Decisions part appear as a Decided fact in both forms.
- A finding deferred to work that existed before the finding does not make that work "filed by review": it stays planned (vt8q shows 5 of 5 planned, with no Filed atelier-vy5i).
- What we set out to do states each success criterion in full, continuation lines included.
- Every text colour on the review page meets WCAG AA (4.5:1) on each surface it sits on in both palettes, enforced by a unit test; links in text are underlined; chips show a focus ring.
- Fixture and unit tests cover each of these.

## Context

The S4 validation at c73d55e8 found the renderer uses only Deviations, Surprises and Open items (crates/atelier-cli/src/commands/mission_review.rs report_facts and delivered_lines, and the HTML equivalents). In vt8q, 9 items record decisions only in that part, so they appear nowhere in the debrief. The S7 review at c73d55e8 added: atelier-vy5i shown as filed by review though it was planned; criteria cut to their first sentence; and AA contrast failures for faint, backlog, done and todo text and colour-only links.
