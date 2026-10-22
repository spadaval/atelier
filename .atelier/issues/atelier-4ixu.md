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
status: "todo"
title: "Commit a step's records together instead of one commit per record"
updated_at: "2026-10-22T19:58:19.748027082+00:00"
---

## Goal

64% of commits in the window touch only `.atelier/`; the median branch is 2 code commits wrapped in 6 records commits ('Record finding', 'Record disposition', 'Record evidence', 'Record fresh approval'). Agents commit each record by hand.

## Done when

Record-writing commands (review finding/disposition/approve, evidence record, transition) accept a `--commit` flag that stages and commits the records they wrote with a generated subject, and a sequence of them can be grouped into one commit (for example `atelier review record` writing several entries). `docs/product` describes it and a test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Unknowns

Whether auto-commit belongs in atelier or in the Agent Factory skill's guidance.
