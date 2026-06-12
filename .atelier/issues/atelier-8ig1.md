---
created_at: "2026-06-11T20:28:37.466311841+00:00"
id: "atelier-8ig1"
issue_type: "validation"
labels:
- "agent-factory"
- "mission"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate Agent Factory orchestrator loop end to end"
updated_at: "2026-06-11T21:17:57.101793541+00:00"
---

## Description

Scenario validation for the expanded autonomous mission loop. Prove an
orchestrator can start or focus a mission, inspect option-oriented mission/epic
status, choose among ready work with context, delegate a ready issue, create or
locate the worktree/branch, start tracked work, run workflow validators, attach
evidence, complete or close work through transition commands, see
blockers/evidence gaps, use an explicit waiver when configured, and close out
with durable proof.

## Outcome

real command transcript or integration tests cover strict mission
work and lightweight non-mission issue work; status presents multiple options
with useful context rather than one prescriptive answer; missing evidence blocks
completion with actionable guidance; waiver behavior is explicit and durable;
every non-pass is classified with a follow-up owner; Agent Factory guidance and
Atelier CLI behavior agree.

## Evidence

Evidence was not specified in the legacy issue record.
