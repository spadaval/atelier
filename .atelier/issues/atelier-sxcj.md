---
created_at: "2026-10-17T19:00:36.340695076+00:00"
id: "atelier-sxcj"
issue_type: "feature"
labels:
- "issue-commands"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Accept --body-file in issue create and issue update"
updated_at: "2026-10-17T19:00:36.340695076+00:00"
---

## Goal

Agents write bodies to temp files and pass `--body "$(cat ...)"`, or edit records and strategy docs with Python string replacement (e592938d did it three times).

## Done when

`issue create` and `issue update` accept `--body-file <path>` (and `-` for stdin), validated like `--body`. Tests cover file and stdin.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-cj54 (replace one body section), atelier-rz2l (import a drafted record).
