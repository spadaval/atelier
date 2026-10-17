---
created_at: "2026-10-17T01:10:57.967000630+00:00"
id: "atelier-501y"
issue_type: "feature"
labels:
- "cli-output"
- "start"
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
title: "Show the item's title, acceptance and mission when start succeeds"
updated_at: "2026-10-17T01:10:57.967000630+00:00"
---

## Goal

A successful `atelier start` (`crates/atelier-cli/src/commands/start.rs:264-306`) prints the git command, `Applied transition start…`, `From/To`, `Branch`, `Base` and the static line "Commit the record changes with the work they describe." It never says what the work is: no title, Done when or mission. `-q` has no defined output. Agents cut out the line they need: `atelier start X 2>&1 | grep Branch` ran 29 times and `| tail -N` 43 times. When start branches from something other than the policy base (`Base: audit-scratch`), it prints that without any warning.

## Done when

- Success output starts `Started atelier-f0rr "Title" on task/atelier-f0rr (from master)`, followed by the Done when text (up to ~6 lines), then `Mission: atelier-1w8c "…"` (direct or via parent), then the existing needs-human notes.
- When the recorded base differs from the policy base branch, a warning line names both.
- `atelier -q start <id>` prints only the branch name.
- CLI tests cover the default output, a child of a parent, the base warning and `-q`.
