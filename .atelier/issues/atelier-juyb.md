---
created_at: "2026-10-17T20:10:21.556378400+00:00"
id: "atelier-juyb"
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
  - kind: "issue"
    id: "atelier-r6bt"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:30:23.390293428+00:00"
status: "done"
title: "Refuse misspelled field lines in decision notes"
updated_at: "2026-11-05T00:09:41.548358140+00:00"
---

## Goal

A decision note line such as `Rejectd: x` passes `check` and is folded into the reason text (independent validation of atelier-r6bt, S1).

## Done when

`check` refuses an indented line in a decision note that looks like a misspelled `Reason:` or `Rejected:` label, or the docs say why it is allowed, shown by a CLI test.

## Notes

- 2026-11-04 [decision] A misspelled label is a continuation line whose leading Word: matches Reason: or Rejected: ignoring case, or is within two edits of either, but is not the exact label
  Reason: Catches typos and case slips while leaving ordinary prose lines such as Note: or Risk: alone; the check is a nudge, so a rare false refusal is fixed by rewording
  Rejected: refusing any Word: continuation line
  Rejected: documenting that misspellings are allowed
- 2026-11-04 [decision] Narrow the misspelled-label rule: ignoring case, the word starts with the label's first letter and is at most one insertion, deletion, substitution or adjacent transposition from Reason or Rejected, and is not the exact label
  Reason: Review finding atelier-nu4k: two Levenshtein edits refused ordinary words such as Person:, Region:, Lesson:, Selected: and Detected:, and distance 1 alone still refuses Season:; the narrowed rule still catches Rejectd, Reasons, Raeson, reason and REJECTED
  Rejected: a case-insensitive exact match only, which misses Rejectd and Raeson

## Close Reason

`check` now refuses a decision note with a line after the first that starts with a misspelled `Reason:` or `Rejected:` label, such as `Rejectd:` or `reason:`, naming the label, instead of folding it into the reason text; ordinary prose lines such as `Note:` still pass. A CLI test shows it.

### Deviations

None.

### Decisions

The near-miss rule is a decision note on this item; review finding atelier-nu4k narrowed it from two edits to one edit with the same first letter, recorded in a later note.

### Surprises

Before the change, a misspelled `reason:` line on a note with no real reason was reported as "has no reason", which pointed away from the typo; the misspelled-label finding now comes first.

### Open items

None. The repository's own records pass the stricter `check`.
