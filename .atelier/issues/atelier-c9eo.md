---
created_at: "2026-06-14T16:31:09.407778079+00:00"
id: "atelier-c9eo"
issue_type: "task"
labels:
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
title: "Update proof and mission closeout docs for shell missions"
updated_at: "2026-06-14T16:31:09.407778079+00:00"
---

## Description

Product and Agent Factory docs should describe missions as coordination shells and route optional mission-level validation into linked validation work.

## Outcome

- Product, validation, and Agent Factory guidance describe missions as
  coordination shells by default.
- Optional mission-level validation is modeled as linked validation work or a
  workflow approval, not as a pseudo-child closeout convention or embedded
  mission validator system.
- Guidance distinguishes local issue/epic proof from optional mission-level
  validation without duplicating proof requirements.

## Evidence

- Docs diff updates `docs/product/work-model.md`,
  `docs/product/cli-surface.md`, `docs/architecture/quality/validation.md`, and
  `AGENTFACTORY.md` or records why a file does not need changes.
- File diff or review artifact shows documentation examples for linked
  validation work when explicit mission-level validation is needed.
- `git diff --check` and `atelier lint` pass.
