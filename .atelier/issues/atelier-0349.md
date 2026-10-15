---
created_at: "2026-10-15T23:47:14.043094544+00:00"
id: "atelier-0349"
issue_type: "bug"
labels:
- "adoption"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make atelier land fast enough for the Stop hook"
updated_at: "2026-10-15T23:47:14.043094544+00:00"
---

## Goal

`atelier land` takes about 60 seconds on this repository (89 reviews, a few hundred records), on master with nothing to check, while `prime` takes 0.1 seconds. The Claude Stop hook runs `land` at every stop, so every agent turn waits a minute. Most of the time is system time, which suggests many Git subprocesses.

## Done when

The cause is diagnosed and `land` on this repository's master completes in a few seconds, with a test or benchmark that would catch a return to per-record Git calls.

## Context

Measured 2026-10-15 with both the installed binary (68911771) and current master: real 1m0s, user 17s, sys 45s. The adoption reviews also measured 56-67 s in scratch clones.
