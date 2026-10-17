---
created_at: "2026-10-17T02:37:51.861454745+00:00"
id: "atelier-e95f"
issue_type: "feature"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let repositories define custom work views declaratively"
updated_at: "2026-10-17T02:37:51.861454745+00:00"
---

## Goal

Let a repository define its own work views as committed, declarative definitions, so an orchestrator can see items sliced by the fields and filters its project or role needs (project labels, configured statuses, a reviewer's columns, one mission's land-related items) without scripting around Atelier's output.

## Done when

A design is accepted (an ADR, since it qualifies ADR 0022 Decision 4) and then built, so that:
- View definitions live in `.atelier/`, are validated by `atelier check`, and are reviewed like other records.
- A definition may only filter, choose fields from a fixed vocabulary, sort (using the shared order), and group. It cannot redefine derived states (ready, blocked, evidence freshness, merge readiness); those always come from core, so a view never contradicts a gate.
- Definitions render through the same item-line contract and renderer as built-in views, so output stays dense Markdown. No scripts and no machine format.
- Built-in views such as `work ready` and `work mission` are expressed in the same format.

## Scope

Declarative configuration only. Runtime code, scripting hooks, and JSON or YAML output are out of scope.

## Context

From discussion with Sumeet on 2026-10-17 about "code mode" interfaces. The questions an orchestrator asks (what is ready, what holds it up, what needs a human) are universal, and wanting a custom view for one of them means the built-in view is incomplete: file that defect, per ADR 0022 Decision 4. How the data is sliced is not universal, and building every slice in would bloat the views. A committed custom view is itself a signal: if many repos define the same one, promote it to built-in.

Blocked by atelier-i2qq, which writes the shared item-line contract this format depends on. Defining views before that contract exists would freeze today's inconsistent views into a public format.

## Unknowns

- Whether demand survives the view fixes. Re-count the piped atelier commands in transcripts (the method atelier-bw0b used) after missions atelier-1w8c, atelier-4pzj and atelier-o3lm land. If agents no longer reshape output, close this as superseded.
- Whether definitions belong in `.atelier/config.toml` or in their own records.
- The field vocabulary, and how a view names a mission or a parent.
