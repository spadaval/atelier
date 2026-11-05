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
title: "Show the report's Decisions part in the debrief"
updated_at: "2026-11-05T04:30:37.210222565+00:00"
---

## Goal

S4 of docs/strategies/review-and-introspection.md: the debrief shows each item's whole report, including its `### Decisions` part, in both forms.

## Done when

A delivered item's and a done child's non-empty Decisions part appear as a Decided fact in the Markdown and HTML debrief, a fixture test covers both, and `atelier debrief atelier-vt8q` shows the decisions vt8q's items reported (for example v0.3.0 over v0.2.0 in atelier-vy5i).

## Context

The S4 validation at c73d55e8 found the renderer uses only Deviations, Surprises and Open items (crates/atelier-cli/src/commands/mission_review.rs report_facts and delivered_lines, and the HTML equivalents). In vt8q, 9 items record decisions only in that part, so they appear nowhere in the debrief.
