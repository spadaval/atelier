---
created_at: "2026-11-08T18:09:03.511948849+00:00"
id: "atelier-xe9g"
issue_type: "task"
labels:
- "land"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-8v6s"
    type: "implements"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remove land's unpushed-base-commits warning"
updated_at: "2026-11-08T18:09:03.511948849+00:00"
---

## Goal

Land stops warning about unpushed base commits, so an agent pushes landed work without asking the human (atelier-8v6s).

## Done when

- [ ] `foreign_base_commits_note` and its call in `crates/atelier-cli/src/commands/session.rs` are gone, as is its test in `crates/atelier-cli/tests/session.rs`.
- [ ] `docs/product/sessions-and-hooks.md` no longer describes the warning.
- [ ] Formatting passes and the `land` tests pass.

## Context

Decision atelier-8v6s reverses atelier-mvzi's warning.
