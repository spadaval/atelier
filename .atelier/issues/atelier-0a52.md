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
closed_at: "2026-11-04T23:33:09.135991974+00:00"
status: "done"
title: "Show the strategy's outcome in the mission brief"
updated_at: "2026-11-04T23:33:09.135991974+00:00"
---

## Goal

S5 says the brief contains the strategy outcome; it shows the mission record's Outcome, which can differ from the strategy file's (independent validation of atelier-r6bt). Show the strategy's Outcome, as decided in this item's decision note.

## Done when

The brief, in both forms, shows the linked strategy file's Outcome section, and the mission record's Outcome when the mission has no strategy or the strategy has no Outcome section, shown by a fixture whose two texts differ.

## Notes

- 2026-11-04 [decision] Show the outcome S5 names: the linked strategy file's Outcome section, falling back to the mission record's Outcome when the mission has no strategy or the strategy has no Outcome section
  Reason: S5 says 'the strategy outcome', so the brief can meet it as written without the human rewording S5; the fallback keeps briefs for strategy-less missions working

## Close Reason

`Brief.outcome` is now the linked strategy file's Outcome section. When the mission has no strategy, or its strategy has no non-empty Outcome section, it falls back to the mission record's Outcome. The Markdown and HTML forms both read that one field. The brief reads the strategy file once for Outcome and Risks. The new CLI fixture `brief_shows_the_strategy_outcome_and_falls_back_to_the_mission_outcome` uses a strategy whose Outcome differs from the mission's and checks both forms. It then removes the strategy's Outcome section and checks again, then briefs a mission with no strategy. It failed before the change. docs/product/mission-reviews.md says where the brief's Outcome comes from.

### Deviations
None.

### Decisions
I followed this item's existing decision note. I made no new decisions.

### Surprises
None.

### Open items
The debrief's "What We Set Out To Do" still shows the mission record's Outcome. S4 does not name the strategy outcome, so I left it alone.
