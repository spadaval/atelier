---
created_at: "2026-10-17T19:00:36.354527404+00:00"
id: "atelier-hulj"
issue_type: "task"
labels:
- "retro-2026-10-17"
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Stop review records repeating their body in the title"
updated_at: "2026-10-17T19:00:36.354527404+00:00"
---

## Goal

Review records take 1.17 MB in the window and each record's `title` field repeats its body text.

## Done when

New finding, disposition and approval records carry a short title (or none) and the text once. `review show` and `review list` still render the same information. A test pins the record shape.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
