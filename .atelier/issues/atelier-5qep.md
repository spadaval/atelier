---
created_at: "2026-10-17T01:10:57.964262389+00:00"
id: "atelier-5qep"
issue_type: "feature"
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
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Accept several values and --open in issue list filters"
updated_at: "2026-10-17T01:10:57.964262389+00:00"
---

## Goal

`issue list` filters take a single value each. `--status todo --status backlog` and `-l a -l b` fail with a clap usage error, and no filter means "not done" (agents tried `-s open`, which errors). Error messages don't help either: `--status bogus` gives `Invalid issue status 'bogus'. Use an exact workflow status or `all`.` and `--category bogus` gives `Use a category from the workflow policy.` Neither lists the values (`crates/atelier-app/src/issue_inventory.rs:47-62`), although `--issue-type nope` does, and `--help` doesn't list them. Contradictory filters (`--category todo --status done`) print `No issues match the selected filters.` and don't name the filters.

## Done when

- `--status`, `--category`, `--label` and `--issue-type` accept repeats (or comma lists). Values within one flag are ORed; different flags are ANDed.
- `--open` (or `--category open`) selects every non-done category.
- Invalid status or category errors list the policy's valid values. `--help` names them, or says where they come from.
- The empty result repeats the active filters, e.g. `No issues with status=done and category=todo.`
- CLI tests cover repeats, `--open`, an invalid value and the empty message.
