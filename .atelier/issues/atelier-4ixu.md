---
created_at: "2026-10-17T19:00:36.352888255+00:00"
id: "atelier-4ixu"
issue_type: "feature"
labels:
- "evidence"
- "retro-2026-10-17"
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
status: "in_progress"
title: "Commit a step's records together instead of one commit per record"
updated_at: "2026-10-22T21:14:34.547229877+00:00"
---

## Goal

64% of commits in the window touch only `.atelier/`; the median branch is 2 code commits wrapped in 6 records commits ('Record finding', 'Record disposition', 'Record evidence', 'Record fresh approval'). Agents commit each record by hand.

## Done when

Record-writing commands (review finding/disposition/approve, evidence record, transition) accept a `--commit` flag that stages and commits the records they wrote with a generated subject, and a sequence of them can be grouped into one commit (for example `atelier review record` writing several entries). `docs/product` describes it and a test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Unknowns

Whether auto-commit belongs in atelier or in the Agent Factory skill's guidance.

## Notes

- 2026-10-22 [decision] Grouping uses --stage-records on earlier commands and --commit-records on the last; the flags are named for records because review already owns --commit (the pinned commit)
  Reason: A step is a sequence of separate commands, so grouping needs state between processes. Staging the records and remembering them in the git-ignored .atelier/runtime/staged-records.json lets the next --commit-records commit them as one commit, and lets the pre-write index check tell an earlier staged group from foreign staged changes.
  Rejected: Amending the previous record commit (rewrites history, can orphan review pins)
  Rejected: A batch command taking several entries (new input format to learn, and evidence capture runs arbitrary commands)
  Rejected: Deriving the records from git status (would sweep in unrelated dirty records)
- 2026-10-22 [decision] The records to commit come from the record store's write log, not from git status
  Reason: RecordStore::write_atomic is the single funnel for every record write, so logging its paths gives exactly what the command wrote, independent of other staged or unstaged work.
