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
status: "todo"
title: "Decide whether the brief shows the mission's or the strategy's outcome"
updated_at: "2026-11-04T23:12:59.355320808+00:00"
---

## Goal

S5 says the brief contains the strategy outcome; it shows the mission record's Outcome, which can differ from the strategy file's (independent validation of atelier-r6bt).

## Done when

The brief shows the outcome S5 means, or S5 is reworded by the human, and a fixture with differing texts shows which.

## Notes

- 2026-11-04 [decision] Show the outcome S5 names: the linked strategy file's Outcome section, falling back to the mission record's Outcome when the mission has no strategy or the strategy has no Outcome section
  Reason: S5 says 'the strategy outcome', so the brief can meet it as written without the human rewording S5; the fallback keeps briefs for strategy-less missions working
