---
created_at: '2026-06-11T00:06:58.494243072+00:00'
id: atelier-ny4e
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-0001
    type: advances
  - kind: issue
    id: atelier-0007
    type: advances
  - kind: issue
    id: atelier-0008
    type: advances
  - kind: issue
    id: atelier-000l
    type: advances
  - kind: issue
    id: atelier-000r
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-11T19:54:26.488136007+00:00'
status: closed
title: Workflow policy and validators
updated_at: '2026-06-11T19:54:26.488136007+00:00'
---

## Description

Superseded by mission atelier-n8ag, Autonomous mission operations.

Original intent preserved: deliver risk-scaled workflow policy for Atelier: configurable workflows, validator-backed transitions, lint severities, waivers, action-aware guidance, and closeout validation for strict and lightweight flows.

The executable workflow-policy work is now linked directly to atelier-n8ag so agents have one durable mission authority for autonomy-core work.

## Outcome

### Constraints

- Tiny tasks remain lightweight by default while higher-risk workflows can require evidence, validators, and explicit waivers.

### Risks

- Over-strict workflow policy could make ordinary agent work feel like red tape, while under-specified policy could let closeout quality drift.

## Evidence

- Manual check: Linked issues prove configurable workflows, transition validation, lint severities, waivers, and Milestone 5 closeout validation.

## Notes

Migrated from `.atelier/missions/atelier-ny4e.md` as a declared mission objective issue.
