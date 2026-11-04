---
created_at: "2026-11-04T23:25:59.599297599+00:00"
id: "atelier-hhce"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-hhce
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:33:16.022819452+00:00"
status: "done"
title: "Make mission success criteria optional and record missions as themed containers"
updated_at: "2026-11-04T23:33:16.022819452+00:00"
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

## Close Reason

A mission with only an Outcome and no strategy now passes `check`, the mission lint and the `ready` transition, and `issue create --issue-type mission` writes no placeholder criteria; `brief --html` drops its empty criteria tile, link and section, and the other mission views already omitted criteria. ADR 0024 records missions as themed containers with optional, achievable criteria and points at Agent Factory's Shaping a Mission and Ready Missions sections, which work-model.md, mission-authoring.md and the `ready` transition description now reference (PR #21).

### Deviations
Also corrected docs outside the named scope that said a mission requires criteria: CONTEXT.md, PRODUCT_INTENT.md, the record-store and bundle-schema docs, mission-reviews.md, human-cli-output.md, issue-inventory-and-mission-overview.md, and the `issue create --body` help text. The brief HTML fix touched crates/atelier-cli/src/commands/mission_review.rs.

### Decisions
Hid the criteria tile, contents link and section in `brief --html` only when a mission has neither criteria nor criteria problems, so a malformed criteria section still shows. Left the starter workflow policy for adopters with its generic `ready` description, because adopters may not use Agent Factory.

### Surprises
Only `brief --html` rendered anything for a criteria-less mission ("0 success criteria" tile, "Success Criteria: None defined."); the Markdown brief, debrief (both forms) and `work mission` were already clean.

### Open items
A strategy-linked mission whose strategy has no Success Criteria heading still shows a criteria problem in mission views; scope kept that lookup unchanged, and the ADR notes it. The Agent Factory links point at `main` in spadaval/.agents, where the two sections land when its `agent-factory/mission-shape` branch merges.
