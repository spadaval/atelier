---
created_at: "2026-10-17T01:10:57.992354805+00:00"
id: "atelier-od8b"
issue_type: "task"
labels:
- "cli-output"
- "help"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-od8b
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
status: "in_progress"
title: "Rebuild root help: generated list, lifecycle-ordered common commands"
updated_at: "2026-10-17T07:46:49.793804938+00:00"
---

## Goal

Root help is hand-written `after_help` text (`crates/atelier-cli/src/main.rs:15-61`) that has drifted from the clap `about` strings. Examples:
- init: "current repository" vs "current directory"
- land: "Check" vs "Verify"
- evidence: "Capture validation evidence" vs "First-class evidence records"
- work: "including the plural Mission Overview" vs "Operational multi-issue work views"

`doctor` and `migrate` are real, unhidden commands but never appear, because the template omits `{subcommands}`. `migrate` is also missing from `docs/product/cli-surface.md`, which lists `doctor` both as a surface (line 20) and as removed (line 98). The `migrate epics` about line is about 300 characters.

"Common commands" has 22 lines in no lifecycle order. It lists `issue show <id>` and also `issue show <mission-id>`, puts `issue list` between `work missions` and `work mission`, and includes `prune --apply` and `check <issue-id>`. It leaves out `work human`, `land`, `evidence record`, `review approve`, and `merge <id> --check`. `Orientation` contains `land`. `Planning` contains only `bundle`. There are three blank lines after `Usage:`.

Bare `atelier work` prints a third hand-written index (work.rs:73-82). It leaves out `active` and `all` and uses jargon ("Small top-level executable work picker", "Live mission orchestration dashboard"). ADR 0022 Decision 2 keeps static guidance out of output.

## Done when

- Command descriptions come from one source (clap `about`), and every non-hidden command appears in root help.
- Common commands run in lifecycle order with no duplicates: status, work ready, work mission <id>, issue show <id>, start <id>, evidence record ..., issue transition <id> close --reason ..., review approve ..., land, merge <id> --check.
- The `migrate epics` about line is at most 80 characters, with the detail moved to long help.
- Bare `atelier work` prints the generated subcommand list, including active and all, or runs `work all`.
- `cli-surface.md` lists migrate and drops the contradictory doctor line.
- A test asserts every subcommand name appears in `atelier --help`.

## Context

atelier-sdb0 replaces `prime` with `status` in this help. Coordinate with it, or land after it.
