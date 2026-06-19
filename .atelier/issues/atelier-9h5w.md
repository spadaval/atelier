---
created_at: "2026-06-19T03:58:33.617239490+00:00"
id: "atelier-9h5w"
issue_type: "feature"
labels:
- "config"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add mutually exclusive review mode configuration"
updated_at: "2026-06-19T03:58:33.617239490+00:00"
---

## Description

Add the top-level review configuration contract for projects. This issue is
limited to the mode selector and mode-specific validation boundary.

## Outcome

- `[review] mode = "room"` enables native room behavior without provider
  settings.
- `[review] mode = "provider"` requires a supported provider name and provider
  configuration.
- Config loading rejects missing mode, unknown mode, missing provider,
  unsupported provider, and mixed room/provider-only settings.
- Human error output names the accepted config shapes.

## Evidence

- Unit tests cover accepted room/provider configs and rejected invalid shapes.
- CLI test fixture output shows corrective config errors for each invalid mode
  case.
- Focused config tests and `atelier lint atelier-9h5w` pass.
