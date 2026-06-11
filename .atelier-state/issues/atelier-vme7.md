---
acceptance: []
created_at: "2026-06-11T18:55:07.069485604+00:00"
evidence_required: []
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
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Move Claude hooks into an explicit optional integration"
updated_at: "2026-06-11T18:55:07.069485604+00:00"
---

Separate Claude hook installation from core Atelier tracker initialization.

Scope:
- Introduce an explicit command or documented flow such as atelier integrations claude install for .claude/hooks and .mcp.json setup.
- Make integration code read tracked .atelier/config.toml for project policy where needed.
- Keep packaged hook/rule resources inside the binary or resources tree instead of copying .atelier/rules into every repository.
- Clarify whether hooks are supported product surface, experimental integration, or legacy compatibility.

Out of scope:
- Redesigning hook behavior itself.

Acceptance criteria:
- atelier init no longer installs Claude hooks or MCP config.
- A user who wants hooks has an explicit command/path to install them.
- Docs and help text distinguish core tracker setup from optional Claude integration setup.
