---
created_at: "2026-06-20T16:54:08.885688768+00:00"
id: "atelier-439j"
issue_type: "task"
labels:
- "cutting-pass"
- "mission-collapse"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v2o6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T20:30:33.781173252+00:00"
status: "done"
title: "Add type-aware issue creation and sections for mission objectives"
updated_at: "2026-06-20T20:30:33.781173252+00:00"
---

## Description

Add type-aware issue creation support for mission-shaped objective records.
Mission creation should use the general issue creation surface with
mission-specific section requirements instead of `mission create`.

## Outcome

- A configured or built-in `mission` issue type can be created through the
  general issue command surface.
- Mission objective records have required `Intent`, `Constraints`, `Risks`, and
  `Validation` sections and appropriate optional notes sections.
- Existing task/bug/feature issue creation remains unchanged.

## Evidence

- Focused CLI tests prove mission-type issue creation writes the expected
  canonical Markdown sections.
- `atelier lint <mission-id>` rejects missing required mission sections.
- `target/debug/atelier issue show <mission-id>` displays the typed sections.
