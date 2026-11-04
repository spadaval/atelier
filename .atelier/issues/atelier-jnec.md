---
created_at: "2026-10-17T19:45:46.135278410+00:00"
id: "atelier-jnec"
issue_type: "bug"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "review"
    id: "atelier-2tbo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Keep the debrief's history when HEAD is a work branch that merged the base"
updated_at: "2026-11-04T23:24:46.564844109+00:00"
---

## Goal

`issue_histories` in crates/atelier-app/src/record_history.rs walks HEAD's first-parent line, so on a work branch that merged master in, every master commit since the branch point collapses into that merge: the debrief moves the mission's start, and loses scope added after the start and reopened items, with no warning. Found by review atelier-2tbo on atelier-rx22.

## Done when

A debrief run from a work branch that merged the base reports the same start and deviations as one run from the base (walk the base's first-parent line, then the branch's own), shown by a CLI fixture.

## Notes

- 2026-11-04 [decision] Take the base from the workflow policy's base_branch and walk its first-parent line only up to the merge base with HEAD, then HEAD's own line (fork..HEAD); with no such branch, or on the base, walk HEAD alone as before
  Reason: The debrief describes HEAD, so base commits HEAD lacks stay out; the policy's base is what every debrief shares, and a missing ref keeps fixtures and detached checkouts working
- 2026-11-04 [decision] Keep the branch's own commits from before its merge of the base in the walk, after the base line
  Reason: They hold the branch's own record changes, which the merge's first-parent diff does not repeat; dropping them would lose those changes, and a stale state only misleads when the base and the branch both changed one record before the merge
