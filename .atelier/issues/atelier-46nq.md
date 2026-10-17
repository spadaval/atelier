---
created_at: "2026-10-17T01:10:57.985849753+00:00"
id: "atelier-46nq"
issue_type: "feature"
labels:
- "cli-output"
- "work-views"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Say what each Needs a Human item is and what it holds up"
updated_at: "2026-10-17T01:10:57.985849753+00:00"
---

## Goal

Needs a Human rows read `shown for context atelier-spvd [todo] medium - Pick widget wire format`: `DisplayRole::for_work` maps needs-human to ContextOnly (crates/atelier-cli/src/human_output.rs:596-602), so the one list the human acts on says "shown for context". The row also does not say spvd is a decision, which mission it serves, or that it holds four items, and its medium priority understates that. A human glancing at `work human` cannot tell what to do first.

## Done when

- `work human` and Needs a Human panels in `work ready` and `work mission` print rows like `⚠ atelier-spvd decision  medium  Pick widget wire format — holds 4 (mission atelier-j76w)`.
- They sort by how many items each holds, then priority.
- Tests cover a decision and a needs-human task, with and without dependents.

## Context

Reworks the same row rendering as atelier-deyy. Fixture: scratch clone with mission atelier-j76w; parent atelier-qhnh (children s47h active, 9t9q blocked_by s47h, 1gc5 blocked_by 9vdd); qhnh blocked_by decision atelier-spvd (needs a human); chain 5hp9 <- fhep <- il31 <- ubti (ubti ready, low, no mission); 9vdd advances superseded mission atelier-ba3h.
