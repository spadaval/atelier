---
created_at: "2026-06-11T18:55:07.069485604+00:00"
id: "atelier-vme7"
issue_type: "task"
labels:
- "hooks"
- "integration"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dydv"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-utql"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Move Claude hooks into an explicit optional integration"
updated_at: "2026-06-12T00:12:24.022523831+00:00"
---

## Description

Separate Claude hook installation from core Atelier tracker initialization.

Scope:
- Introduce an explicit command or documented flow such as atelier integrations claude install for .claude/hooks and .mcp.json setup.
- Make integration code read tracked .atelier/config.toml for project policy where needed.
- Keep packaged hook/rule resources inside the binary or resources tree instead of copying .atelier/rules into every repository.
- Clarify whether hooks are supported product surface, experimental integration, or legacy compatibility.

Out of scope:
- Redesigning hook behavior itself.

## Outcome

- atelier init no longer installs Claude hooks or MCP config.
- A user who wants hooks has an explicit command/path to install them.
- Docs and help text distinguish core tracker setup from optional Claude integration setup.

## Evidence

Evidence was not specified in the legacy issue record.
