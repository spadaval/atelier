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
title: "Point `atelier` at the new commands and delete the old ones, with the cutover runbook"
updated_at: "2026-11-10T01:39:35.085413116+00:00"
---

## Goal

Make the one reviewed change after which the tree holds only the new command layer, and put everything the cutover will run under review before it runs.

## Done when

- [ ] Provisional: The `atelier` binary target points at the new command layer; the old command layer, its integration tests and `scripts/smoke-repos.sh` are deleted; CI's `check` step runs the 0.4.1 release while `.atelier/issues/` exists and the tree's binary once it does not. `scripts/cutover/` holds `cutover.sh`, the new `AGENTS.md` text for each of the five repositories, and a replacement for each workflow that runs Atelier in a repository's CI, installing it from this change's commit. The rollback was tried on scratch clones at each stopping point `docs/strategies/redesign-architecture.md` lists, and the Report says what each run showed. `AGENTS.md` says the installed binary stays the tracker and nobody installs from the tree.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Protocol: `docs/strategies/redesign-architecture.md`, Transition, parts 3 and 4. This change is reviewed and merged under the old tool. `AGENTS.md` asks for `scripts/smoke-repos.sh` before review of a command change; this change deletes that script, so it is not run.

## Unknowns

- Whether a repository's forge requires a pull request for its base; the rehearsal reports it.
- Whether installing Atelier in CI from a commit, as astromech does today, is enough, or a release of the new tool is needed (planning finding atelier-nwgr).
- How the runbook restarts `atelier-serve` and checks it, and restores it on rollback (planning finding atelier-q8or).
