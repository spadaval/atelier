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

1. Fixed (review finding atelier-9kwv showed the first acceptance was wrong). config.toml stays a local Atelier path for snapshot reads, but gates read its custom context link types and prune retention from disk, so an uncommitted custom link type let records the committed config rejects pass `land` and `check`. `land` and `merge` now refuse a config.toml that differs from HEAD, including an ignored or untracked one, with `land_config_uncommitted` / `merge_config_uncommitted` (merge refuses a modified tracked one as a dirty checkout first). Test: land_and_merge_refuse_a_project_config_that_differs_from_head.
2. Fixed. `atelier setup` keeps an existing settings file's mode and creates a new one at 0644; the temporary file it writes through started at 0600. Test: setup_keeps_an_existing_settings_mode_and_creates_new_ones_readable.
3. Accepted (review finding atelier-c4ze). Whole-repository `atelier check` passes after `issue create --issue-type mission` with no body, and the strategy link is optional since atelier-9m4k. `atelier check <id>` still reports the missing Outcome and Success criteria, as it reports a bare task's missing Goal and Done when: it lints a draft's sections on request, which is intended, and the mission cannot pass `ready` until they are written.
4. Fixed. Every `atelier review` subcommand and argument has help text.
5. Fixed. The two test doc comments no longer mention export/import.

## Close Reason

Each item is fixed with a test or recorded: (1) land and merge refuse a config.toml that differs from HEAD; (2) setup keeps the settings mode, tested; (3) check <id> linting a bare mission's sections is accepted as intended; (4) review subcommands have help; (5) export/import doc comments fixed. See Design.
