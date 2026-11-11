---
created_at: "2026-10-17T01:10:57.982115462+00:00"
id: "atelier-n831"
issue_type: "bug"
labels:
- "cli-output"
- "status"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Report branch and checkout facts in status in words, not git porcelain"
updated_at: "2026-10-17T01:10:57.982115462+00:00"
---

## Goal

`atelier status` states the branch in words, but on an item's branch it says nothing of how far the branch is behind the base, on the base it says nothing of how far it is ahead of its upstream (`on the base branch master` while `git status` shows `[ahead 1]`), a detached HEAD reads `on a detached HEAD` with no commit ID, and of uncommitted changes it names code (`Not committed: README.md`) and not record changes, although they need different actions (review findings atelier-qptk and atelier-864b, reproduced on the new binary in scratch clones).

## Done when

- [ ] On an item's branch, `status` says how many commits the branch is behind the base, with the command that catches up, e.g. `1 behind master (git merge master)`, when it is. On the base, it says how many commits the base is ahead of its upstream, e.g. `master, 1 ahead of origin/master`. A detached HEAD reads `detached at <sha8>`.
- [ ] `status` names uncommitted record changes separately from code, e.g. `records: 1 uncommitted (.atelier/items/atelier-orb2/item.md)` and `code: 1 uncommitted`.
- [ ] Tests cover a branch behind its base with both kinds of change, a base ahead of its upstream, and a detached HEAD.

## Context

The behind-base computation is in crates/atelier-cli/src/commands/session.rs:338-360 (`land_text`).

Rewritten for the new tool in the cutover sort (atelier-5zre): the new `status` already prints the branch in words and no Git porcelain, so those parts left; the record path is now `.atelier/items/<id>/item.md`.
