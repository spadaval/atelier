---
created_at: "2026-10-17T19:00:36.351780477+00:00"
id: "atelier-avx0"
issue_type: "feature"
labels:
- "orientation"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add a since-commit digest of a mission for orchestrator wake-ups"
updated_at: "2026-10-17T19:00:36.351780477+00:00"
---

## Goal

o3lm had 75 subagent-completion wake-ups and each re-read a ~400k-token context; the adoption run's main thread re-read 0.5-1M tokens on about 1,500 turns (about 1.4B cache-read tokens), the largest single line of spend.

## Done when

`atelier work mission <id> --since <commit>` prints only what changed: items started, closed, reviewed, merged, new findings and follow-ups, and what is next. The Agent Factory skill tells orchestrators to use it and to ask subagents for short reports. A rendering test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
