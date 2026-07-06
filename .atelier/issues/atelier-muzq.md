---
created_at: "2026-06-23T16:21:20.075917646+00:00"
id: "atelier-muzq"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T18:19:33.919096976+00:00"
status: "done"
title: "Isolate issue field handling for future custom fields"
updated_at: "2026-07-06T18:19:33.919096976+00:00"
---

## Description

Keep current issue field behavior stable while isolating it behind issue-domain parsing and validation so future top-level custom fields can be added without generic JSON domain modeling.

## Outcome

- The review field remains supported and future custom fields have an obvious extension point.

## Evidence

- Typed issue-review, record-file round-trip, invalid-field, and review-room
  regression tests pass; transcript: `atelier-aapz`.
