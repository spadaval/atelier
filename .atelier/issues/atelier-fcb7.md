---
created_at: "2026-11-10T00:56:04.348189885+00:00"
id: "atelier-fcb7"
issue_type: "mission"
labels:
- "redesign"
strategy:
  path: docs/strategies/redesign-2-core.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Rewrite the work loop as 17 commands that tell each thread what it holds and what comes next"
updated_at: "2026-11-10T16:32:47.668024286+00:00"
---

## Outcome

A new workspace on master, `next/`, holds the tool rewritten in full: a new `atelier` binary that implements the product specification's records and commands. It is installed nowhere, and the old workspace beside it is untouched. A fresh agent told only to run `atelier status` takes a ready item to done with an independent reviewer. A converted scratch clone of each of the five repositories passes `check`, and one item completes in two of them.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-09 [note] Amended on Sumeet's direction (2026-11-09, in conversation), before any mission was made ready: Git hooks guard Git's actions and `land` goes; every rule can be skipped on purpose with ATELIER_SKIP and a reason; `merge` loses its flags and never waits, since the harness waits; `show` and `list` are spelled out so a lead or planner loses no view; his go-ahead stays `ready --by`. The command count is 17.
- 2026-11-09 [note] Amended on Sumeet's direction (2026-11-09, in conversation), before any mission was made ready: the tool is rewritten in full, in a fresh Cargo workspace `next/` that depends on nothing under `crates/`. Pieces of the old code are copied only on purpose, named by the item that copies them. Mission 3's switch deletes `crates/` whole and moves `next/` into its place. A view of what changed since his go-ahead is the dashboard's to derive; the tool gives it each act with its commit and records readable at any commit.
- 2026-11-10 [note] 2026-11-10: orchestration started, unattended, on Sumeet's go-ahead (atelier-a2uw). Step A runs in order: mal9, engh, 5cuc, syyj, then the fresh-agent runs in 67nn. Implementers are Claude Opus 5.5 and reviewers GPT-6.1 Sol. Each item has its own worktree under ~/.t3/worktrees/atelier/<id>.
- 2026-11-10 [decision] The Authority's ban on new dependencies is read as: the new workspace may use the crates the old workspace already depends on (the root Cargo.toml's workspace dependencies and the test crates in Cargo.lock); any other crate is escalated to the orchestrator
  Reason: The specifications name no crate at all, so a literal reading would forbid serde and clap. They do say the new workspace has the crates the old one has, and they name YAML front matter and config.toml, which the old dependency set already parses.
  Rejected: Treat every crate as needing Sumeet's approval
- 2026-11-10 [decision] Cut merge_strategy from config.toml: all five repositories set merge_commit and the field has one legal value. atelier-mal9 removes it from the config type and both specifications
  Reason: The Authority lets the orchestrator cut what proves unnecessary, and the product specification adds a named option only when a second real project needs one.
  Rejected: Keep a field that every reader ignores
- 2026-11-10 [note] 2026-11-10: atelier-mal9 escalated after three review rounds, each with one or two fix-now findings, all fixed. Three of the four came from the hand-written check for fenced code blocks. Answered: a fourth round on the fix as it stands, no refusal for an unclosed fence and no Markdown parser dependency.
- 2026-11-10 [note] 2026-11-10: atelier-mal9 (records, the next/ workspace and its CI job; pull request 96, four review rounds) and atelier-engh (identity, the act table, start, submit, status; pull request 97, two rounds) are merged. next/ holds 2,914 non-test lines. The act table holds only the rows of commands that exist; each later step adds its rows with its commands. atelier-5cuc is next.
