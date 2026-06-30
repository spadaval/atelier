---
created_at: "2026-06-30T19:18:31.878755328+00:00"
id: "atelier-akxm"
issue_type: "epic"
labels:
- "git"
- "review"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-sszj"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Execute workflow actions in configured order"
updated_at: "2026-06-30T19:18:31.878755328+00:00"
---

## Description

Remove the hidden pre/post workflow action phases so transition actions execute exactly in the order configured in `.atelier/workflow.yaml`. This is follow-up work for mission `atelier-sszj`: mission and provider review workflows need to be able to validate a clean worktree, push the source branch, and then open the review artifact without requiring manual remote setup or a checkout of `master`.

## Outcome

- Workflow transition actions execute in configured order; action phase is not inferred from the action name.
- `git.worktree_clean`, `git.push`, and `review.open` can be configured as an intuitive `request_review` sequence.
- Canonical writes from actions such as `review.open` are preserved without relying on hidden phases.
- Failure behavior is explicit when an action fails before or after a status mutation.
- Tests prove `git.push` can run before provider `review.open` and that ordered execution is reflected in transition activity.

## Evidence

- `cargo test -p atelier-cli workflow --lib` covers ordered action execution and canonical state preservation.
- Provider-review CLI integration proof shows `request_review` pushes the source branch before opening the PR.
- `target/debug/atelier check <issue-id>` passes.
