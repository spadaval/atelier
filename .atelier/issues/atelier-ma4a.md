---
created_at: "2026-10-30T06:40:11.262191293+00:00"
id: "atelier-ma4a"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-abdp"
  - kind: "issue"
    id: "atelier-efnf"
  - kind: "issue"
    id: "atelier-r9hb"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ll3p"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show a work item's review history where it is judged"
updated_at: "2026-10-30T06:40:25.715333306+00:00"
---

## Goal

A reader deciding whether an owner is ready can see how its review went: how many rounds it took, whether a fix-now finding is still open, and whether its Done when changed after the item was started. Today none of these appear in branch readiness, the review request brief or the debrief.

## Done when

Every child is done with its own proof, the increment has an independent approval, and a receipt on atelier-ll3p judges criterion review-history-shown with a captured run of the children's tests.

## Background

Loops seen in the 2026-10-15 to 10-29 transcripts: atelier-vkzj took 7 review rounds, efkh about 6 and 37uq 5, each round finding a wider form of the same flaw. About 12 items had Done when or criteria rewritten, waived or unlinked after a finding, often then re-approved. Atelier already keeps per-record history from Git (crates/atelier-app/src/record_history.rs) and reads review events per owner (`review list`).
