---
created_at: "2026-06-17T18:00:33.745331853+00:00"
id: "atelier-e7oj"
issue_type: "task"
labels:
- "config"
- "forgejo"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T00:51:57.660441332+00:00"
status: "done"
title: "Add Forgejo config and role sudo-user mapping"
updated_at: "2026-06-18T00:51:57.660441332+00:00"
---

## Description

Extend `.atelier/config.toml` with Forgejo integration settings and role-based
sudo identity mapping.

## Outcome

- Project config can name Forgejo host, owner, repo, and admin token
  environment variable.
- Project config can map worker, reviewer, validator, manager, and admin roles
  to Forgejo sudo users.
- Missing or invalid Forgejo config produces actionable `atelier pr` and
  validator errors.

## Evidence

- Focused config parser tests cover valid Forgejo config and invalid or missing
  role/token settings.
- Manual check of config documentation file content shows Forgejo and sudo
  mapping guidance.
