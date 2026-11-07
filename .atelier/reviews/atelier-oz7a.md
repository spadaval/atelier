---
schema: atelier.review
schema_version: 2
id: atelier-oz7a
title: 'AGENTS.md:8-9 says ''Open missions and their strategies are in atelier work missions'', but the work missions overview lists only id/status/title/scope counts (checked: atelier-r6bt shows no strategy); the strategy path appears only in ''atelier work mission <id>''. An agent following AGENTS.md will not find the strategy where it is told to look. Fix: ''Open missions are in atelier work missions; atelier work mission <id> shows each one''s strategy.'''
target: atelier-bw0b
identity: Independent reviewer 3 (Claude subagent)
commit: 6c51447373552e58ab54acc8783d3a64c6a9991e
created_at: 2026-10-16T23:56:15.168389530Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (author)
    commit: 30dffbfe201ad35937d483d46fa38f5aa9d125ee
    recorded_at: 2026-10-16T23:57:13.902163564Z
    resolution_commit: 30dffbfe201ad35937d483d46fa38f5aa9d125ee
    reason: AGENTS.md points to work mission <id> for strategies
    migrated_from: atelier-jbuv
---

AGENTS.md:8-9 says 'Open missions and their strategies are in atelier work missions', but the work missions overview lists only id/status/title/scope counts (checked: atelier-r6bt shows no strategy); the strategy path appears only in 'atelier work mission <id>'. An agent following AGENTS.md will not find the strategy where it is told to look. Fix: 'Open missions are in atelier work missions; atelier work mission <id> shows each one's strategy.'
