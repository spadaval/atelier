---
created_at: "2026-06-13T02:52:24.818111700+00:00"
id: "atelier-n9up"
issue_type: "task"
labels:
- "evidence"
- "product"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0vjq"
  - kind: "issue"
    id: "atelier-dv3d"
  - kind: "issue"
    id: "atelier-rzsg"
  - kind: "issue"
    id: "atelier-s8z0"
  - kind: "issue"
    id: "atelier-xmss"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T04:07:29.982799344+00:00"
status: "done"
title: "Define accountable-work evidence target model"
updated_at: "2026-06-13T04:07:29.982799344+00:00"
---

## Description

Define where evidence should attach. Missions are objectives, not work, so normal evidence should attach to accountable work such as implementation, validation, review, or closeout issues. Mission readiness should derive from linked work and its evidence instead of direct mission proof attachments.

## Outcome

- Evidence policy says normal evidence attaches to accountable work, not directly to mission objectives.
- Mission status and closeout expectations derive proof from linked validation, closeout, and implementation work.
- Any remaining direct mission evidence behavior is classified as legacy, migration-only, or intentionally retained with a narrow reason.

## Evidence

- File-change review of validation, evidence, and mission docs shows the target model.
- Review artifact lists current commands or tests that attach evidence to missions and assigns update or compatibility handling.
- `atelier lint`, `atelier export --check`, and focused docs checks pass.
