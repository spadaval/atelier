---
created_at: "2026-10-17T18:58:40.742588912+00:00"
id: "atelier-y8xw"
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
    id: "atelier-t66i"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Cap the one-way decisions in the brief decision view"
updated_at: "2026-10-17T18:58:40.742588912+00:00"
---

## Goal

`decision_brief_lines` in crates/atelier-cli/src/human_output.rs prints every one-way decision bearing on an item with no cap, and `status` runs it at every session start, against the strategy rule that anything shown at session start stays bounded. Found by review atelier-t66i on atelier-1mk4.

## Done when

The brief form shows at most the newest few one-way decisions and a `+N more` line naming `atelier issue show <id> --full`, as status already caps reviews and evidence, shown by a CLI test with more one-way decisions than the cap.
