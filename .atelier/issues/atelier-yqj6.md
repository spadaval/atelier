---
created_at: "2026-06-13T20:37:13.456958042+00:00"
id: "atelier-yqj6"
issue_type: "task"
labels:
- "architecture"
- "cleanup"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-vu88"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Delete or isolate inherited lock sync and compatibility residue"
updated_at: "2026-06-13T22:55:08.963901128+00:00"
---

## Description

Inherited lock/sync and compatibility code still exists even though normal workflow uses local work association and removed command surfaces. Audit and remove dead paths, stale guidance, or references to commands that no longer exist.

## Outcome

- Removed lock/sync command behavior is not reachable or referenced by normal errors, docs, help, or tests.
- Any retained lock or sync code has an explicit current product owner and is isolated from normal work start/close paths.
- Compatibility tables, modules, and comments that no longer serve migration are deleted or assigned to a bounded migration issue.

## Evidence

- rg residue scan for locks, sync, compatibility, legacy, alias, fallback, and removed command names is attached.
- Focused tests prove start/worktree behavior uses current work association without remote lock assumptions.
- Deleted-code or isolation review records retained exceptions.
