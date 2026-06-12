---
created_at: "2026-06-08T19:13:32+00:00"
id: "atelier-000s"
issue_type: "task"
labels:
- "assignee:root"
- "config"
- "milestone"
- "spec"
- "task"
- "validator"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000l"
  - kind: "issue"
    id: "atelier-000n"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define repository-owned workflow configuration contract"
updated_at: "2026-06-09T20:09:55.000848680+00:00"
---

## Description

Design Atelier's external workflow configuration contract before implementing validator-backed transitions. Scope includes choosing the repo-owned config path, schema shape, strict parse/validation behavior, environment-variable indirection rules, hook definitions, workflow validator definitions, dynamic reload expectations, and how workflow-specific guidance is rendered. Use Symphony's WORKFLOW.md model as prior art without assuming its daemon or Linear-specific scheduler model.

Out of scope: direct coding-agent process supervision, durable agent-run rows, retry queues, or live session metrics.

## Outcome

A documented contract names the config path and schema; invalid config cases have stable error names; environment expansion only happens through explicit opt-in values; hooks have timeout and failure semantics; workflow validators have stable names, input context, pass/fail result shape, and actionable failure messages; dynamic reload or reload-check behavior is specified; examples cover tiny-task and stricter milestone workflows; docs explicitly defer direct agent-run management.

## Evidence

Evidence was not specified in the legacy issue record.
