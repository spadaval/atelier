---
created_at: "2026-07-06T17:20:25.692343496+00:00"
id: "atelier-tdgs"
issue_type: "feature"
labels:
- "cli"
- "mission-dashboard"
- "projection"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dy3u"
  - kind: "issue"
    id: "atelier-sjsz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Build the mission-to-epic overview projection"
updated_at: "2026-07-06T17:20:25.692343496+00:00"
---

## Description

Add a dedicated overview model that selects non-done missions by default, resolves directly linked epics through mission `advances` relationships, summarizes their descendant work without leaf rows, and accounts for direct-linked and unassigned nonterminal work according to the approved contract. Provide deterministic ordering, progress/state counts, omission facts, and explicit inclusion of done missions.

## Outcome

- The Mission Overview model exposes visible missions, linked epic rows, collapsed descendant counts, direct-work summaries, unassigned-work facts, and stable ordering without deriving mission scope from issue parentage.
- Done missions are absent from the default model and present only when explicitly requested; child tasks remain available through `work mission` and `work epic` drill-downs rather than global rows.

## Evidence

Evidence was not specified in the bundle.
