---
created_at: "2026-10-17T19:45:46.135278410+00:00"
id: "atelier-jnec"
issue_type: "bug"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep the debrief's history when HEAD is a work branch that merged the base"
updated_at: "2026-10-17T19:45:46.135278410+00:00"
---

## Goal

`issue_histories` in crates/atelier-app/src/record_history.rs walks HEAD's first-parent line, so on a work branch that merged master in, every master commit since the branch point collapses into that merge: the debrief moves the mission's start, and loses scope added after the start and reopened items, with no warning. Found by review atelier-2tbo on atelier-rx22.

## Done when

A debrief run from a work branch that merged the base reports the same start and deviations as one run from the base (walk the base's first-parent line, then the branch's own), shown by a CLI fixture.
