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
status: "in_progress"
title: "Refuse misspelled field lines in decision notes"
updated_at: "2026-11-04T23:29:12.585811580+00:00"
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
