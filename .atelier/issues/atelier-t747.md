---
created_at: "2026-11-10T00:56:09.383226546+00:00"
id: "atelier-t747"
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
    id: "atelier-1oel"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-orb2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Run the skill against no skill on Claude Opus and Codex, and cut what no scenario supports"
updated_at: "2026-11-10T01:14:41.759781943+00:00"
---

## Goal

Give every kept procedure an eval scenario it changed, and remove the rest.

## Done when

- [ ] Provisional: the mission criterion `skill-earns-its-place`.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- Whether three runs per cell separate the arms. The strategy's default is to cut a procedure whose scenario shows no difference. Planning review finding atelier-395f argues that three tied runs are inconclusive, and that duplicated text should be cut first and judgment rules only on better evidence. Settle which rule applies when this step is planned.
