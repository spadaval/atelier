---
created_at: "2026-06-13T02:33:44.652636399+00:00"
id: "atelier-ey4y"
issue_type: "epic"
labels:
- "agent-factory"
- "cost"
- "delegation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-29yn"
  - kind: "issue"
    id: "atelier-dxy1"
  - kind: "issue"
    id: "atelier-n8nu"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Improve Agent Factory delegation and model routing"
updated_at: "2026-06-13T02:36:09.470158989+00:00"
---

## Description

Update Agent Factory orchestration so subagents are used earlier for bounded evidence-producing slices, and so cheaper/faster models are used only when the assignment fits their capability. The policy should reduce main-agent token burn without losing implementation, review, or independent validation quality.

## Outcome

- Delegation guidance says when to use subagents early for scouting, fixture repair, docs drift checks, transcript capture, focused validation, and stale-test inventory.
- Model routing identifies 5.4 Mini as suitable for carefully bounded, low-ambiguity work such as basic behavior validation, transcript capture, fixture repair, and straightforward refactor-style implementation when the orchestrator judges the task fit is safe.
- Model routing reserves higher-reasoning models for complex open-ended implementation, complex review, ambiguous architecture, cross-cutting refactors, hard debugging, and final adversarial closeout.
- Worker prompts require exact tracker IDs, one role, owned files or workflows, proof expected, model rationale, and independence requirements.
- Orchestrators can prove delegation quality through evidence-producing handoffs instead of private summaries.

## Evidence

- Agent Factory guidance and repo binding docs describe model routing and bounded delegation rules.
- A dogfood validation run or transcript shows assignments using the new policy and records whether the chosen model was appropriate.
- Review confirms the policy does not weaken independent validation for high-risk closeout.
