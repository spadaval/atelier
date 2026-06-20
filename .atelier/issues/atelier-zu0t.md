---
created_at: "2026-06-19T22:54:23.411264873+00:00"
id: "atelier-zu0t"
issue_type: "task"
labels:
- "config"
- "migration"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-qx40"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T00:02:06.614438311+00:00"
status: "done"
title: "Remove runtime and compatibility paths from project config"
updated_at: "2026-06-20T00:02:06.614438311+00:00"
---

## Description

Remove `compatibility_state_root` and committed runtime/cache path settings from `.atelier/config.toml`. Runtime database, cache, and compatibility/migration state should be implementation defaults or local state, not tracked project policy, unless the config-boundary decision explicitly keeps a narrowed field.

## Outcome

- Tracked project config contains project policy, not local runtime storage paths.
- Any compatibility state root is removed or has an explicit non-compatibility target name and owner.

## Evidence

- .atelier/config.toml file change removes `compatibility_state_root`.
- .atelier/config.toml file change removes committed runtime/cache/database path settings or replaces them with the approved target shape.
- `target/debug/atelier lint` passes after config migration.
- CLI integration tests prove init/config loading works without committed runtime path settings.
