---
created_at: "2026-10-17T01:10:57.970482449+00:00"
id: "atelier-woul"
issue_type: "bug"
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
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show field-level changes for update entries in bundle preview"
updated_at: "2026-10-17T01:10:57.970482449+00:00"
---

## Goal

An `update` entry replaces `labels`, `title`, `body` and `priority` wholesale (docs/spec/bundle/schema.md, Updating records), but preview shows only `old -> atelier-uyuu (updated)  <new title>`. Reproduced: applying `"labels": ["bundle"]` to a record labelled `bundle, fixtures` silently dropped `fixtures`, and preview gave no hint. Notes appended and links added to the existing record are not attributed to it either.

## Done when

- For each updated record, preview and apply list the changes, e.g.
  `title: "Validate bundle fixtures" → "Validate bundle fixtures (revised)"`, `labels: bundle, fixtures → bundle (removes fixtures)`, `+1 note`, `+blocks p.b`.
- Body replacement shows which sections change (`body: Goal, Done when replaced`).
- A test covers a label removal shown in preview.
