---
created_at: "2026-06-14T16:31:03.958530812+00:00"
id: "atelier-mllk"
issue_type: "task"
labels:
- "assignee:root"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Make mission closeout aggregate linked work only"
updated_at: "2026-06-14T17:22:25.891331258+00:00"
---

## Description

Mission close should be ready when linked work is done, mission blockers are clear, and configured health gates pass; mission Validation prose is guidance, not a coded evidence contract.

## Outcome

- Mission closeout is ready when mission-linked work is done, mission blockers
  are clear, configured health validators pass, and any configured workflow
  approval has completed.
- Mission `Validation` prose remains useful guidance for humans and validators,
  but it is not parsed as a coded evidence contract.
- `mission audit` is removed as a required close gate or converted to
  non-blocking orientation consistent with shell missions.

## Evidence

- Focused mission closeout test or transcript shows a mission closing with all
  linked work done and blockers clear without mission-level token matching.
- A negative test or transcript shows an open linked issue or mission blocker
  still blocks mission close.
- File diff in `docs/product/work-model.md`, `docs/product/cli-surface.md`, or
  `docs/product/workflow-configuration.md` describes missions as coordination
  shells.
- `git diff --check` and `atelier lint` pass.
