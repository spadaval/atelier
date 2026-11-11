---
schema: atelier.item
schema_version: 3
id: atelier-wg6p
title: Don't turn record mentions inside paths and branch names into chips
status: dropped
mission: null
labels:
- review
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T04:40:10.556389088Z
---

## Goal

Chips split paths and branch names: '~/.cache/[atelier-g6a1]/s4-check.sh', 'task/[atelier-ot1c] e2e2cf0d', 'bug/[atelier-vo0z] e70496f7'. Mentions inside a path or ref should stay text.

## Done when

- [ ] A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown debrief and the site's debrief page. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)

## Report

Dropped: The debrief page was deleted with the site in 0.5.
