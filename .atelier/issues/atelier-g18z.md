---
created_at: "2026-06-12T05:12:25.940409872+00:00"
id: "atelier-g18z"
issue_type: "task"
labels:
- "docs"
- "reliability"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-k9m8"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T23:06:44.821490789+00:00"
status: "done"
title: "Block closeout on docs help and tests drift"
updated_at: "2026-06-12T23:06:44.821490789+00:00"
---

## Description

Make documentation, help output, and tests part of reliability validation. A
mission should not close while these sources contradict each other.
- Closeout detects when docs and help list different command surfaces.
- Tests that assert obsolete behavior block closeout unless tied to an explicit
  migration window.
- Agent Factory guidance is checked against implemented commands.
- Drift findings name the file, command, or test that must change.
- Docs/help consistency checks or transcript tests compare the implemented
  command surfaces against Agent Factory guidance, repository docs, and CLI help.

- A stale-guidance check fails when docs, help, or tests still present obsolete
  commands as normal workflow without an explicit migration window.

- Run focused docs/help tests and record any intentionally deferred drift with a
  linked owner.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
