---
created_at: "2026-10-28T00:35:18.044404302+00:00"
id: "atelier-5tl2"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-5tl2
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7v9q"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Name the release installer wherever Atelier tells users how to install or upgrade"
updated_at: "2026-10-28T02:02:31.563818724+00:00"
---

## Goal

Make the release installer the documented install and upgrade path, keep the source command for source checkouts, and replace the stale changelog that still describes the SQLite model.

## Done when

`atelier_core::REINSTALL_COMMAND` guidance (and every message built from it, including `workflow_config_schema_newer`) names the installer one-liner and the source fallback, covered by tests asserting the text. README has an Install section with the one-liner. `docs/product/development-setup.md` and `AGENTS.md` Command References agree with it. `CHANGELOG.md` describes the Markdown-record tracker, with an entry for the first release. Delivers S4.

## Scope

`crates/atelier-core/src/lib.rs` reinstall text and its callers' tests, README, development-setup, AGENTS.md command references, CHANGELOG. Not the installer itself.

## Context

Strategy: `docs/strategies/release-distribution.md`. Installer URL comes from the dist config in the release issue (expected `https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh`). `rg 'cargo install' crates` finds the message sites.
