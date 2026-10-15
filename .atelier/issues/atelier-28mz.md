---
created_at: "2026-10-15T18:57:15.135112410+00:00"
id: "atelier-28mz"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9vq3"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Decide whether children of draft epics can start"
updated_at: "2026-10-15T18:57:15.135112410+00:00"
---

## Goal

Decide and enforce whether work under a `draft` epic can be listed as ready or started.

## Done when

The chosen rule is documented, and tests cover `work ready` and `start` for a child of a draft epic.

## Context

External evaluation 2026-10-15: a child started under a draft epic, and `work mission` lists it as ready. The tracker-commands reference says draft epics are planning context, not ready work, but says nothing about their children. Deferred from the evaluation fixes because it needs a policy decision (a workflow validator or a ready-classification rule).
