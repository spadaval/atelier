---
created_at: "2026-11-04T23:03:20.457889128+00:00"
id: "atelier-l63c"
issue_type: "mission"
labels:
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Show agents what changed and what is in flight without rebuilding it from git"
updated_at: "2026-11-04T23:03:20.457889128+00:00"
---

## Outcome

A fresh session, a woken orchestrator or the human learns from Atelier's own views what changed in the tracker and what is really in flight: recent activity at session start, a since-commit digest of a mission for orchestrator wake-ups, planned versus built work and backlog health in `work missions`, and docs that exist only on unmerged branches. They stop reconstructing this from `git log` or by re-reading a whole mission's context.

## Background

Re-shaped on 2026-11-04 under ADR 0024. It began as the follow-on criteria split out of atelier-4pzj, which mixed this theme with a transcript measure, the `prime` removal and custom work views. The transcript measure was dropped, and atelier-3i7x, atelier-jjvc and atelier-e95f stand alone, because none of them shares this outcome and two wait on things this mission's work cannot finish. The items come from the 2026-10-17 retrospective of the 10-15..10-17 runs, where orchestrator wake-ups re-reading their whole context were the largest single line of spend (about 1.4B cache-read tokens).
