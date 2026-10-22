---
created_at: "2026-10-17T19:00:36.342496349+00:00"
id: "atelier-auca"
issue_type: "bug"
labels:
- "evidence"
- "retro-2026-10-17"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Stop evidence records ending with a trailing blank line"
updated_at: "2026-10-17T19:00:36.342496349+00:00"
---

## Goal

New evidence records end with an extra blank line at EOF. Reviewers raised it as a finding three times (7u4o, ihns, deyy), each time costing a disposition.

## Done when

Records written by `evidence record` end with exactly one newline; `git diff --check` is clean on them. A test pins it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
