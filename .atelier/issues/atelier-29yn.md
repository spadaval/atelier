---
created_at: "2026-06-13T02:36:09.470158989+00:00"
id: "atelier-29yn"
issue_type: "validation"
labels:
- "agent-factory"
- "delegation"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Dogfood delegation policy on a bounded multi-agent run"
updated_at: "2026-06-13T02:36:09.470158989+00:00"
---

## Description

Validate the new delegation and model-routing policy by using it on a small bounded run. The run should include at least one assignment judged suitable for 5.4 Mini by the orchestrator and one explicit escalation or no-escalation decision.

## Outcome

- Assignment prompts include model choice, model rationale, tracker IDs, role, owned scope, and expected evidence.
- Resulting handoffs include evidence-producing output that an orchestrator can inspect without private chat context.
- The validation identifies whether the policy reduces main-agent work or creates new coordination overhead.

## Evidence

- Evidence record attached to this validation issue includes assignment summaries, model choices, outputs inspected, and follow-up findings.
- File-change review or tracker note records any policy changes needed after dogfooding.
- `atelier lint`, `atelier export --check`, and any focused checks used during the dogfood run pass or are recorded with residual risk.
