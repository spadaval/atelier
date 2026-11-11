---
schema: atelier.item
schema_version: 3
id: atelier-kset
title: Print each check finding once, on one line, against its record, and say what check checked
status: draft
mission: atelier-uf97
labels:
- check
- cli-output
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.971709219Z
---

## Goal

`atelier check` findings are noisy and mislabelled. Checked 2026-11-07 on a record with an invalid status and another with an `advances` link to a missing issue:
- The invalid status is printed twice, once labelled `records:` and once under the issue's ID (crates/atelier-app/src/lint.rs). An invalid issue_type is the same (atelier-ry7d, merged here).
- The missing reference is labelled `records:` instead of the linking record.
- Every `records:` finding is prefixed `Tracker record files are invalid:`.
- Workflow diagnostics render as a four-line block (`error: workflow_issue_status_invalid` / `path:` / `reference:` / `message:`).
- YAML line numbers count from the start of the front matter, so they are one less than the file line.
- An invalid workflow policy is printed twice, once as the `Invalid working policy` load warning and once as the finding.

A passing check says only `Check passed.`, the same for the whole repository and for `check <id>`, naming neither the record nor the count; a failure's last line is `Error: check_findings: check failed with N findings…`, so `| tail -1` keeps the count and drops the finding (atelier-4t5l, merged here). Agents piped `atelier check` through `tail` 166 of 323 times in the transcripts.

## Done when

- [ ] Each fault prints once, on one line, against the record to fix: `✗ atelier-uyuu  .atelier/issues/atelier-uyuu.md:18  status 'frobnicated' is not valid for task; use backlog, todo, …`. A test pins the count for an invalid status and an invalid issue_type.
- [ ] A missing reference is attributed to its linking record, and YAML line numbers are file-relative; tests cover both.
- [ ] A whole-repository pass prints `✓ check: 742 records valid` (the actual count), and `check <id>` prints `✓ atelier-wtnp valid`.
- [ ] A failure's last line names the first problem and the next step, not only a count; a test asserts it.

## Context

The diagnostic block also appears in status and transition output, so fix it at the shared formatter. The `Lint found N issue(s)` header the audit reported is now `Check found N findings:`. atelier-sfd9 adds the file and field to a missing-reference finding; share the format.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-sfd9 (Give a repair route for links to missing records that arrive by merge or hand edit)
