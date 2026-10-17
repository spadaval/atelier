---
created_at: "2026-10-17T01:10:57.969557562+00:00"
id: "atelier-0xcr"
issue_type: "task"
labels:
- "bundle"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Put the bundle contract in bundle --help for repos without Atelier's docs"
updated_at: "2026-10-17T01:10:57.969557562+00:00"
---

## Goal

`atelier bundle preview --help` and `apply --help` show `<INPUT>` and `--yes` with no description. Nothing names the schema, its required envelope (`schema: "atelier.bundle"`, `schema_version: 2`, `title`, `resources`), the issue fields, or an example. The contract exists only in docs/spec/bundle/schema.md inside the Atelier repository. Adopters such as astromech have no copy, and the Agent Factory reference (.agents/skills/agent-factory/references/atelier.md:51) only says to run preview before apply. ADR 0022 Decision 2 puts usage guidance in `atelier help`.

## Done when

- `atelier bundle --help` shows a minimal valid bundle (a mission, a parent, a child, an update entry) and one line per issue field (client_ref, title, body, issue_type, priority, status, labels, parent, blocked_by, blocks, advances, notes, operation, id), plus the evidence fields.
- `bundle apply --help` describes `--yes` and says apply does not commit.
- A test runs `bundle preview` on the help example in a fresh `atelier init` repo and it succeeds.
