---
created_at: "2026-06-15T15:17:32.624440938+00:00"
id: "atelier-nyn0"
issue_type: "task"
labels:
- "app-layer"
- "commands"
- "migration"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Migrate command workflows vertically into app CLI and storage crates"
updated_at: "2026-06-15T15:17:32.624440938+00:00"
---

## Description

Migrate command workflows by product surface so use-case logic lives in atelier-app, terminal rendering lives in atelier-cli, and storage calls go through atelier-sqlite or atelier-records. Old root command files should be deleted as each workflow surface moves.

## Outcome

- Command workflows are migrated vertically by product surface: issue/start/search, mission, plan/evidence/history, status/graph/workflow, worktree/branch, and init/import/export/rebuild/lint/doctor/maintenance/man.
- Use-case logic lives in `atelier-app`, terminal rendering lives in `atelier-cli`, and persistence calls go through `atelier-sqlite` or `atelier-records`.
- Old root command files are deleted as each workflow surface moves, with no compatibility command shims.

## Evidence

- Workflow-by-workflow proof maps each migrated command surface to app, CLI, and storage ownership.
- Representative transcripts for each command group prove visible command intent remains stable.
- Search or file-inventory proof shows old root command files are deleted after their workflow slices migrate.
