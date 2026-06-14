---
created_at: "2026-06-12T19:39:54.111802532+00:00"
id: "atelier-v6nd"
issue_type: "task"
labels:
- "agent-factory"
- "assignee:root"
- "process"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9pkx"
  - kind: "issue"
    id: "atelier-wws5"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-8fc1"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T21:25:55.304932055+00:00"
status: "done"
title: "Teach Agent Factory proof escalation rules"
updated_at: "2026-06-12T21:25:55.304932055+00:00"
---

## Description

Update Agent Factory so agents get one clear rule: ordinary work closes with
proof on the issue; risky, broad, or parent-level claims need an independent
check.
- Agent Factory guidance explains default proof and escalation in plain
  operational language.
- Orchestrator prompts name the proof expected for each assigned worker and
  create or block on validation issues only when independence is required.
- Implementers record the proof for their slice but do not act as independent
  validators for high-risk or parent-level claims.
- Review, validate, docs, and readiness subskills use the same proof language
  and failure classifications without competing terminology.
- Repository `AGENTFACTORY.md` is updated only where the local Atelier binding
  needs command-specific examples.
- Patch the relevant Agent Factory skill procedures and standards files under
  `/root/.agents/skills/agent-factory/`.
- Process review confirms the skill keeps ordinary issue proof lightweight and
  escalates risky, broad, or parent-level claims.
- Demonstrate the proof rule on at least one current `atelier-tcmr` issue or
  epic.
- Run relevant docs/process checks plus `atelier lint` and `atelier export
  --check`.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
