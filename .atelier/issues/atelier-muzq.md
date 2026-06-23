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
status: "todo"
title: "Isolate issue field handling for future custom fields"
updated_at: "2026-06-23T16:21:20.075917646+00:00"
---

## Description

Keep current issue field behavior stable while isolating it behind issue-domain parsing and validation so future top-level custom fields can be added without generic JSON domain modeling.

## Outcome

- The review field remains supported and future custom fields have an obvious extension point.

## Evidence

Evidence was not specified in the bundle.
