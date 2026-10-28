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
closed_at: "2026-10-28T02:16:57.843296469+00:00"
status: "done"
title: "Name the release installer wherever Atelier tells users how to install or upgrade"
updated_at: "2026-10-28T02:16:57.843296469+00:00"
---

## Goal

Make the release installer the documented install and upgrade path, keep the source command for source checkouts, and replace the stale changelog that still describes the SQLite model.

## Done when

`atelier_core::REINSTALL_COMMAND` guidance (and every message built from it, including `workflow_config_schema_newer`) names the installer one-liner and the source fallback, covered by tests asserting the text. README has an Install section with the one-liner. `docs/product/development-setup.md` and `AGENTS.md` Command References agree with it. `CHANGELOG.md` describes the Markdown-record tracker, with an entry for the first release. Delivers S4.

## Scope

`crates/atelier-core/src/lib.rs` reinstall text and its callers' tests, README, development-setup, AGENTS.md command references, CHANGELOG. Not the installer itself.

## Context

Strategy: `docs/strategies/release-distribution.md`. Installer URL comes from the dist config in the release issue (expected `https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh`). `rg 'cargo install' crates` finds the message sites.

## Close Reason

The stale-binary fix now says "upgrade to the latest release with `<installer one-liner>`, or from Atelier's source checkout run `cargo install --path crates/atelier-cli --locked`". README has an Install section, development-setup, AGENTS.md and sessions-and-hooks agree with it, and CHANGELOG describes the Markdown-record tracker as the first release's entry. Exact-text tests pass (atelier-r2ea); CI 40618793997 passed.

### Deviations
`docs/product/sessions-and-hooks.md` also quoted the old step, so it was updated too. The issue named only development-setup and AGENTS.md.

### Decisions
The installer comes first and the source command second, in one line, so that one phrasing serves every repository as atelier-u4ql decided. The changelog entry stays under `[Unreleased]` until the human picks the version.

### Surprises
None.

### Open items
- atelier-vy5i: rename `[Unreleased]` to the chosen version, and tag a stable (non-prerelease) version, because `latest/download` skips prereleases.
- atelier-pg2b: Agent Factory's install step should point at the installer.
- The installer URL returns 404 until the first release is published.
