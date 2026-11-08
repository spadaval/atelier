---
created_at: "2026-11-08T05:35:32.055259750+00:00"
id: "atelier-4k28"
issue_type: "task"
labels:
- "tooling"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Smoke-test a new atelier binary against every local Atelier repository before installing it"
updated_at: "2026-11-08T05:35:32.055259750+00:00"
---

## Goal

Review of atelier changes caught cross-repository regressions by hand (atelier-2ygf, atelier-2j35, atelier-8ljp): the new binary refused astromech's per-child reviews and other repositories' records. A script should catch these before review and before install.

## Done when

- [ ] A script in this repository builds the current tree's binary, makes scratch clones of each local Atelier repository (discovered from a list in the script or a directory scan, including atelier, astromech, life-on-rails, cosplay-studio, demiurge and worldcrafter), and runs read-only commands with both the new and the installed binary: at least `check`, `doctor`, `status`, `work missions` and `review request` for an open item where one exists.
- [ ] It prints a per-repository summary and exits non-zero when the new binary errors or refuses where the installed one does not. It never writes to the real repositories.
- [ ] Run once on the current base: the Report shows its output.
- [ ] The install step in this repository's docs (or AGENTS.md) says to run it before installing a new binary.

## Scope

A script and the doc line. No change to atelier's commands.

## Context

From the 2026-11-08 review audit.
