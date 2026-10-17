---
created_at: "2026-10-17T01:10:57.971709219+00:00"
id: "atelier-kset"
issue_type: "bug"
labels:
- "check"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Print each check finding once, on one line, against the owning record"
updated_at: "2026-10-17T01:10:57.971709219+00:00"
---

## Goal

`atelier check` findings are noisy and mislabelled (reproduced in a scratch clone):

- A bad status in atelier-uyuu is printed twice, once under `(canonical)` and once under `atelier-uyuu`.
- Missing references are labelled `(canonical)` instead of the linking record.
- Workflow diagnostics render as a five-line block, `error: workflow_issue_status_invalid` / `path: /home/.../.atelier/workflow.yaml` / `reference:` / `message:`. The path is absolute and points at workflow.yaml even when the fault is in an issue file.
- Every finding is prefixed `Tracker record files are invalid:`.
- The missing-Goal message repeats the ID twice and the path three times: `Invalid record file .atelier/issues/atelier-wtnp.md: Missing required issue body section 'Goal' for issue atelier-wtnp, section Goal, path .atelier/issues/atelier-wtnp.md`.
- YAML line numbers count from the start of the front matter, so they are one less than the file line (17 vs 18).
- An invalid workflow policy is printed twice, once as the `Invalid working policy` load warning and once as the finding.
- The header `Lint found 1 issue(s)` uses "issue" for a problem and names a `lint` command that does not exist.

## Done when

- Each fault prints once on one line: `✗ atelier-uyuu  .atelier/issues/atelier-uyuu.md:18  status 'frobnicated' is not valid for validation; use backlog, todo, …`.
- The header reads `check: 2 problems in 2 records`.
- Tests cover a duplicate-prone status fault, a missing reference attributed to its linking record, and a file-relative YAML line number.

## Context

crates/atelier-cli/src/commands/issue.rs:1026-1043; the diagnostic block format also appears in status and transition output, so fix it at the shared formatter.
