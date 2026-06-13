---
created_at: "2026-06-13T20:44:46.646892265+00:00"
id: "atelier-c9ej"
issue_type: "task"
labels:
- "docs"
- "readiness"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-4ykl"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add docs and agent-command freshness checks"
updated_at: "2026-06-13T20:55:09.157083606+00:00"
---

## Description

Agent-facing docs are only useful when their commands still run. Add a lightweight validation path for AGENTS.md, AGENTFACTORY.md, docs command examples, and common setup/check commands so stale guidance is caught before handoff.

## Outcome

- A documented check or script validates the key agent-facing commands from AGENTS.md and AGENTFACTORY.md.
- The validation path is fast enough for routine handoff and clear about what it does not cover.
- Stale command examples produce actionable failures rather than quiet drift.

## Evidence

- File change introduces or documents the docs-command freshness check.
- Command transcript shows the freshness check passing and demonstrates at least one representative checked command.
- `atelier lint`, `atelier export --check`, and relevant focused tests pass.
