---
created_at: "2026-10-15T22:08:58.923650405+00:00"
id: "atelier-5qcs"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-5qcs
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T23:15:00.021482559+00:00"
status: "done"
title: "Don't take the project slug from the directory name"
updated_at: "2026-10-15T23:15:00.021482559+00:00"
---

## Goal

`atelier init` defaults `project_slug` to the directory name. In a worktree such as `astromech-atelier` that mints `astromech-atelier-xxxx` IDs, which then can't be changed cheaply.

## Done when

`init` derives the slug from the origin remote's repository name (falling back to the main worktree's directory), prints the slug it chose, and accepts `--slug`. A test covers a linked worktree.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). The slug was fixed by hand in .atelier/config.toml.

## Close Reason

init derives project_slug from the origin remote's repository name, then the main worktree's directory, then the directory name; prints the chosen slug and its source; --slug overrides. Covered by test_init_slug_comes_from_origin_then_the_main_worktree (linked worktree, remote, --slug).
