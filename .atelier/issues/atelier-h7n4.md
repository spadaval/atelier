---
created_at: "2026-06-19T22:54:23.405637605+00:00"
id: "atelier-h7n4"
issue_type: "task"
labels:
- "config"
- "workflow-policy"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-cko9"
  - kind: "issue"
    id: "atelier-jwvd"
  - kind: "issue"
    id: "atelier-qx40"
  - kind: "issue"
    id: "atelier-z7vb"
  - kind: "issue"
    id: "atelier-zu0t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T23:40:34.268903290+00:00"
status: "done"
title: "Define config and workflow policy ownership boundary"
updated_at: "2026-06-19T23:40:34.268903290+00:00"
---

## Description

Define the target boundary between tracked project config and workflow policy. Keep project-specific backend selection allowed in tracked config, but decide which review-provider settings move into workflow action parameters versus remain in `.atelier/config.toml`. Also define which local runtime/cache/path settings must not remain committed project policy.

## Outcome

- A durable artifact states what belongs in `.atelier/config.toml`, what belongs in `.atelier/workflow.yaml`, and what belongs only in local runtime or environment.
- Dependent schema, docs, and workflow migration work are blocked until this boundary is settled.

## Evidence

- ADR or docs/product/workflow-configuration.md file change defines workflow-owned review action configuration.
- docs/product/development-setup.md or config documentation file change defines local runtime/cache ownership.
- Manual check of .atelier/config.toml and .atelier/workflow.yaml target examples confirms provider backend, runtime path, and workflow action boundaries are explicit.
