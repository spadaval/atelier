---
created_at: "2026-11-08T06:14:31.018984919+00:00"
id: "atelier-67ay"
issue_type: "bug"
labels:
- "agent-factory"
- "setup"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-pak1"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:49.024826725+00:00"
status: "declined"
title: "Move a replaced skill copy aside in setup --link instead of deleting it"
updated_at: "2026-11-11T03:55:49.024826725+00:00"
---

## Goal

`atelier setup --link` deletes a real directory at `~/.claude/skills/agent-factory` or `~/.agents/skills/agent-factory` whenever the `skills` lock lists agent-factory, but the lock does not say which agent paths it installed. A hand-made directory with edits at the other path is lost (reproduced in review atelier-pak1).

## Done when

- [ ] `setup --link` renames a replaced real directory to `<path>.bak` (or `.bak.<n>` if taken) instead of deleting it, and prints where it went. A test puts a file in a lock-tracked copy, links, and finds the file under the backup path.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new binary has no `setup --link` (`atelier setup --help` takes only a harness), so it never replaces a skill copy.
