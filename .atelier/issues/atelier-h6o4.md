---
created_at: "2026-10-17T01:10:57.977741388+00:00"
id: "atelier-h6o4"
issue_type: "bug"
labels:
- "cli-output"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-uv5d"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T14:35:41.926425951+00:00"
status: "done"
title: "Show review list as one readable line per event, in time order"
updated_at: "2026-10-23T14:35:41.926425951+00:00"
---

## Goal

`atelier review list` is hard to read, and for short or unknown IDs it prints nothing at all. Rows use Rust Debug formatting (`{:?}` at crates/atelier-cli/src/commands/review.rs:118):

```
atelier-1isf Disposition { finding: "atelier-xs5u", decision: FixNow, supersedes: None, resolution_commit: Some("d1c3b1bf…"), follow_up: None } atelier-bw0b Claude Opus 5.5 (author) sdb0 Done when…
```

Each row also repeats the owner given as the filter, prints the whole multi-paragraph summary on one line (atelier-bw0b rows run past 1,000 characters), leaves out the pinned commit and date, and is sorted by random event ID rather than by time. `--quiet` is accepted and ignored. The filter compares strings exactly (`record.target == id`, review.rs:111), so `atelier review list nqgr` prints nothing and exits 0 even though atelier-nqgr has 10 events. `atelier review list atelier-doesnotexist` also prints nothing and exits 0. Agents go around the view instead: the cross-cutting audit counted 106 `ls/grep/cat .atelier/reviews` commands in transcripts.

## Done when

- Rows are in chronological order and look like `atelier-1isf  disposition fix_now → atelier-xs5u  @d1c3b1bf  Claude Opus 5.5 (author)  2026-10-16  sdb0 Done when: setup claude warns…` (summary cut to its first line). With an owner filter, the owner column is dropped. No `Some(`, `None` or `{` appears in the output.
- `review list nqgr` lists the same events as `review list atelier-nqgr`. An unknown ID errors (`issue atelier-doesnotexist not found`) with a nonzero exit. A known owner with no events prints `No review events for atelier-xxxx.`
- `--quiet` prints one event ID per line.
- Tests pin the row format, short-ID resolution, the unknown-ID error and quiet output.

## Scope

`--json` stays as it is (ADR 0022 keeps it for scripts). The same renderer should serve `prime`/`status`, which print the Debug text today.

## Context

atelier-sdb0 (status shows the owner's reviews) should reuse this renderer. ADR 0022 Decision 3.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done: short IDs and unknown IDs now resolve (review.rs:139; drop those claims). Still: rows use {:?} (review.rs:167-170), multi-line summaries spill, ID order not time order, no pin/date, owner column repeated, a known owner with no events prints nothing. Reuse the renderer at status.rs:323 (also {:?}). 'prime/status' is now just status.
- 2026-10-23 [decision] review list <id> lists the events of the issue's branch owner scope (review_gate::scope), naming the target of events on children with 'on <id>'; --json keeps the stored records whose target is the issue, as before.
  Reason: The gates judge findings across the owner's scope; --json is the script interface ADR 0022 keeps unchanged.
  Rejected: Filtering rows to target == id
- 2026-10-23 [decision] Rows use the stored event names (finding, disposition, trivial_exemption) and cut the summary's first line at 100 characters.
  Reason: Stored names grep the same as the records; long single-paragraph summaries still made rows of several hundred characters, and review show prints the whole text.

## Close Reason

`atelier review list` prints one row per event in time order (`<id>  <kind>[  on <target>]  @<pin>  <identity>  <date>  <summary first line>`), with no Debug text; with an issue the owner column is dropped, a known owner with no events prints `No review events for <id>.`, `--quiet` prints the listed IDs, and `status`'s Reviews rows use the same renderer.

### Deviations
Short-ID resolution and the unknown-ID error were already on master (regroom note), so only tests were added for them. `review list <id>` now lists the branch owner's whole scope, naming the target of events on children; `--json` is unchanged. Without an ID, base exemptions are merged into the time order instead of following the events. Summary lines are also cut at 100 characters.

### Decisions
See the decision notes on this item (scope-wide rows; stored event names and the 100-character cut).

### Surprises
None.

### Open items
None.
