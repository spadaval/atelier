---
schema: atelier.item
schema_version: 3
id: atelier-c6g2
title: Teach Agent Factory to use the mission digest and check's skipped-graph notice
status: dropped
mission: null
labels:
- agent-factory
priority: low
blocked_by:
- atelier-avx0
- atelier-fb1i
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T06:03:09.874332233Z
---

## Goal

Two Atelier items had Done when clauses that needed a change in the Agent Factory skill, which lives in spadaval/.agents (ADR 0023). Those clauses moved here when the missions were regrouped on 2026-11-05, so that atelier-l63c and atelier-2mky can finish on this repository's work alone.

## Done when

- [ ] Once atelier-avx0 lands, the skill tells orchestrators to read `work mission <id> --since <commit>` on wake-up and to ask subagents for short reports.
- [ ] Once atelier-fb1i lands, the Administration section of `skills/agent-factory/references/atelier.md` states how `check` reports graph errors when a record fails to parse.
- [ ] Both edits are committed in spadaval/.agents.

## Report

Dropped: Both commands it waits on are dropped (atelier-avx0, atelier-fb1i), and the skill file it would edit was deleted by atelier-orb2.
