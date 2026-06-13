---
created_at: "2026-06-13T02:36:07.173901476+00:00"
id: "atelier-n8nu"
issue_type: "task"
labels:
- "agent-factory"
- "cost"
- "delegation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-29yn"
  - kind: "issue"
    id: "atelier-dxy1"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Document Agent Factory model routing policy"
updated_at: "2026-06-13T02:36:07.173901476+00:00"
---

## Description

Define when orchestrators should consider 5.4 Mini and when they should use or escalate to a higher-reasoning model. The policy should be explicit enough that future assignments include a model rationale grounded in task complexity, ambiguity, risk, and review depth.

## Outcome

- 5.4 Mini is recommended only for bounded, low-ambiguity, low-risk work such as basic behavior validation, search, fixture repair, docs drift scans, transcript capture, focused tests, straightforward validation, stale-test inventory, and basic refactor-style implementation.
- Higher-reasoning models are required for complex open-ended implementation, complex review, ambiguous architecture, cross-cutting refactors, hard debugging, and final adversarial closeout.
- Orchestrator prompts include model choice and reason, including why a Mini model is capable enough when one is selected.

## Evidence

- File-change review of Agent Factory guidance and repo binding docs shows the policy.
- Review artifact includes example assignments where 5.4 Mini is appropriate, where it is rejected as underpowered, and where escalation is required.
- Docs check commands, `atelier lint`, and `atelier export --check` pass.
