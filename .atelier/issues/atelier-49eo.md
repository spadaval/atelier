---
created_at: "2026-10-17T19:00:36.341930232+00:00"
id: "atelier-49eo"
issue_type: "bug"
labels:
- "issue-commands"
- "retro-2026-10-17"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Use a decision's Decision section as its close reason"
updated_at: "2026-10-17T19:00:36.341930232+00:00"
---

## Goal

`Transition 'decide' is blocked ... missing required field close_reason` right after a `## Decision` section had been written (b5ed7f93), so the decision was stated twice.

## Done when

`issue transition <id> decide` without `--reason` uses the body's Decision section when present, and refuses with a pointer to it when absent. A test covers both.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
