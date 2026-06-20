---
created_at: "2026-06-19T22:54:23.408624399+00:00"
id: "atelier-z7vb"
issue_type: "task"
labels:
- "actions"
- "config"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0d5k"
  - kind: "issue"
    id: "atelier-qx40"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Move review provider policy into workflow actions"
updated_at: "2026-06-20T00:05:23.702621290+00:00"
---

## Description

Move review-provider policy that affects transition behavior from `.atelier/config.toml` into workflow action configuration. The project config may still select a concrete backend and non-secret connection defaults, but action-owned details such as which review provider action runs, branch source/target templates, and role attribution used by workflow actions should be visible from `.atelier/workflow.yaml`.

## Outcome

- A reader can understand review transition behavior from the workflow file.
- Secrets remain outside tracked files, and project-specific backend selection remains explicit.

## Evidence

- .atelier/workflow.yaml file change shows review action parameters for Forgejo-backed review open/link behavior.
- .atelier/config.toml file change removes workflow-action-specific Forgejo policy that moved into workflow actions.
- CLI integration tests prove review.open action reads workflow action configuration and still uses env vars for secrets.
