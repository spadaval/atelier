---
created_at: "2026-11-08T06:31:25.892632238+00:00"
id: "atelier-u0ai"
issue_type: "task"
labels:
- "tooling"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-x2px"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:57.337783163+00:00"
status: "declined"
title: "Run review request in the smoke script for repositories whose open items have no branch yet"
updated_at: "2026-11-11T03:55:57.337783163+00:00"
---

## Goal

`scripts/smoke-repos.sh` runs `review request` only on branches an unfinished item owns, so a repository with open items but no started branch (cosplay-studio today) gets no `review request` run.

## Done when

- [ ] For a repository with no branch owned by an unfinished item, the script runs `review request` once for one open item, and both binaries' results are compared like the other commands.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It runs `review request` in `scripts/smoke-repos.sh`, and the What is removed table removes the `review` subcommands; the switch deletes the script (architecture specification, Transition part 3).
