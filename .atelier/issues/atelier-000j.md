---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000j"
issue_type: "epic"
labels:
- "domain-model"
- "epic"
- "milestone"
- "spec"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000r"
  children:
  - kind: "issue"
    id: "atelier-0005"
  - kind: "issue"
    id: "atelier-000h"
  - kind: "issue"
    id: "atelier-000u"
  - kind: "issue"
    id: "atelier-001n"
  - kind: "issue"
    id: "atelier-0022"
  - kind: "issue"
    id: "atelier-0024"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T14:41:58.783598154+00:00"
status: "done"
title: "Milestone 4: First-class records and typed links"
updated_at: "2026-06-11T14:41:58.783598154+00:00"
---

## Description

Introduce first-class missions, milestone checkpoint records, plans, evidence, workflow validators, and typed links so generic issues stop carrying every durable concept. Preserve existing issue behavior where useful, but make mission intent, checkpoint state, execution plans, validation proof, workflow transition checks, and semantic links explicit records or commands.

Milestones are validated target states, not work containers or super-epics. Workflow validators belong to workflow policy and enforce transitions; milestone records own validation criteria.
Data model and CLI support first-class records for missions, milestone checkpoints, plans, evidence, workflow validators, and typed links, or explicitly staged subsets. Issues are no longer the only durable representation for non-issue concepts. Compatibility migration paths are defined where reasonable. Tests cover persistence, links, export/rebuild, JSON output, evidence validation, and workflow validator results.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
