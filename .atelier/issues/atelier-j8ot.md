---
created_at: "2026-06-30T16:10:01.354496724+00:00"
id: "atelier-j8ot"
issue_type: "epic"
labels:
- "docs"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fs79"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-sszj"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Migrate workflow defaults and guidance to mission branch model"
updated_at: "2026-06-30T16:10:01.354496724+00:00"
---

## Description

Apply the new workflow contract to repository defaults, documentation, help text, and tests.

## Outcome

- `.atelier/workflow.yaml` uses simplified workflow names and the new `git.*` action and validator names.
- The checked-in workflow opts into mission integration branches where appropriate without forcing that behavior on every repository.
- Documentation and command help no longer describe `branch_templates`, `branch.prepare`, `branch.push`, `base.sync`, or `git.on_base_branch` as the target vocabulary.
- Existing branch and worktree cleanup guidance accounts for older `codex/<id>` and template-derived branches.

## Evidence

- Docs/help searches show no stale target-state references to removed branch-template or old action names.
- Representative CLI help and workflow validation tests pass.
- `target/debug/atelier check atelier-j8ot` passes.
