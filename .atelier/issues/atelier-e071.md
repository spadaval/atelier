---
created_at: "2026-06-20T16:54:08.882641410+00:00"
id: "atelier-e071"
issue_type: "task"
labels:
- "artifact-update"
- "cutting-pass"
- "mission-collapse"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-19xa"
  - kind: "issue"
    id: "atelier-439j"
  - kind: "issue"
    id: "atelier-db6z"
  - kind: "issue"
    id: "atelier-djoq"
  - kind: "issue"
    id: "atelier-ybz1"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T19:23:48.061685329+00:00"
status: "done"
title: "Define mission-as-type target command contract"
updated_at: "2026-06-20T19:23:48.061685329+00:00"
---

## Description

Document the target command contract for representing missions as typed
objective issues instead of a parallel mission command namespace.

Resolve the product decisions before implementation:

- Which mission commands are removed, retained temporarily, or replaced.
- Whether the mission status view lands at `issue status <id>`, type-aware
  `issue show <id>`, or another general surface.
- How mission sections map to issue-type section schemas.
- How transition notes replace mission close reasons.
- Why mission focus/start is removed instead of renamed.

## Outcome

- Product docs and command audit describe the target mission-as-type command
  model.
- Replacement command names and removal order are explicit enough for
  implementation issues to proceed.
- Dependent implementation issues are blocked on this decision artifact.

## Evidence

- Updated `docs/product/command-audit/mission.md` and related command-audit
  pages show the target model.
- `target/debug/atelier issue show atelier-4h62` shows dependent
  implementation issues blocked by this artifact-update issue.
