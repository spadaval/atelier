---
created_at: "2026-10-17T22:13:22.538334817+00:00"
id: "atelier-lyow"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "review"
    id: "atelier-2boy"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-3jh7"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-g0oy"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T07:10:47.830952937+00:00"
status: "superseded"
title: "Tidy labels for findings and receipts mentioned on the debrief page"
updated_at: "2026-10-18T07:10:47.830952937+00:00"
---

## Goal

Tidy how the debrief page labels mentioned findings and receipts (review of atelier-nexd): a summary that starts with an ID leaves that ID in the label (atelier-3jh7); labels are capped by six words, not length, so path-led summaries reach about 110 characters (atelier-g0oy); the doc comment on `rich()` still says mentions show their ID (atelier-2boy).

## Done when

Labels replace or drop IDs, are capped at 60 characters, and the comment is current, shown by a fixture whose summary starts with an ID.

## Close Reason

Overtaken by atelier-8bil: mentioned findings and receipts are now chips with their ID and kind, so the label tidy-ups (IDs in labels, the six-word cap, the rich() comment) no longer apply.
