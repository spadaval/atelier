---
created_at: "2026-06-19T03:58:38.908306324+00:00"
id: "atelier-tv53"
issue_type: "task"
labels:
- "config"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate review mode config and starter workflow guidance"
updated_at: "2026-06-19T03:58:38.908306324+00:00"
---

## Description

Validate the review mode configuration from the operator and starter-workflow
perspective. This issue is documentation and behavior validation, not the core
parser implementation.

## Outcome

- Starter workflow guidance shows a room-mode project and a provider-mode
  Forgejo project selecting exactly one mode.
- Config validation output for wrong-mode commands points to the appropriate
  `atelier review ...` action or config fix.
- Workflow docs and help agree on which review commands are room-only,
  provider-only, and shared.

## Evidence

- Transcript or CLI test output covers valid room config, valid provider config,
  wrong-mode room command, and wrong-mode provider command.
- Search output proves docs/help config examples use the new `[review]` shape.
- `atelier lint atelier-tv53` and relevant docs/help parity checks pass.
