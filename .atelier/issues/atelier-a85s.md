---
created_at: "2026-06-14T05:58:17.170336917+00:00"
id: "atelier-a85s"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4yrt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Fold workflow init into root init"
updated_at: "2026-06-14T05:58:17.170336917+00:00"
---

## Description

Root init creates workflow policy and uses the repo-local Beads migration flag
owned by `atelier-vj08`; workflow init is removed from normal and hidden
operator paths.

## Outcome

Root `atelier init` creates the starter workflow policy as part of normal
tracker setup. `workflow init` is removed from normal and hidden operator
paths. The `atelier init --import-beads` behavior provided by `atelier-vj08`
is integrated into the setup flow without duplicating the migration surface.

## Evidence

- `atelier init --help` documents workflow setup and `--import-beads`.
- Fresh init transcript shows `.atelier/workflow.yaml` created and `atelier
  lint` passing before issue creation.
- Fixture or transcript with `.beads/issues.manual.jsonl` shows
  `atelier init --import-beads` imports records explicitly.
- Root help and `atelier workflow --help` do not route normal setup through
  `workflow init`.
- `git diff --check`, `atelier lint`, and focused init/import tests pass.
