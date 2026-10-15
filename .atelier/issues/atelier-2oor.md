---
created_at: "2026-10-15T18:45:05.756391340+00:00"
id: "atelier-2oor"
issue_type: "bug"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Mint record IDs from the configured project slug"
updated_at: "2026-10-15T18:45:05.756391340+00:00"
---

## Goal

New record IDs use `project_slug` from `.atelier/config.toml`, and `init` derives a default slug from the repository directory name with an override flag.

## Done when

- `issue create`, `bundle apply` and every other minting path use the configured slug (tests with a non-`atelier` slug).
- `atelier init` in a directory named `astromech` writes `project_slug = "astromech"`; `--slug` overrides it; invalid slugs are refused before writing.
- Existing `atelier-` records stay valid in a repo with a different slug.

## Scope

Record ID generation, init, project config, and docs that describe them.

## Context

External evaluation 2026-10-15 on astromech: IDs were minted from `DEFAULT_PROJECT_SLUG` in crates/atelier-records/src/record_id.rs; only evidence.rs read ProjectConfig.project_slug.
