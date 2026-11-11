---
schema: atelier.item
schema_version: 3
id: atelier-lehs
title: Remove the Impact section from issue show
status: dropped
mission: atelier-4pzj
labels:
- cli-output
- issue-commands
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.962696285Z
---

## Goal

`issue show --full` always prints an Impact section (`render_impact_section` in crates/atelier-cli/src/commands/issue.rs), built from `downstream_impact` (crates/atelier-app/src/record_index.rs). That function follows children plus `derived`/`caused-by`/`falsifies` links, but `caused-by` is not an accepted link role, so that branch never matches. On a done parent it relists the children already shown under Subissues, with advice to review them before "changing or closing" a closed record; on most items it prints `No downstream issues found.` Nothing in the transcripts reads it.

## Done when

- [ ] The Impact section is gone from `issue show` and `issue show --full`, and `downstream_impact` with it unless something else uses it.
- [ ] Rendering tests no longer expect it, and a test shows a done parent's `--full` view without it.

## Context

Decision atelier-mzpn chose option (a) of the original two: delete the section rather than narrow it to non-child dependents.

## Report

Dropped: `issue show` and its Impact section were removed in 0.5.
