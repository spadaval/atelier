---
created_at: "2026-11-10T00:56:08.329209632+00:00"
id: "atelier-pz1y"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-jciw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jq24"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add `check`, the Git hooks and the `ATELIER_SKIP` bypass, and `setup` for Git and Codex"
updated_at: "2026-11-10T02:49:55.088653752+00:00"
---

## Goal

Move Atelier's rules about Git actions to where Git acts, give every rule one intentional bypass, and orient Codex sessions as Claude sessions are oriented today.

## Done when

- [ ] Provisional: `check` validates every record. `atelier setup` installs the pre-commit, pre-push and post-checkout hooks, each a one-line call to `atelier check --hook <name>`, and `status` says when a clone has none. Scenario tests cover the hook cases the architecture specification's Testing section lists, including a direct change to `.atelier/config.toml` refused at push. `ATELIER_SKIP` with a reason passes any rule of a hook or a command, is printed, and puts a trailer on a commit Atelier makes; an empty reason is refused, and an unparseable record cannot be skipped. There is no `land` command, and `setup claude` writes the session-end hook as `atelier check`, which also reports record changes not yet committed. A Codex session in a fixture repository starts with `status` output.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- Which Codex hook event runs at session start.
- How `setup` keeps a hook a clone already has, and passes Git's hook inputs through to it (planning finding atelier-1l4r). If none fits, `AGENTS.md` says to run `atelier status` first.
