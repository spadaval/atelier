---
acceptance: []
created_at: "2026-06-11T18:54:40.116960501+00:00"
evidence_required: []
id: "atelier-nwlx"
issue_type: "task"
labels:
- "config"
- "gitignore"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dydv"
  - kind: "issue"
    id: "atelier-xcy9"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Track Atelier project config and ignore only runtime files"
updated_at: "2026-06-11T18:54:40.116960501+00:00"
---

Replace the current repo-level ignore of all .atelier/ with precise tracked/ignored boundaries.

Scope:
- Add a tracked .atelier/config.toml for project-level Atelier settings.
- Update .gitignore so .atelier/ is not ignored wholesale.
- Ignore only local runtime artifacts such as .atelier/state.db, .atelier/state.db-shm, .atelier/state.db-wal, .atelier/cache/, .atelier/agent.json, and .atelier/.locks-cache/.
- Remove the need for .atelier/.gitignore unless a narrowly scoped inner ignore remains justified.

Out of scope:
- Designing a large configuration schema beyond the fields needed for current runtime and integration behavior.

Acceptance criteria:
- git status can show tracked .atelier/config.toml and canonical record files.
- state.db and lock/cache/local identity artifacts remain ignored.
- Documentation describes config as tracked project state, not local runtime state.
