---
created_at: "2026-10-17T01:44:17.482012585+00:00"
id: "atelier-rc8j"
issue_type: "mission"
labels:
- "cli-output"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-093h"
    type: "advances"
  - kind: "issue"
    id: "atelier-2tux"
    type: "advances"
  - kind: "issue"
    id: "atelier-3ekh"
    type: "advances"
  - kind: "issue"
    id: "atelier-3u2u"
    type: "advances"
  - kind: "issue"
    id: "atelier-6rh7"
    type: "advances"
  - kind: "issue"
    id: "atelier-7ts2"
    type: "advances"
  - kind: "issue"
    id: "atelier-9jr8"
    type: "advances"
  - kind: "issue"
    id: "atelier-b05y"
    type: "advances"
  - kind: "issue"
    id: "atelier-bphy"
    type: "advances"
  - kind: "issue"
    id: "atelier-bvl3"
    type: "advances"
  - kind: "issue"
    id: "atelier-cisa"
    type: "advances"
  - kind: "issue"
    id: "atelier-ft88"
    type: "advances"
  - kind: "issue"
    id: "atelier-grsi"
    type: "advances"
  - kind: "issue"
    id: "atelier-h6o4"
    type: "advances"
  - kind: "issue"
    id: "atelier-kiry"
    type: "advances"
  - kind: "issue"
    id: "atelier-mp6v"
    type: "advances"
  - kind: "issue"
    id: "atelier-ns04"
    type: "advances"
  - kind: "issue"
    id: "atelier-o8ni"
    type: "advances"
  - kind: "issue"
    id: "atelier-u0ig"
    type: "advances"
  - kind: "issue"
    id: "atelier-uv5d"
    type: "advances"
  - kind: "issue"
    id: "atelier-yrlj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make every gate end in a verdict, with each failure's cause and fix"
updated_at: "2026-10-17T01:44:17.482012585+00:00"
---

## Outcome

`land`, `merge --check`, `review` and `evidence` tell their reader:
- what was checked or recorded;
- which checks fail, against which record, and why;
- the exact command that fixes each failure;
- a final verdict.

Evidence keeps the tail of a failing command and can be listed per item. Review state can be read per item without opening record files. Stop-hook output stays separate from landing failures.

## Success criteria

- S1: `land` and `merge --check` list failing checks first, one line each with the record, the cause and a runnable fix using real IDs, then end with a verdict. Every gate is evaluated even when a precondition fails. A passing land says what it checked (atelier-9jr8, atelier-6rh7, atelier-kiry, atelier-grsi, atelier-7ts2, atelier-ns04, atelier-yrlj, atelier-bvl3, atelier-2tux). Shown by tests over the land and merge refusal paths.
- S2: When an approval is not current, the gate names the approval it considered and why (atelier-cisa). Shown by tests for stale-by-code, stale-by-base-merge and author-identity cases.
- S3: `evidence record` confirms in one line with the result. Captured output keeps the failure tail. `evidence list` filters by item. `evidence show` is brief (atelier-3ekh, atelier-bphy, atelier-u0ig, atelier-o8ni, atelier-b05y). Shown by CLI tests.
- S4: `review list <id>` shows review state, with findings under their dispositions. `review show` is brief. Recording a review event confirms what was recorded and warns on an author identity (atelier-uv5d, atelier-h6o4, atelier-3u2u, atelier-093h). Shown by CLI tests.
- S5: Stop-hook adapter errors are reported apart from landing failures. Uncommitted records alone on the base do not block a turn (atelier-ft88, atelier-mp6v). Shown by hook tests.
- S6: Claude Code transcripts recorded after S1-S4 land show `land`, `merge --check`, `evidence record` and `review list` piped through head, grep, sed or tail in under a third of runs. Shown by a dated note with the counts.

## Background

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method and the transcript scripts. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Where an item here rewrites the same renderer as an atelier-o3lm or atelier-1w8c item, it is blocked by that item. The 2026-10-16 transcript retrospective found gate refusals that named no fix to be the most expensive friction after review churn.
