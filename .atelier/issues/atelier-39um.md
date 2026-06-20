---
created_at: "2026-06-20T17:02:38.162390498+00:00"
id: "atelier-39um"
issue_type: "task"
labels:
- "graph"
- "refactor"
- "removal"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Delete graph and tree rendering code after replacement views land"
updated_at: "2026-06-20T17:02:38.162390498+00:00"
---

## Description

Current state: `crates/atelier-cli/src/commands/tree.rs` implements graph/tree
rendering, mission tree expansion, blocker suffixes, work ordering, and compact
mission summaries for the `graph` command family. The command-collapse epic is
already moving the useful hierarchy, blocker, and impact questions into issue
and objective status views.

Desired state: once replacement issue/objective views cover the supported
questions, the old graph/tree rendering module, root dispatch, help text, and
tests are deleted instead of preserved as a parallel implementation.

Reason: keeping `tree.rs` after graph collapse would preserve the exact
special-case surface the cutting pass is trying to remove. The replacement
views should own hierarchy and impact questions directly.

Non-scope: this issue should not remove graph/tree behavior before
`atelier-ehit` proves the replacement view coverage.

## Outcome

- `graph tree`, `graph impact`, and their old tree-rendering implementation are
  absent unless the command contract explicitly retains a narrowed diagnostic
  surface.
- Reusable ordering or blocker helpers needed by replacement views are moved to
  the shared objective/status model instead of staying in `tree.rs`.
- Product docs, help text, and command audit contain no current-workflow
  guidance for the removed graph/tree commands.

## Evidence

- `target/debug/atelier graph tree --help` and any removed graph subcommands
  fail as unknown commands after replacement views land.
- `rg "commands::tree|graph tree|graph impact" crates docs .atelier/issues`
  shows only removal-history or intentionally retained diagnostic references.
- Focused CLI tests prove the replacement issue/objective views answer the
  former graph hierarchy, blocker, and impact questions.
