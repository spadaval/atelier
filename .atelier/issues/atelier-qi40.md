---
created_at: "2026-07-06T20:37:49.372239276+00:00"
id: "atelier-qi40"
issue_type: "task"
labels:
- "agent-factory"
- "mission-review"
- "orchestration"
- "subskill-implement"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "blocked"
title: "Separate mission authorship, review, and orchestration handoffs"
updated_at: "2026-07-16T18:48:47.884962040+00:00"
---

## Description

Assigned subskill: implement. Update Agent Factory plan, orchestrate, install, readiness, assignment, and repository-shape guidance as needed. A planner creates or repairs the draft and requests review; a separately assigned mission reviewer inspects it; an orchestrator begins execution only from Atelier-reported ready state. Route tactical commands and lifecycle policy to Atelier-owned surfaces rather than duplicating them in the skill.

## Outcome

- Agent Factory cannot interpret completion of `plan` as authority to mark the same draft ready, and mission orchestration requires current independent approval reported by Atelier.
- Delegated mission-review assignments include mission ID, repository and revision context, complete graph scope, expected findings or approval destination, and independence requirements.
- Installation and readiness checks identify repositories whose lifecycle cannot support the required separation instead of claiming full operability.

## Evidence

- Dogfood handoffs record a planner leaving an authored graph in draft, a separately assigned mission reviewer inspecting the exact revision, and an orchestrator proceeding only after Atelier reports current approval and ready state.
- Assignment inspection confirms mission-review handoffs include repository/mission/revision context, all reachable issue IDs or an equivalent complete-graph selector, expected finding/approval destination, and author/editor independence information.
- Installation/readiness fixtures for a repository with and without lifecycle support show the supported case routes through Atelier commands and the unsupported case reports the missing capability instead of claiming operability.
