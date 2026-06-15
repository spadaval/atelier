---
created_at: "2026-06-15T15:16:23.282843597+00:00"
id: "atelier-4ren"
issue_type: "task"
labels:
- "app-layer"
- "view-model"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T18:33:50.254471183+00:00"
status: "done"
title: "Return request outcome and view model APIs from atelier-app"
updated_at: "2026-06-15T18:33:50.254471183+00:00"
---

## Description

Make atelier-app expose explicit command request, outcome, and view-model types for use-case orchestration, and remove direct println/eprintln behavior from application code.

## Outcome

- `atelier-app` exposes command request, outcome, and view-model types for migrated use cases.
- Application orchestration code does not call `println!` or `eprintln!`; terminal presentation is rendered by `atelier-cli`.
- Workspace discovery, command storage access policy, diagnostics coordination, Git/process coordination, and command use-case orchestration live in `atelier-app` where they are not CLI rendering concerns.

## Evidence

- Search transcript proves no direct `println!` or `eprintln!` calls remain in `atelier-app` application orchestration code.
- Targeted tests or transcripts exercise at least one migrated command through request/outcome/view APIs rendered by `atelier-cli`.
- Code inspection or focused tests prove workspace discovery and storage policy are reachable through `atelier-app` APIs rather than root command modules.
