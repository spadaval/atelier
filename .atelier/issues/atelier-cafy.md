---
created_at: "2026-10-15T23:47:13.976529080+00:00"
id: "atelier-cafy"
issue_type: "task"
labels:
- "adoption"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Let follow-up records for deferred findings keep an approval current"
updated_at: "2026-10-15T23:47:13.976529080+00:00"
---

## Goal

A `defer` disposition needs an existing follow-up issue. Filing that issue after the reviewer approved (the natural order, since the reviewer recommends the defer) makes the approval stale, because a new issue record is not one of the changes approval freshness ignores. Every deferred finding then costs a second approval round.

## Done when

Either approval freshness ignores newly created issue records that are outside the owner's scope, or the review docs and the Agent Factory reference say to file follow-ups before the final approval. A test covers the chosen rule.

## Context

Found 2026-10-15 on atelier-9k7o: filing atelier-lwdu after approval atelier-j2r2 blocked review.current until a fresh approval.
