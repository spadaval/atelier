---
created_at: "2026-06-10T20:58:48.813640540+00:00"
id: "atelier-o54s"
issue_type: "epic"
labels:
- "assignee:root"
- "cli"
- "human-output"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-o78q"
  - kind: "issue"
    id: "atelier-omnw"
  - kind: "issue"
    id: "atelier-ugeo"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T22:08:01.634171323+00:00"
status: "done"
title: "Define Atelier human-output design system"
updated_at: "2026-06-10T22:08:01.634171323+00:00"
---

## Description

Create the shared design and implementation direction for non-JSON CLI rendering. This epic should audit current default outputs, choose the reusable formatter primitives, and define the rules future commands follow.

Why:
The current output problems are systemic: each command renders its own ad hoc view, so fixes to one surface do not create a durable pattern.

Scope:
- Inspect current mission, issue, list, ready/search, and tree human renderers.
- Define formatter primitives for color, status chips, section headings, indentation, wrapping, terminal width, empty states, relative density, and next-command footers.
- Decide how color is enabled/disabled and how non-color output remains complete.
- Capture output examples influenced by Jira CLI `jira view`, translated into Atelier domain language.

Out of scope:
- Implementing every command output change.
- Changing JSON schemas.
- TUI work.

## Outcome

- A documented pattern exists for detail views, grouped list/queue views, compact hierarchy views, and empty/error states.
- Formatter helper boundaries are clear enough for implementation epics to use without re-litigating style.
- Any high-leverage unresolved choices are captured as artifact-update tasks before implementation depends on them.

Recommended subskill: agent-factory plan or implement.

## Evidence

Evidence was not specified in the legacy issue record.
