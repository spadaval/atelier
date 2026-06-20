---
created_at: "2026-06-20T16:47:51.065222668+00:00"
id: "atelier-hdff"
issue_type: "epic"
labels:
- "command-surface"
- "cutting-pass"
- "removal"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-6hcl"
  - kind: "issue"
    id: "atelier-7eio"
  - kind: "issue"
    id: "atelier-bz8g"
  - kind: "issue"
    id: "atelier-p1yz"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Remove workspace and retired command machinery"
updated_at: "2026-06-20T21:58:28.562664117+00:00"
---

## Description

Remove command machinery that exists for retired workflow concepts or deferred
workspace features. This epic owns the visible worktree removal, stale command
modules, legacy translation tests, and follow-up module splits once deleted
surfaces are gone.

## Outcome

- The visible worktree feature is stripped pending redesign.
- Retired command modules and tests no longer preserve removed behavior.
- Public help, docs, and focused integration tests agree on removed commands.
- Remaining CLI modules are split only after dead surfaces are deleted.

## Evidence

- Removed commands fail as unknown commands.
- `target/debug/atelier --help` lists only supported command roots.
- Focused command-surface tests and `git diff --check` pass.
