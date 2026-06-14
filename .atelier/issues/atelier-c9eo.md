---
created_at: "2026-06-14T16:31:09.407778079+00:00"
id: "atelier-c9eo"
issue_type: "task"
labels:
- "docs"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-c4uz"
  - kind: "issue"
    id: "atelier-mllk"
  - kind: "issue"
    id: "atelier-wbed"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Define shell-mission proof contract before implementation"
updated_at: "2026-06-14T16:52:16.460203097+00:00"
---

## Description

Before implementation changes land, product, workflow, validation, and Agent Factory docs should define the shell-mission proof contract: missions coordinate linked work by default, and optional mission-level validation is explicit linked validation work or workflow approval.

## Outcome

- Product, validation, and Agent Factory guidance describe missions as
  coordination shells by default.
- Optional mission-level validation is modeled as linked validation work or a
  workflow approval, not as a pseudo-child closeout convention or embedded
  mission validator system.
- Guidance distinguishes local issue/epic proof from optional mission-level
  validation without duplicating proof requirements.
- The contract is explicit enough for implementation issues `atelier-c4uz`,
  `atelier-mllk`, and `atelier-wbed` to follow without inventing divergent
  closeout semantics.

## Evidence

- Docs diff updates `docs/product/work-model.md`,
  `docs/product/cli-surface.md`, `docs/architecture/quality/validation.md`, and
  `AGENTFACTORY.md` or records why a file does not need changes.
- File diff or review artifact shows documentation examples for linked
  validation work when explicit mission-level validation is needed.
- `atelier issue blocked atelier-c4uz`, `atelier issue blocked atelier-mllk`,
  and `atelier issue blocked atelier-wbed` show those implementation issues are
  blocked on this contract issue until it closes.
- `git diff --check` and `atelier lint` pass.
