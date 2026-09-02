---
created_at: "2026-09-02T19:05:56.297585459+00:00"
id: "atelier-to59"
issue_type: "bug"
labels:
- "bug"
- "git"
- "workflow"
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-to59
    owner_kind: issue
    review_target: master
    work_branch: bug/atelier-to59
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Namespace the local Git integration action"
updated_at: "2026-09-02T19:06:12.013919036+00:00"
---

## Description

Rename the public built-in workflow action `branch_integrate` to `git.integrate` across policy parsing, planning, execution, output, tests, and current target-state documentation. Update this repository’s task close transition to run `tracker.commit` followed by `git.integrate`. Preserve historical activity records as authored data, but do not accept the obsolete action name as a compatibility alias.

## Outcome

Workflow configuration consistently uses the namespaced `git.integrate` action. The starter policy and this repository’s task close transition integrate locally owned branches after committing tracker state, obsolete `branch_integrate` configuration is rejected, and focused tests prove successful integration plus rejection of the old name.
