---
created_at: "2026-06-10T16:09:12.870412513+00:00"
id: "atelier-p1yj"
issue_type: "task"
labels:
- "cli"
- "human-output"
- "issue-show"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-pakd"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T17:29:23.393254540+00:00"
status: "done"
title: "Render richer issue show human header and core sections"
updated_at: "2026-06-10T17:29:23.393254540+00:00"
---

## Description

Implement the core richer human layout for `atelier issue show <id>`.

What:
- Enhance `commands::agent_factory::show` human output only.
- Render a compact header with `<id> <type/title context>` and title, status, type, priority, and assignee/owner when present.
- Include created, updated, and closed timestamps when available.
- Render labels as a compact comma-separated row.
- Preserve full description and acceptance criteria without truncation.
- Add consistent ASCII section headers and spacing for long text.
- Show close reason for closed issues when available.
- Ensure `atelier show <id>` inherits the same human output through the existing shortcut path.

Out of scope:
- Changing the `issue.show --json` envelope or fields.
- Adding a new `issue view` command or `--format` flag.
- Implementing hierarchy/dependency row expansion beyond existing IDs; that is owned by the dependency-context child.
- Activity preview and command footer behavior; those are owned by the activity/footer child.

## Outcome

- Human `atelier issue show <id>` includes the richer header, metadata rows, labels, full description, acceptance criteria, and closed reason where applicable.
- Human `atelier show <id>` uses the same improved display.
- Existing terminal-safe rendering helpers are used where they fit; no new terminal-width dependency is introduced for v1.
- `atelier issue show <id> --json` remains byte/shape compatible except for unrelated ordering that existing JSON tests already tolerate.
- Nonexistent issue errors remain clear.

Recommended subskill: agent-factory implement.

## Evidence

Evidence was not specified in the legacy issue record.
