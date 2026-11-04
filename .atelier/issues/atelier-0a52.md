---
created_at: "2026-10-17T20:10:21.803980772+00:00"
id: "atelier-0a52"
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
  - kind: "issue"
    id: "atelier-r6bt"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show the strategy's outcome in the mission brief"
updated_at: "2026-11-04T23:31:35.958207095+00:00"
---

## Goal

S5 says the brief contains the strategy outcome; it shows the mission record's Outcome, which can differ from the strategy file's (independent validation of atelier-r6bt). Show the strategy's Outcome, as decided in this item's decision note.

## Done when

The brief, in both forms, shows the linked strategy file's Outcome section, and the mission record's Outcome when the mission has no strategy or the strategy has no Outcome section, shown by a fixture whose two texts differ.

## Notes

- 2026-11-04 [decision] Show the outcome S5 names: the linked strategy file's Outcome section, falling back to the mission record's Outcome when the mission has no strategy or the strategy has no Outcome section
  Reason: S5 says 'the strategy outcome', so the brief can meet it as written without the human rewording S5; the fallback keeps briefs for strategy-less missions working
