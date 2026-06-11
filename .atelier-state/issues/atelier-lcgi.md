---
acceptance: []
created_at: "2026-06-11T18:54:24.351600365+00:00"
evidence_required: []
id: "atelier-lcgi"
issue_type: "task"
labels:
- "docs"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dinu"
  - kind: "issue"
    id: "atelier-nwlx"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-b8zm"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define the one-directory .atelier filesystem contract"
updated_at: "2026-06-11T23:07:08.456713794+00:00"
---

Document the target contract for a single .atelier/ root before implementation begins.

Scope:
- Replace the .atelier-state/.atelier split in product and architecture docs with one .atelier/ root.
- Specify which paths are tracked canonical project state and which paths are ignored local runtime state.
- Make .atelier/config.toml the tracked project config target unless a later decision explicitly chooses a different file.
- State that state.db stays local and rebuildable.

Out of scope:
- Moving files or changing code paths.

Acceptance criteria:
- SPEC.md, CONTEXT.md, AGENTFACTORY.md, AGENTS.md, and canonical storage docs agree on the same layout.
- The docs no longer present .atelier-state as the target name.
- The contract explicitly says copied rule trees are not project state.
