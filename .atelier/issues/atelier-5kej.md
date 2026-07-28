---
created_at: "2026-07-28T16:39:31.758419104+00:00"
id: "atelier-5kej"
issue_type: "mission"
labels:
- "agent-factory"
- "mission-control"
- "planning"
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
status: "ready"
title: "Align Atelier with adaptive Agent Factory missions"
updated_at: "2026-07-28T16:40:25.246916410+00:00"
---

## Description

Governing strategy: `docs/strategies/agent-factory-adaptive-missions.md` revision 1.

Atelier should remain a pleasant interface over planning files while supporting Agent Factory's strategic plans, outcome-oriented epics, evidence-bounded task horizons, routine replanning, and just-in-time expansion. Product intent belongs in strategy and outcome records; the issue graph is the current implementation hypothesis.

This mission does not add plan approval, planning identities, graph freezes, completeness gates, or a first-class plan lifecycle.

## Outcome

- A Manager can discover the active strategy revision, distinguish durable product outcomes from the current executable task horizon, and identify why an unexpanded epic should expand.
- A Manager can revise issues, dependencies, sequencing, and obsolete speculative work as evidence changes without an authorization ceremony or frozen whole-mission graph.
- Atelier's product contract, CLI guidance, and repository-owned Agent Factory binding agree on strategic replanning and just-in-time expansion.
- End-to-end scenarios demonstrate the adaptive mission loop without turning Atelier into an approval or portfolio-management system.

## Evidence

Evidence was not specified in the bundle.
