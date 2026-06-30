---
created_at: "2026-06-29T20:16:42.089703500+00:00"
id: "atelier-6knt"
issue_type: "task"
labels:
- "command-audit"
- "complexity"
- "docs"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Keep command audit guidance decision-oriented"
updated_at: "2026-06-29T20:16:42.089703500+00:00"
---

## Description

The refreshed command audit should enforce a complexity budget without becoming another high-paperwork process. Audit entries should state the operator question, the role that asks it, product/cognitive complexity, architecture/code complexity, budget verdict, and next action, not generic summaries or sprawling narratives.

## Outcome

Command-audit guidance tells reviewers to record concise Keep, Simplify, Fold, Hide, or Remove decisions tied to operator value and implementation cost. Existing audit pages follow that format closely enough that future workers can act without reinterpreting generic summaries, and `docs/product/zen.md` plus command-audit docs agree that product complexity and architecture complexity are separate costs.
