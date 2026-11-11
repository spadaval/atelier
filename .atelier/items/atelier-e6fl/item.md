---
schema: atelier.item
schema_version: 3
id: atelier-e6fl
title: Refuse --reason on transitions that write no Report
status: dropped
mission: atelier-uf97
labels:
- cli-output
- issue-commands
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.965486113Z
---

## Goal

`atelier issue transition <id> defer --reason "waiting on vendor"` succeeds and the reason is stored nowhere (checked 2026-11-07: the record does not contain it). `apply_transition` (crates/atelier-cli/src/commands/workflow.rs) writes the Report only on transitions that require `close_reason`; on any other, such as `plan`, `defer` or `start`, it drops `--reason` silently.

## Done when

- [ ] On a transition that does not require `close_reason`, `--reason` is refused before anything is written, with a message that names the transition and points at `atelier issue note <id> "…"`.
- [ ] CLI tests cover `defer --reason` (refused, record unchanged) and `supersede --reason` (unchanged).

## Context

Decision atelier-mzpn chose the refuse option, so `--reason` keeps one meaning: it writes the Report (CONTEXT.md, atelier-ml3r). The original's `block` example is gone; no policy on this host has a blocked status.

## Report

Dropped: Transitions were removed in 0.5.
