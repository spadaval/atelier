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
title: "Cut release 0.4.1 of the old tool, the fallback the rollback and CI use"
updated_at: "2026-11-10T01:39:35.640973419+00:00"
---

## Goal

Publish the old tool one last time, complete, before anything in the tree changes the old binary, so that a rollback can reinstall exactly what this host runs today, dashboard included.

## Done when

- [ ] Provisional: release 0.4.1 is published from master by the existing release process, from a commit where the old `atelier` binary is unchanged and still has `serve`. Installed in a scratch directory, it passes `atelier check` on this repository's records and serves the site.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Protocol: `docs/strategies/redesign-architecture.md`, Transition, part 3. A release takes the light path under the old tool.

## Unknowns

- None expected. The installed binary today is 0.4.0 plus six commits, so 0.4.0 itself is not the fallback.
