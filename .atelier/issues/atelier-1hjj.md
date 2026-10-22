---
created_at: "2026-10-22T20:59:30.576687438+00:00"
id: "atelier-1hjj"
issue_type: "bug"
labels:
- "merge"
priority: "P3"
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
closed_at: "2026-10-22T23:26:10.652513006+00:00"
status: "done"
title: "Print merge's report only from the attempt that returns"
updated_at: "2026-10-22T23:26:10.652513006+00:00"
---

## Goal

`atelier merge` checks whether the base moved by records-only commits once before printing its gate report (crates/atelier-cli/src/commands/merge.rs, around lines 530-541) and again after taking the mutation locks (around 586-589). If the base moves between the two, the report has already printed and the attempt still retries, so `--json` stdout holds two arrays. The window is now microseconds plus any lock wait. Found in review of atelier-c3kh (finding atelier-t9ew).

## Done when

A records-only base move at any point before integration yields exactly one printed report, from the attempt that returns: either take the locks before the pre-print check, or hold the report and print it only on return. A test that moves the base after the pre-print check, using a test hook, asserts stdout parses as one JSON document.

## Notes

- 2026-10-22 [decision] Take the locks, then read the base once and print the report, instead of holding the report
  Reason: The pre-print check ran before the locks, so a move in between printed a report and then retried. Printing after the locked base read leaves one report per returning attempt, with the print path the only code touched (the claim-check block is untouched, for the atelier-r28p merge). A non-retry move (code commit, rewrite, base gone) still prints the report before its refusal, as before; the base-ref check moved ahead of the snapshot-clean and head checks, which changes only which refusal shows first when several apply.
  Rejected: Hold the report in a String and print on return (touches every later bail); a second pre-print check (leaves the window)

## Close Reason

`merge` now prints its gate report once, after taking the mutation locks and reading the base under them, so a records-only base move before integration retries without a report and `--json` stdout holds one document. A non-retry move still prints the report before its refusal.

### Deviations
None.

### Decisions
See the decision note on atelier-1hjj: take the locks, then read the base and print, rather than hold the report. The base-ref check now runs before the clean-snapshot and head checks, which only changes which refusal shows first when several apply.

### Surprises
None.

### Open items
Pre-existing clippy warnings in start.rs and tests/session.rs were not touched. A merge that fails to take its locks no longer prints the report first.
