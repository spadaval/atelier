---
created_at: "2026-10-15T19:31:02.027687676+00:00"
id: "atelier-v44v"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-v44v
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:26:55.654093135+00:00"
status: "done"
title: "Small CLI rough edges from rebuild validation"
updated_at: "2026-10-16T19:26:55.654093135+00:00"
---

## Goal

Resolve or consciously accept the minor issues found by independent validation.

## Done when

Each item below is fixed with a test or recorded as accepted: (1) land does not refuse uncommitted .atelier/config.toml edits; (2) atelier setup leaves settings.json at mode 0600; (3) issue create accepts a mission with neither a strategy link nor success criteria, which check then flags; (4) review subcommands lack help descriptions; (5) two test doc comments still mention export/import.

## Context

Independent S1-S8 validation 2026-10-15 (atelier-xz11).

## Design

Resolution of each item (2026-10-16):

1. Accepted. `land` does not refuse uncommitted `.atelier/config.toml` edits because config.toml is a local Atelier path by design (`is_local_atelier_path`, with runtime files, locks and agent.json), and committed-snapshot tests pin that it is not a merge input. It holds the slug, prune defaults and custom context-only link types, none of which bypass a gate.
2. Fixed. `atelier setup` keeps an existing settings file's mode and creates a new one at 0644; the temporary file it writes through started at 0600. Test: setup_keeps_an_existing_settings_mode_and_creates_new_ones_readable.
3. No longer reproduces. `issue create --issue-type mission` with no strategy or success criteria now passes `check` too, so create and check agree (mission sections and the strategy link were reworked in atelier-9vq3 and atelier-9m4k).
4. Fixed. Every `atelier review` subcommand and argument has help text.
5. Fixed. The two test doc comments no longer mention export/import.

## Close Reason

Each item is fixed with a test or recorded: (1) uncommitted config.toml is accepted as a local path by design; (2) setup keeps the settings mode, tested; (3) mission create and check now agree, no longer reproduces; (4) review subcommands have help; (5) export/import doc comments fixed. See Design.
