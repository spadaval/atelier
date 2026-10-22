---
created_at: "2026-10-17T01:10:57.979015085+00:00"
id: "atelier-0vp3"
issue_type: "bug"
labels:
- "cli-output"
- "setup"
priority: "P1"
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
title: "Detect the base branch at init, and give land_base_mismatch a fix"
updated_at: "2026-10-17T01:10:57.979015085+00:00"
---

## Goal

In a fresh `git init` repository on `master`, `atelier init` writes `base_branch: main` (the default policy text in crates/atelier-workflow/src/lib.rs:14). It does not detect the branch or mention the choice. After the install is committed:
- `atelier land` and `land --install` say `land_work_untraceable: base branch main not found, so this unowned branch's tracker changes cannot be checked; fetch main or run atelier start <id>`, which sends the user to fetch a branch that does not exist;
- `atelier start` records `base master`;
- the next land fails with `land_base_mismatch: recorded base master disagrees with trusted policy base main`, which gives no recovery at all;
- status lists the real base as `No started work on: master`.

## Done when

- `init` sets `base_branch` to the current branch, or to origin's HEAD branch when there is one, and prints `Base branch: master (from the current branch)`.
- `init --base <branch>` overrides the detected branch.
- When the configured base does not exist locally or remotely, land and `land_base_mismatch` name the fix: `set branch_policy.base_branch in .atelier/workflow.yaml`.
- Tests cover a repository on `master`, one on `main`, and a mismatch.

## Context

Init lives in crates/atelier-app/src/init.rs (no branch detection today) and crates/atelier-cli/src/commands/init.rs. The land messages are at crates/atelier-cli/src/commands/session.rs:258 and 465-470. `start` accepting a base that disagrees with the policy may need its own check.
