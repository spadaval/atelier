---
created_at: "2026-11-09T03:59:22.001051041+00:00"
id: "atelier-cfq9"
issue_type: "feature"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Show a mission's sizing and derisking at the top of its brief"
updated_at: "2026-11-09T03:59:22.001051041+00:00"
---

## Goal

The human decides whether to approve a plan from its brief, but the brief shows nothing about the mission's cost, value or risk, and the planning reviewers' sizing sits at the bottom in review prose. Show the strategy's new Sizing and Derisking sections under the Outcome, and say plainly when they are missing, so an unsized plan is visible.

## Done when

- [ ] `atelier brief <mission>` (Markdown and `--html`) shows the linked strategy's Sizing and Derisking sections directly under the Outcome, read the way the brief already reads Risks and Unknowns (`strategy_section` in `crates/atelier-app/src/mission_review.rs`). Integration tests cover a strategy with both sections.
- [ ] A strategy missing either section, or a mission with no strategy, shows "Sizing: not written" or "Derisking: not written" in that place; a test covers it. Nothing refuses.
- [ ] `docs/product/mission-reviews.md` describes the two sections in the brief.

## Scope

The brief only. Not the debrief, `atelier check`, or the site.
