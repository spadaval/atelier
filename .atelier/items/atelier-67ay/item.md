---
schema: atelier.item
schema_version: 3
id: atelier-67ay
title: Move a replaced skill copy aside in setup --link instead of deleting it
status: dropped
mission: null
labels:
- agent-factory
- setup
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T06:14:31.018984919Z
---

## Goal

`atelier setup --link` deletes a real directory at `~/.claude/skills/agent-factory` or `~/.agents/skills/agent-factory` whenever the `skills` lock lists agent-factory, but the lock does not say which agent paths it installed. A hand-made directory with edits at the other path is lost (reproduced in review atelier-pak1).

## Done when

- [ ] `setup --link` renames a replaced real directory to `<path>.bak` (or `.bak.<n>` if taken) instead of deleting it, and prints where it went. A test puts a file in a lock-tracked copy, links, and finds the file under the backup path.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-pak1

## Report

Dropped: `setup --link` was removed in 0.5; `atelier setup` installs hooks only.
