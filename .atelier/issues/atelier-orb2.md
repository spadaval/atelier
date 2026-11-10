---
created_at: "2026-11-10T00:56:09.122058912+00:00"
id: "atelier-orb2"
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
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Rewrite Agent Factory against the new tool, one file per role"
updated_at: "2026-11-10T00:56:09.122058912+00:00"
---

## Goal

Make the skill say how to decide and leave what to run to the tool: `SKILL.md` and one file each for lead, implementer, reviewer, decider and planner.

## Done when

- [ ] Provisional: no role file restates what `status` or `--help` prints; the constitution's live rules are in the role files and the rest is deleted; the agent-facing line count is reported against today's 1,219.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- Which of today's procedures (diagnose, validate, migrate, install) survive as their own files; the evals decide.
