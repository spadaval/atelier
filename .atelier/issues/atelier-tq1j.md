---
created_at: "2026-10-18T06:31:27.971725108+00:00"
id: "atelier-tq1j"
issue_type: "feature"
labels:
- "evidence"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Record where a follow-up was found with a discovered_from link"
updated_at: "2026-10-18T06:31:27.971725108+00:00"
---

## Goal

Record where a follow-up was found with a typed `discovered_from` link on the new issue, pointing at the record it surfaced from: the item being worked, a review finding, or an evidence receipt. Today only review deferrals carry this, on the disposition event; follow-ups from mission validation say it in Goal prose ("independent validation of atelier-r6bt") and ones an implementer noticed live only in Close Reason's Open items. Without the link, the debrief cannot place discovered work at its source and time, and every follow-up must also `advances` the mission, which mixes "found along the way" with the mission's real scope.

## Done when

`atelier issue create --discovered-from <id>` stores the link; `check` refuses a target that does not exist and a deferred disposition whose follow-up names a different source; `issue show` prints the source; the debrief lists discovered work by source without requiring an `advances` link to the mission. Shown by CLI tests.

## Scope

The link type, the create flag, check rules, issue show, the debrief's discovered-work grouping, and the Agent Factory skill's filing guidance (tracker-records.md, atelier.md, finding-disposition.md). Backfill existing review follow-ups from their dispositions and the three validation follow-ups of atelier-r6bt (0a52, juyb, mg7w) by hand. Remove the evidence schema's unused `follow_up_ids` field, which no command writes.

## Context

Discussed with the human on 2026-10-18 while redesigning the debrief as an after-action report. Shape Up's imagined versus discovered tasks is the model; Beads names the same relation `discovered-from` (docs/references/minibeads.md). Link types: WELL_KNOWN_LINK_TYPES in crates/atelier-records/src/lib.rs. Dispositions: `follow_up` on review disposition events.

## Unknowns

Whether a follow-up found in review stores both the disposition's `follow_up` and its own `discovered_from` (two stores check keeps in agreement, recommended), or only the disposition. Whether discovered work that the mission must finish still takes `advances` as well.
