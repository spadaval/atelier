---
created_at: "2026-10-17T01:10:57.965486113+00:00"
id: "atelier-e6fl"
issue_type: "bug"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Refuse --reason on transitions that write no Report"
updated_at: "2026-11-07T21:03:34.812629181+00:00"
---

## Goal

`atelier issue transition <id> defer --reason "waiting on vendor"` succeeds and the reason is stored nowhere (checked 2026-11-07: the record does not contain it). `apply_transition` (crates/atelier-cli/src/commands/workflow.rs) writes the Report only on transitions that require `close_reason`; on any other, such as `plan`, `defer` or `start`, it drops `--reason` silently.

## Done when

- [ ] On a transition that does not require `close_reason`, `--reason` is refused before anything is written, with a message that names the transition and points at `atelier issue note <id> "…"`.
- [ ] CLI tests cover `defer --reason` (refused, record unchanged) and `supersede --reason` (unchanged).

## Context

Decision atelier-mzpn chose the refuse option, so `--reason` keeps one meaning: it writes the Report (CONTEXT.md, atelier-ml3r). The original's `block` example is gone; no policy on this host has a blocked status.
