---
schema: atelier.item
schema_version: 3
id: atelier-w4ef
title: Make check and check <id> apply one rule set
status: draft
mission: atelier-uf97
labels:
- cli-output
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T08:25:02.589462877Z
---

## Goal

`atelier check <id>` reports a work item whose Goal or Done when is missing or a placeholder (`Done when was not specified.`, which `issue create` writes when no body is given), but the whole-repository `atelier check` passes: the placeholder rule runs only when an ID is given (crates/atelier-app/src/lint.rs, `scoped`). Checked 2026-11-07. Since atelier-ilba a write is refused only for the check findings it adds, judged by the whole-repository rules, so the same gap lets through a write that `check <id>` would refuse. Separately, docs/architecture/quality/agent-factory-atelier-validation.md mentions `atelier check --fix`, which does not exist.

## Done when

- [ ] `check` and `check <id>` report the same findings for the same record, and atelier-ilba's write guard judges writes by that one rule set; a test covers an issue missing its required sections under both forms.
- [x] docs/architecture/quality/agent-factory-atelier-validation.md names only commands that exist (ADRs and dated audit records stay as written).

## Context

Found while implementing atelier-pr4k. Decision atelier-mzpn meant this for atelier-ilba, which merged without it.

## Unknowns

Whether a `backlog` item may keep placeholder sections (capture now, refine later); atelier-k55o settles it for `issue create`, and `check` follows the same answer. One record in this repository and one in astromech hold placeholders today; a rule that turns their `check` red needs a migration or a note naming them.

## Notes

- 2026-11-08 [note] The validation-doc line holds: b13d74e4 merged docs/no-check-fix (the first gardener run's routine upkeep, digest atelier-j86z), which removed check --fix and the export and rebuild diagnostics.
