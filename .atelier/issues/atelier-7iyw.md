---
created_at: "2026-10-17T01:10:57.991130940+00:00"
id: "atelier-7iyw"
issue_type: "bug"
labels:
- "cli-output"
- "cross-cutting"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-7iyw
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T06:58:59.803494328+00:00"
status: "done"
title: "Date notes in the same timezone as other timestamps"
updated_at: "2026-10-17T06:58:59.803494328+00:00"
---

## Goal

`append_note(now.date_naive(), ...)` (`crates/atelier-cli/src/commands/issue.rs:1013`, `commands/bundle.rs:960`) dates notes with the UTC date. A note written at 21:00 EDT on 2026-10-16 is dated `2026-10-17`, while the same `issue show` prints `Created:  2026-10-16 20:31 -04:00` in local time. atelier-fdxj's note shows the confusion: `- 2026-10-17 [human] 2026-10-17: left out of the orientation mission ...`. Prune cutoffs (`prune.rs:871-1058`) also use UTC dates, while `Cutoff: before 2026-10-10` is shown with no zone.

## Done when

- Notes are dated in the local zone that `Created`/`Updated` display uses.
- Prune states the zone of its cutoff, or uses the local date.
- A test with a fixed clock just after UTC midnight and a negative offset dates the note with the local day.

## Context

Display format rule: see the output-contract item in this mission.

## Close Reason

Notes (issue note, bundle apply) and prune's retention cutoff, record comparisons and candidate 'latest' dates now use human_output::local_date, the local day display_timestamp shows; last_closed_mission reuses it and docs/product/human-cli-output.md states that a bare date is the local day. Proven by a fixed-clock unit test (2026-10-17T00:31Z at -04:00 dates the note 2026-10-16), an integration test that fails on the UTC-dated build and passes now (issue note under TZ=UTC+12 and UTC-14), and passing nextest atelier-wylr, fmt atelier-f3vi and diff-check atelier-xq2r.
