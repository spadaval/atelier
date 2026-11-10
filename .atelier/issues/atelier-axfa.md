---
created_at: "2026-11-10T00:56:08.066927831+00:00"
id: "atelier-axfa"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add `escalate` and `decide`, rebuttals in the brief, and `list --deferred`"
updated_at: "2026-11-10T00:56:08.066927831+00:00"
---

## Goal

Give a disagreement between implementer and reviewer its three exits in the tool.

## Done when

- [ ] Provisional: scenario tests cover a rebuttal the next reviewer withdraws, one it keeps, and an escalation a decider answers; `list --deferred` shows deferred findings across done items.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- How `status` shows an unanswered question to the lead.
