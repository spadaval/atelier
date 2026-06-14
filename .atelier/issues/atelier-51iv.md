---
created_at: "2026-06-14T01:02:45.714730495+00:00"
id: "atelier-51iv"
issue_type: "bug"
labels:
- "cli"
- "projection"
- "resilience"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Read commands should degrade when canonical rebuild fails"
updated_at: "2026-06-14T02:53:28.418166502+00:00"
---

## Description

Ordinary read/orientation commands such as `atelier issue list`, `atelier status`,
and `atelier mission status` currently depend on fresh projection validation. When
a single canonical record is malformed, for example one evidence record
missing/using an unexpected front matter shape, these commands can fail entirely
instead of helping the user inspect and repair the repository. That makes a
localized canonical-record problem take down basic tracker orientation.

Expected behavior:

- Strict commands (`atelier rebuild`, `atelier lint`, `atelier export --check`,
  closeout/mutation gates) continue to fail on malformed canonical records.
- Ordinary read/orientation commands use the last known good local projection
  when canonical validation or rebuild fails.
- Degraded reads print a clear warning naming the malformed record or canonical
  diagnostic and give repair commands such as `atelier lint` and `atelier
  rebuild`.
- Mutating commands and closeout remain blocked until canonical records are
  repaired.

Validation criteria:

- A fixture with one malformed evidence record makes `atelier rebuild`, `atelier
  lint`, or `atelier export --check` fail with a precise diagnostic.
- The same fixture leaves `atelier issue list` usable from the last good
  projection and prints a visible degraded-state warning.
- Mutation/closeout commands remain blocked in the degraded state.
- Tests cover at least `issue list` and one repository-orientation surface such
  as `status` or `mission status`.

## Outcome

- Strict rebuild/lint/export and closeout/mutation gates still reject malformed
  canonical records.
- Ordinary read/orientation commands can continue from the last known good
  projection when canonical refresh fails.
- Degraded read output names the canonical diagnostic and repair commands.

## Evidence

- Test fixture or integration test transcript for malformed evidence plus
  `atelier rebuild`, `atelier lint`, and `atelier export --check` strict
  failures.
- Test transcript showing `atelier issue list` still returns issue rows from the
  last good projection with a degraded-state warning.
- Test transcript for one orientation command such as `atelier status` or
  `atelier mission status` in the same degraded state.
- Test or manual check showing mutation/closeout remains blocked until the
  malformed record is repaired.
