---
created_at: "2026-06-13T20:55:07.102444536+00:00"
id: "atelier-a0fd"
issue_type: "task"
labels:
- "agent-readiness"
- "artifact-update"
- "process-boundary"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3iom"
  - kind: "issue"
    id: "atelier-4u5h"
  - kind: "issue"
    id: "atelier-4ykl"
  - kind: "issue"
    id: "atelier-i9ob"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-p2m2"
    type: "related"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T22:52:30.456745416+00:00"
status: "done"
title: "Define Atelier and Agent Factory ownership boundary"
updated_at: "2026-06-13T22:52:30.456745416+00:00"
---

## Description

Decide which recurring orchestration rules belong in Atelier product behavior, workflow policy, docs, help, or validators, and which should remain Agent Factory delegation guidance.

## Outcome

- AGENTFACTORY.md is classified into clear buckets: repository binding, orchestration-only guidance, and Atelier-owned product/process behavior.
- Product-owned behavior has destination decisions: CLI help, product docs, workflow policy, lint, mission status, mission audit, doctor, or readiness checks.
- The boundary explicitly keeps role assignment, subskill selection, model routing, and independent-review judgment in Agent Factory unless Atelier gains first-class assignment metadata.

## Evidence

- Artifact update records the ownership matrix and names the source sections moved or intentionally retained.
- `atelier issue show` review confirms follow-up implementation issues are blocked on this decision where behavior or documentation would otherwise diverge.
- `atelier lint atelier-a0fd` and `atelier export --check` pass after the boundary artifact is committed.
