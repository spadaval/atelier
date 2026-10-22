---
created_at: "2026-10-22T19:58:07.743188662+00:00"
id: "atelier-yim7"
issue_type: "feature"
labels:
- "land"
- "start"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-81e5"
  - kind: "issue"
    id: "atelier-ecah"
  - kind: "issue"
    id: "atelier-mhx0"
  - kind: "issue"
    id: "atelier-mvzi"
  - kind: "issue"
    id: "atelier-yc4d"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Push and start work safely"
updated_at: "2026-10-22T19:58:17.506247375+00:00"
---

## Goal

A refused land can never be pushed, and a push names base commits that belong to another owner. `start` creates the owner branch from an explicit base from any checkout. It refuses a branch whose type prefix does not match, and refuses an item another live session holds unless told to take over.

## Done when

- Each child's Done when holds, shown by CLI tests, with independent review.
- Delivers atelier-k8h8 S10.

## Unknowns

How atelier-81e5 tells a live session from a stale lease (process ID, heartbeat or worktree lock). Decide inside the child and record it as a decision note.

## Background

atelier-ecah and atelier-mvzi change land in `crates/atelier-cli/src/commands/session.rs` and the hooks `setup claude` writes. atelier-yc4d, atelier-mhx0 and atelier-81e5 change `crates/atelier-cli/src/commands/start.rs`. The saved workarounds this retires are: check land's exit status before pushing, and start from a fresh type branch.
