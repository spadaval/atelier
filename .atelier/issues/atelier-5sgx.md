---
created_at: "2026-06-23T15:22:28.048372977+00:00"
id: "atelier-5sgx"
issue_type: "feature"
labels:
- "cli"
- "formatter"
- "human-output"
- "workflow-state"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3js3"
  - kind: "issue"
    id: "atelier-4wmp"
  - kind: "issue"
    id: "atelier-7fof"
  - kind: "issue"
    id: "atelier-t8ew"
  - kind: "issue"
    id: "atelier-wxox"
  - kind: "issue"
    id: "atelier-ycj9"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Introduce shared human terminal formatter and color policy"
updated_at: "2026-06-23T20:24:23.775087872+00:00"
---

## Description

Implement the reusable terminal formatting boundary needed by the UX refresh.

## Outcome

- Shared helpers cover headings, dim or secondary text, status styling,
  footers, bounded lists, path summaries, workflow/blocker/display-role labels,
  and color enablement.
- Shared helpers can render concise public recovery callouts without each
  command inventing its own wording.
- Shared row and group helpers support display roles such as executable,
  selectable, blocked, blocked-through-parent, context-only, and omitted.
- Color is automatic only for interactive terminals, disabled for
  non-interactive output and `NO_COLOR`, and never carries meaning without
  text.
- Existing quiet output remains terse and colorless.
- At least two command surfaces consume the shared helpers without duplicating
  style policy.

## Evidence

- Focused CLI or formatter tests prove color auto behavior, `NO_COLOR`
  behavior, colorless logs, reusable style helpers, public recovery callout
  rendering, and display-role row rendering.
- `cargo fmt -- --check`, focused CLI tests, and `git diff --check` pass.
