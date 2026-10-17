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

`git_state` (crates/atelier-cli/src/commands/status.rs:182-200) copies `git status --short --branch` output verbatim:
- the branch line can read `Branch  : master...origin/master [ahead 1]`, `Branch  : HEAD (no branch)` or `Branch  : No commits yet on master`;
- the checkout line can read `Checkout: dirty (8 paths:  M .atelier/issues/atelier-43m1.md,  M crates/…/init.rs, ?? .atelier/issues/atelier-vhkt.md, 5 more omitted)`, with porcelain codes and a double space.

Records and code are not separated, although they need different actions. On a work branch status never reports how far the branch is behind its base. `land` computes that and prints `Base master has 1 commit(s) this branch does not…`, but status shows only `Branch  : task/atelier-7b6e`.

## Done when

- The branch line reads like `Branch: task/atelier-7b6e, 1 behind master (git merge master)` or `Branch: master, 1 ahead of origin/master`. A detached HEAD reads `detached at <sha8>`.
- The checkout line splits `records: 2 uncommitted (.atelier/issues/atelier-43m1.md, …)` from `code: 6 uncommitted`, with no porcelain codes.
- A test covers ahead, behind, detached, records-only and code-only states.

## Context

The behind-base computation is in crates/atelier-cli/src/commands/session.rs:338-360 (`land_text`).
