---
created_at: "2026-06-19T03:58:36.383292487+00:00"
id: "atelier-13yy"
issue_type: "task"
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
closed_at: "2026-06-19T04:26:45.090641845+00:00"
status: "done"
title: "Move Forgejo settings under review provider config"
updated_at: "2026-06-19T04:26:45.090641845+00:00"
---

## Description

Move existing Forgejo PR configuration under the new review provider namespace
without adding compatibility aliases or a long-lived fallback path.

## Outcome

- Forgejo provider config lives under the nested review provider shape described
  in the mission plan.
- Existing Forgejo config migrates to the new shape with host, owner, repo,
  admin token environment, and role author mappings preserved.
- Old Forgejo PR config locations are rejected after migration with guidance to
  the new `[review]` shape.
- Forgejo role check/provision surfaces read the new config path.

## Evidence

- Migration fixture proves old Forgejo config rewrites to the new nested review
  provider shape.
- Negative config tests prove old locations are rejected after migration.
- Forgejo config command tests show the role surfaces reading the new path.
