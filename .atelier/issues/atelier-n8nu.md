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

Define when orchestrators should use 5.4 Mini by default and when they should escalate to a higher-reasoning model. The policy should be explicit enough that future assignments include a model rationale.

## Outcome

- Routine bounded work defaults to 5.4 Mini: search, fixture repair, docs drift scans, transcript capture, focused tests, straightforward validation, and stale-test inventory.
- Higher-reasoning models are reserved for ambiguous architecture, cross-cutting refactors, hard debugging, and final adversarial closeout.
- Orchestrator prompts include model choice and reason.

## Evidence

- File-change review of Agent Factory guidance and repo binding docs shows the policy.
- Review artifact includes example assignments for the 5.4 Mini default and escalation cases.
- Docs check commands, `atelier lint`, and `atelier export --check` pass.
