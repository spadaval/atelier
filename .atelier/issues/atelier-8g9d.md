---
created_at: "2026-10-17T01:10:57.983055076+00:00"
id: "atelier-8g9d"
issue_type: "task"
labels:
- "cli-output"
- "status"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Name the missions behind status's mission count"
updated_at: "2026-10-17T01:10:57.983055076+00:00"
---

## Goal

The status summary is:
```
Ready work:    6
Needs a human: 1
Current work:  none
Current missions: 2
```
`Current missions: 2` is a bare count, so the reader needs `atelier work missions` to learn what the two missions are. The label also breaks the alignment of the block. `atelier status --help` says "Show checkout, mission, work, and tracker signposts", but status names no mission. `--quiet` prints `work=none current_missions=2 ready=6 human=1`, using `none` where every other key is a number (status.rs:14-24).

## Done when

- Each current mission appears as one line, for example `atelier-1w8c draft high  Orient a session from one view… 2 ready, 3 blocked`. The alternative is to remove the count; pick one.
- The summary labels align.
- `--quiet` prints `work=0` when nothing is active.
- Tests are updated.

## Context

atelier-ihns covers the between-missions state, and atelier-sdb0 adds the owner's mission. Coordinate so a mission appears once.
