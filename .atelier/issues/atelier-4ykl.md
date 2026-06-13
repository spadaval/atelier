---
created_at: "2026-06-13T20:55:09.157083606+00:00"
id: "atelier-4ykl"
issue_type: "task"
labels:
- "agent-readiness"
- "cli"
- "docs"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Replace Agent Factory command recipes with Atelier-owned command contracts"
updated_at: "2026-06-13T20:55:09.157083606+00:00"
---

## Description

Reduce AGENTFACTORY.md to binding and orchestration guidance by moving recurring command lists, setup checks, and recovery recipes into committed Atelier docs, help, or diagnostic commands.

## Outcome

- AGENTFACTORY.md stops being the durable command cookbook for normal tracker operation.
- Recurring command recipes, setup checks, and recovery procedures are represented by Atelier help, product docs, readiness checks, or focused diagnostics.
- Agent Factory binding remains short enough to route agents to the right Atelier-owned surfaces without duplicating their contracts.

## Evidence

- Diff review maps removed AGENTFACTORY.md command recipes to their new Atelier-owned destinations.
- Docs-command freshness check covers the moved command examples or explicitly scopes them out.
- `atelier lint`, `atelier export --check`, and the relevant docs/readiness check pass.
