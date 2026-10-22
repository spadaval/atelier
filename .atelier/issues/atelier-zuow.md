---
created_at: "2026-10-22T19:58:07.838889696+00:00"
id: "atelier-zuow"
issue_type: "feature"
labels:
- "evidence"
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-zuow
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-4ixu"
  - kind: "issue"
    id: "atelier-rc8p"
  - kind: "issue"
    id: "atelier-tq1j"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:37:47.825592359+00:00"
status: "done"
title: "Record and request review with less hand work"
updated_at: "2026-10-22T21:37:47.825592359+00:00"
---

## Goal

A step's records land in one commit, and `atelier review request` writes the reviewer brief. A follow-up records where it was found with a `discovered_from` link instead of prose.

## Done when

- Each child's Done when holds, shown by CLI tests, with independent review.
- Delivers atelier-k8h8 S9's record-commit part (atelier-4ixu) and S8's tooling part (atelier-rc8p, atelier-tq1j).

## Background

The children change the review, evidence and issue-create commands and the debrief, none of which the other first-wave increments touch. Mission atelier-rc8j later reworks how these commands report; keep output changes to what each child needs.

## Close Reason

### Outcome
A step's records can land in one commit: record-writing commands take `--stage-records` and `--commit-records`, which stage or commit only the records that command wrote (atelier-4ixu). A follow-up records where it was found with a `discovered_from` link, shown by `issue show`, checked by `check`, and used by the debrief (atelier-tq1j). `atelier review request <id>` prints the reviewer brief from records and policy (atelier-rc8p). Delivers atelier-k8h8 S9's record-commit part and S8's tooling part.

### Deviations
The flag is `--commit-records`, not `--commit`, because `review` already uses `--commit` for the pinned commit. 56 existing follow-up issues were backfilled with `discovered_from`, beyond tq1j's Done when, so the debrief places past follow-ups too. This repository's `[review] rules_file` is not set yet, because the installed binary rejects unknown config tables.

### Decisions
See the decision notes on atelier-4ixu, atelier-tq1j and atelier-rc8p.

### Surprises
None beyond the flag name clash.

### Open items
After merge and reinstall, set `[review] rules_file` to a committed file holding this host's reviewer rules. `evidence record --commit-records` pins the commit before its own record commit. The test helper `run_atelier` commits dirty state before some commands.
