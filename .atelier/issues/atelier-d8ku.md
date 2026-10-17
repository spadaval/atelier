---
created_at: "2026-10-17T10:00:37.372029871+00:00"
id: "atelier-d8ku"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Give --commit and leftover parser errors the shared invalid-value form"
updated_at: "2026-10-17T10:00:37.372029871+00:00"
---

## Goal

Mission atelier-o3lm validation (evidence atelier-ba8q) found invalid values still outside the shared form in `docs/product/human-cli-output.md`:
- `review record <id> --identity r --commit zzz x` prints `review commit must be a full Git commit ID` and exits 1; a well-formed unknown commit prints a raw `git merge-base --is-ancestor … failed: fatal: …` with no recovery step.
- `prune --retention-days abc` and `issue list --limit -3` print clap's own wording (exit 2).

## Done when

- These use `Invalid <field> '<value>'. Valid values: …` and exit 2; an unknown commit names the commit and one recovery step.
- Tests cover each.
