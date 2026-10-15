---
created_at: "2026-10-15T19:31:02.027687676+00:00"
id: "atelier-v44v"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Small CLI rough edges from rebuild validation"
updated_at: "2026-10-15T19:31:02.027687676+00:00"
---

## Goal

Resolve or consciously accept the minor issues found by independent validation.

## Done when

Each item below is fixed with a test or recorded as accepted: (1) land does not refuse uncommitted .atelier/config.toml edits; (2) atelier setup leaves settings.json at mode 0600; (3) issue create accepts a mission with neither a strategy link nor success criteria, which check then flags; (4) review subcommands lack help descriptions; (5) two test doc comments still mention export/import.

## Context

Independent S1-S8 validation 2026-10-15 (atelier-xz11).
