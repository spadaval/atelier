---
created_at: "2026-06-12T19:29:16.395428888+00:00"
id: "atelier-7r55"
issue_type: "task"
labels:
- "assignee:root"
- "mission"
- "projection"
- "relationships"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ys5p"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Normalize mission relationship semantics"
updated_at: "2026-06-12T22:01:21.088495836+00:00"
---

## Description

Straighten out mission relationship semantics while the mission model changes.
Mission work, blockers, validation, evidence, checkpoints, and supporting
records should not depend on broad attachment buckets or "anything except a
blocker" filtering.

## Outcome

- Supported mission relationship types and buckets are documented and enforced.
- `atelier mission add-work`, blocker handling, validation links, evidence
  links, and status/show projections write and read explicit mission semantics.
- Mission status and show output count only the relationships intended for each
  section.
- Unknown or invalid mission relationships produce clear diagnostics or remain
  visibly outside the work queue.

## Evidence

- Tests proving unrelated mission attachments are not displayed as linked work.
- Tests or transcripts for mission work, blocker, validation, and evidence
  relationships.
- Documentation diff aligning product language, canonical storage, and command
  behavior.
