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
closed_at: "2026-10-22T21:25:11.434223369+00:00"
status: "done"
title: "Record where a follow-up was found with a discovered_from link"
updated_at: "2026-10-22T21:25:11.434223369+00:00"
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

## Notes

- 2026-10-22 [decision] A review follow-up stores both the disposition's follow_up and its own discovered_from link; check keeps them in agreement
  Reason: The disposition is the review's record of the deferral and the link is the follow-up's record of where it was found; check refuses a deferral whose follow-up declares a source other than the finding's item (no declared source is allowed, so older follow-ups stay valid), so the two cannot drift.
  Rejected: Only the disposition (leaves validation and implementer follow-ups without a source)
- 2026-10-22 [decision] discovered_from sources may be an issue, a review event or an evidence receipt, set only at issue create; issue link --role discovered_from stays issue-to-issue
  Reason: Follow-ups surface from findings and receipts as often as from items, and the create flag is where the source is known; the stored relates entry already carries a kind, so no schema change was needed. Review and receipt sources resolve to the item they are about for the debrief and the check rule.
  Rejected: A new link command for non-issue sources (more surface than the Done when asks for)
- 2026-10-22 [decision] Discovered work needs no advances link to appear in the debrief, and takes one only when the mission must finish it
  Reason: The debrief now reads discovered_from through the same source map as review deferrals, so 'found along the way' is separate from the mission's scope; filing it as scope would again mix the two.
- 2026-10-22 [decision] Debrief wording: work found through a discovered_from link reads 'found in <item>', a deferral reads 'from review of <item>'; a follow-up that is both appears once, as from review
  Reason: Smallest change that lists discovered work by source: one extra OpenEntry flag and one helper choosing the phrase, with existing output unchanged for review follow-ups (atelier-rc8j will rework debrief output).

## Close Reason

`atelier issue create --discovered-from <id>` stores a `discovered_from` link on the new issue (source: issue, review event or receipt); `issue show` prints it; `check` refuses a missing source and a deferral whose follow-up declares a different source; the debrief lists such work under Found Along The Way as `found in <item>` with no `advances` link needed. Six CLI tests in tests/cli_integration/discovered_from.rs show it.

### Deviations
`issue link --role discovered_from` is accepted (it is now a well-known role) but, like every issue link, only issue to issue; review and receipt sources are set at create or by hand. The skill edits are one sentence each in tracker-records.md and atelier.md; finding-disposition.md is unchanged. Also updated atelier.md's "creates no commits" line, which atelier-4ixu made false.

### Decisions
Four decision notes on atelier-tq1j: both stores kept in agreement by check, sources and where they are set, no `advances` needed, and the debrief wording.

### Surprises
Clap's help layout switched every `issue create` option to next-line help when the new flag's value name was long, failing a workflow_help test; the value name is `ID` and the help one short line.

### Open items
None. Backfilled by hand: the 56 follow-ups of deferred findings now name their finding, and 0a52, juyb and mg7w name atelier-r6bt. Removed the unused evidence `follow_up_ids` field.
