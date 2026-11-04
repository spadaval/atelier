---
created_at: "2026-11-04T23:25:59.599297599+00:00"
id: "atelier-hhce"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make mission success criteria optional and record missions as themed containers"
updated_at: "2026-11-04T23:25:59.599297599+00:00"
---

## Goal

Missions stall when given success criteria their work cannot reach (transcript counts, waits on adopters); 29 of 54 criteria only restate their items. Make a mission a themed container whose success criteria are optional, and require any criterion to be achievable and checkable by the mission's own work at close.

## Done when

- An ADR records the decision and points to Agent Factory's Shaping a Mission and Ready Missions sections for the process.
- `atelier check` and the mission lint accept a mission with neither Success criteria nor a strategy; `issue create --issue-type mission` with only an Outcome writes no placeholder criteria. Tests cover both.
- `brief`, `debrief` and `work mission` render a mission without criteria without empty or misleading sections (shown by a test or captured output).
- work-model.md, mission-authoring.md and the `ready` transition description in `.atelier/workflow.yaml` match.

## Scope

crates/atelier-app/src/lint.rs, crates/atelier-core/src/lib.rs (mission body defaults), their tests, docs/adr, docs/product. The criteria parser, evidence `--criterion` and criterion marks stay as they are.
