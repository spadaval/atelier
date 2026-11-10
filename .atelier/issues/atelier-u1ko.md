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
title: "Point `atelier` at the new commands and delete the old ones, with the old tool's last release and the cutover runbook"
updated_at: "2026-11-10T01:29:41.330899323+00:00"
---

## Goal

Make the one reviewed change after which the tree holds only the new command layer, and put everything the cutover will run under review before it runs.

## Done when

- [ ] Provisional: release 0.4.1 of the old tool is cut from master first. The `atelier` binary target points at the new command layer; the old command layer, its integration tests and `scripts/smoke-repos.sh` are deleted; CI's `check` step runs the 0.4.1 release while `.atelier/issues/` exists and the tree's binary once it does not. `scripts/cutover/` holds `cutover.sh` and the new `AGENTS.md` text for each of the five repositories. Each row of the rollback table in `docs/strategies/redesign-architecture.md` was tried on scratch clones, and the Report says what each run showed. `AGENTS.md` says the installed binary stays the tracker and nobody installs from the tree.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Protocol: `docs/strategies/redesign-architecture.md`, Transition, parts 3 and 4. This change is reviewed and merged under the old tool. `AGENTS.md` asks for `scripts/smoke-repos.sh` before review of a command change; this change deletes that script, so it is not run.

## Unknowns

- Whether a repository's forge requires a pull request for its base; the rehearsal reports it.
