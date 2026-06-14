---
created_at: "2026-06-13T22:59:14.676327671+00:00"
id: "atelier-50tm"
issue_type: "task"
labels:
- "clippy"
- "quality"
- "readiness"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T23:15:51.514725400+00:00"
status: "done"
title: "Triage large-function hotspots from clippy too_many_lines"
updated_at: "2026-06-13T23:15:51.514725400+00:00"
---

## Description

The first `cargo clippy --all-targets -- -W clippy::too_many_lines` baseline
reported large-function hotspots in production and test code. Triage those
findings into concrete refactors, justified local exceptions, or narrower
follow-up work instead of leaving them as an undocumented review warning.
- Current `too_many_lines` findings are inventoried by file and function.
- Each hotspot is resolved by refactor, documented exception, or explicit
  tracker follow-up.
- The repo's large-function review command can run without leaving ambiguous
  ownership for known hotspots.
- `cargo clippy --all-targets -- -W clippy::too_many_lines` transcript or a
  filtered review transcript is attached.
- An issue note, audit artifact, or follow-up issue list cites any remaining
  hotspots by file/function and owner issue ID.
- Focused tests or validation commands are attached for any refactors taken.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
