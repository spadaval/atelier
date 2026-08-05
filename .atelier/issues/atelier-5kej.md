---
created_at: "2026-07-28T16:39:31.758419104+00:00"
id: "atelier-5kej"
issue_type: "mission"
labels:
- "agent-factory"
- "mission-control"
- "planning"
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-5kej
    owner_kind: mission
    review_target: master
    work_branch: mission/atelier-5kej
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6qah"
    type: "advances"
  - kind: "issue"
    id: "atelier-clne"
    type: "advances"
  - kind: "issue"
    id: "atelier-k7mc"
    type: "advances"
  - kind: "issue"
    id: "atelier-zm6u"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
strategy:
  path: "docs/strategies/agent-factory-adaptive-missions.md"
  revision: 2
title: "Align Atelier with adaptive Agent Factory missions"
updated_at: "2026-08-05T18:02:11.358862444+00:00"
---

## Description

Atelier should remain a pleasant interface over planning files while supporting Agent Factory's strategic plans, outcome-oriented epics, evidence-bounded task horizons, routine replanning, and just-in-time expansion. Product intent belongs in strategy and outcome records; the issue graph is the current implementation hypothesis.

This mission does not add plan approval, planning identities, graph freezes, completeness gates, or a first-class plan lifecycle.

## Outcome

- A Manager can discover the active strategy revision, distinguish the draft outcome map from the current executable horizon, and expand only the next justified epic.
- A Manager can revise issues, dependencies, sequencing, and obsolete speculative work as evidence changes without an authorization ceremony or frozen whole-mission graph.
- Atelier's product contract, CLI guidance, and repository-owned Agent Factory binding agree on strategic replanning and just-in-time expansion.
- End-to-end scenarios demonstrate the adaptive mission loop without turning Atelier into an approval or portfolio-management system.

## Evidence

- The strategy's Assurance scenarios are demonstrated from a fresh repository state.
- Focused workflow and CLI tests prove draft epics remain visible without entering ready work.
- Independent validation checks the completed mission against Atelier's anti-red-tape boundary.
