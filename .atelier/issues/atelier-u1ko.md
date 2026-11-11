---
created_at: "2026-11-10T01:29:41.330899323+00:00"
id: "atelier-u1ko"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5zre"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mlwx"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-oci2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-t747"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Delete the old workspace and put the new one in its place, with the cutover runbook"
updated_at: "2026-11-10T04:55:19.564304978+00:00"
---

## Goal

Make the one reviewed change after which the tree holds only the new tool, and put everything the cutover will run under review before it runs.

## Done when

- [ ] Provisional: `crates/`, the old workspace with all its tests, and `scripts/smoke-repos.sh` are deleted, and the contents of `next/` are moved to the repository root, so the root workspace, the release build and `cargo install --path crates/atelier-cli` are the new tool; CI's `check` step runs the fallback release while `.atelier/issues/` exists and the tree's binary once it does not. `scripts/cutover/` holds `cutover.sh`, the new `AGENTS.md` text for each of the five repositories, and a replacement for each workflow that runs Atelier in a repository's CI, installing it from this change's commit. The rollback was tried on scratch clones at each stopping point `docs/strategies/redesign-architecture.md` lists, and the Report says what each run showed. `AGENTS.md` says the installed binary stays the tracker and nobody installs from the tree.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Protocol: `docs/strategies/redesign-architecture.md`, Transition, parts 3 and 4. This change is reviewed and merged under the old tool. `AGENTS.md` asks for `scripts/smoke-repos.sh` before review of a command change; this change deletes that script, so it is not run.

## Unknowns

- Whether a repository's forge requires a pull request for its base; the rehearsal reports it.
- Every file the move touches: the root `Cargo.toml` and `Cargo.lock`, the release profile and the dist metadata, CI's two sets of jobs and their working directories, and the web paths in `release.yml` and `.github/build-setup.yml`. Only tracked sources move, not `next/target` or `node_modules` (planning findings atelier-8609 and atelier-ncen).
- Whether installing Atelier in CI from a commit, as astromech does today, is enough, or a release of the new tool is needed (planning finding atelier-nwgr).
- How the runbook stops and disables `atelier-serve`, and on rollback enables it again and checks the old site answers (planning finding atelier-q8or; the new binary has no `serve`, ADR 0032).
- Which hook settings each repository needs. `atelier setup` writes `.claude/settings.json` in the repository it runs in, so the conversion commit of each repository must carry them and the rollback restore them (planning finding atelier-qe1q).
