---
schema: atelier.item
schema_version: 3
id: atelier-l9ty
title: Mark blocked and needs-human items in issue list
status: draft
mission: atelier-4pzj
labels:
- cli-output
- issue-commands
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.963954243Z
---

## Goal

The `issue list` Status column prints the raw workflow status (`crates/atelier-cli/src/human_output.rs:389-397`). atelier-sdb0, atelier-7u4o and atelier-deyy show as `todo`, while `work mission` and `issue show` say they are blocked by atelier-bw0b. Ready, blocked and needs-human items look the same. An agent filtering `--status todo` for something to pick up gets blocked items with no cue and has to run `issue show` on each one.

## Done when

- [ ] Each row keeps its status word and adds one cue where it applies, using the work classification: `todo ⚠ blocked by atelier-bw0b` (or `blocked by 2` when there are several), and `todo · needs a human`. Ready and done rows add nothing.
- [ ] A CLI test covers a blocked todo row, a needs-human row and a plain ready row. Redirected output stays plain text.

## Scope

`issue list` rendering only; classification logic is reused from `atelier_app::work_classification`.

## Context

The symbols follow atelier-deyy's rule (✓/⚠ next to the status word).
