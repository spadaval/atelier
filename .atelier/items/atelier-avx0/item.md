---
schema: atelier.item
schema_version: 3
id: atelier-avx0
title: Add a since-commit digest of a mission for orchestrator wake-ups
status: dropped
mission: atelier-4pzj
labels:
- orientation
- retro-2026-10-17
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T19:00:36.351780477Z
---

## Goal

o3lm had 75 subagent-completion wake-ups and each re-read a ~400k-token context; the adoption run's main thread re-read 0.5-1M tokens on about 1,500 turns (about 1.4B cache-read tokens), the largest single line of spend. A fresh session likewise has no summary of what merged or changed in the tracker recently and rebuilds it from `git log` (atelier-majk, merged here).

## Done when

- [ ] `atelier work mission <id> --since <commit>` prints only what changed since that commit: items started, merged, superseded or filed; findings and their decisions; approvals; loose ends filed or done; and what is next. A rendering test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). `--since <commit>` answers atelier-majk's open question of what "recent" means without session records (ADR 0014/0019): the caller names the commit it last saw. Kept in decision atelier-mzpn for its cost evidence.

## Unknowns

Whether status also prints a short digest at session start, and since which commit (the upstream, or the last commit the session saw); `git log` or `git diff --stat` since the commit may be enough there.

## Report

Dropped: `work mission --since` names a removed view. `atelier log <mission> --since <time>` (atelier-8b0a) now prints what changed since a point.
