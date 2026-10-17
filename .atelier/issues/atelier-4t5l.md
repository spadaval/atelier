---
created_at: "2026-10-17T01:10:57.971093279+00:00"
id: "atelier-4t5l"
issue_type: "task"
labels:
- "check"
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make atelier check confirm what it checked"
updated_at: "2026-10-17T01:10:57.971093279+00:00"
---

## Goal

A passing `atelier check` prints only `Lint passed.`, the same for the whole repository and for `atelier check wtnp`. It names neither the record nor the count, and `-q` changes nothing. Agents piped `atelier check` through `tail` 166 of 323 times in the transcripts (`2>&1 | tail -5`, `| tail -1`). On failure, `tail -1` keeps only `Error: Lint failed with 1 finding(s)` and drops the finding.

## Done when

- A whole-repo pass prints `✓ check: 742 records valid` (with the actual count).
- A per-ID pass prints `✓ atelier-wtnp valid`.
- A failure's last line names the first problem and the next step, not only a count.
- Tests assert all three.

## Context

Pairs with the one-line findings item in this mission.
