---
created_at: "2026-11-10T01:39:35.640973419+00:00"
id: "atelier-mlwx"
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
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Fix the old tool's fallback release: 0.4.0 if it passes, else a new 0.4.1"
updated_at: "2026-11-10T01:48:07.663576792+00:00"
---

## Goal

Fix which release of the old tool a rollback reinstalls and CI's `check` step runs, before anything in the tree changes the old binary. It must do what this host's installed binary does today, dashboard included.

## Done when

- [ ] Provisional: the 0.4.0 release, installed in a scratch directory, is tried first: if it passes `atelier check` on this repository's records and serves the site, it is the fallback and nothing is released. If it fails, the Report names what it lacks, and release 0.4.1 is published from a commit where the old `atelier` binary is unchanged and passes the same test.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Protocol: `docs/strategies/redesign-architecture.md`, Transition, part 3. A release takes the light path under the old tool.

## Unknowns

- Whether 0.4.0 passes. The installed binary is 0.4.0 plus six commits, and planning finding atelier-xzaf found those commits change only the review brief's wording.
