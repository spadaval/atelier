---
created_at: "2026-06-13T20:37:10.204903692+00:00"
id: "atelier-2ehd"
issue_type: "task"
labels:
- "architecture"
- "refactor"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Split RecordStore god module by cohesive ownership"
updated_at: "2026-06-13T20:37:10.204903692+00:00"
---

## Description

RecordStore currently owns record kind registry, issue sections, mission sections, generic domain records, parsing, rendering, validation, relationship mutation, ID allocation, atomic writes, migration normalization, and tests in one large module. Split or reconnect those responsibilities around durable record ownership.

## Outcome

- Shared record primitives remain central, while issue/mission/evidence/plan parsing and rendering live behind cohesive record-kind modules.
- Relationship mutation and cycle checks have an explicit domain service boundary instead of being scattered through render/store code.
- Tests target parser/render/validation behavior by record kind and no longer require one omnibus module to understand every record.

## Evidence

- Architecture diff or review artifact shows the new module boundary and dependency direction.
- Focused parser/render/rebuild tests pass for issue, mission, plan, and evidence fixtures.
- cargo fmt -- --check and relevant cargo nextest slices pass.
