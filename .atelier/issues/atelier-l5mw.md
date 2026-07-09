---
created_at: "2026-07-06T20:37:49.363294325+00:00"
id: "atelier-l5mw"
issue_type: "task"
labels:
- "agent-factory"
- "mission-review"
- "subskill-implement"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-qi40"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Add the Agent Factory mission-review subskill"
updated_at: "2026-07-09T16:55:49.192400350+00:00"
---

## Description

Assigned subskill: implement. Add `mission-review` as the independent counterpart to `plan`. Define its inputs as an exact mission draft and complete reachable graph; its read-only stance; its coverage, dependency, decomposition, sequencing, external-prerequisite, validation, and initial-parallelism rubric; and its output as blocking findings or approval. Keep code `review` and scenario `validate` responsibilities separate.

## Outcome

- Agent Factory has an explicit `mission-review` routing rule and procedure that a fresh subagent can execute without loading the code-review procedure as a substitute.
- The procedure attempts to falsify readiness, reports findings with affected issue IDs and dependency paths, and approves only the exact inspected graph revision.
- The mission reviewer does not edit the authored graph or implement fixes unless separately reassigned.

## Evidence

- A dogfood reviewer applies the new procedure to an exact complete mission graph and records revision-bound approval, then applies it to defective graphs and reports blocking findings with affected issue IDs and dependency paths.
- Routing tests or focused skill inspection show `mission-review` is selectable independently from `plan`, code `review`, `orchestrate`, and `validate`, and its assignment template requires repository, mission ID, exact revision, complete graph scope, evidence destination, and independence context.
- A negative dogfood case shows the material author cannot approve the graph and the reviewer does not edit it; requested repairs return to a separately assigned planner.
