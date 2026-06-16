---
created_at: "2026-06-16T15:47:12.509982886+00:00"
id: "atelier-0nv2"
issue_type: "validation"
labels:
- "branch"
- "git"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T17:24:57.519992208+00:00"
status: "done"
title: "Validate automatic branch lifecycle and squash merge flow"
updated_at: "2026-06-16T17:24:57.519992208+00:00"
---

## Description

Independently validate the integrated branch lifecycle after the contract and implementation slices land. Start from the product claims, not from the implementation summary.

## Outcome

- Validator proves `atelier start` prepares the correct branch for child issue, standalone issue, and epic work without manual branch setup.
- Validator proves child issue close under an epic records terminal tracker state on the epic branch and does not merge the epic branch to base.
- Validator proves standalone issue close and epic close commit terminal tracker state and squash-merge the owner branch into the configured base branch.
- Validator proves alternate configured merge strategy or base branch is honored.
- Validator proves failure paths do not leave closed tracker state without branch integration.
- Validator confirms docs/help/Agent Factory guidance no longer teach explicit branch setup commands as the normal workflow.

## Evidence

- First-class validation evidence records the scenario setup, commands, observed branches, log output, merge strategy, and line-by-line classification of each Outcome bullet.
- Command transcript includes `git branch --show-current`, `git log --oneline --graph` or equivalent, `atelier issue show`, `atelier status`, and `atelier issue transition --options` at the relevant points.
- Search transcript covers docs/help/Agent Factory guidance for obsolete branch-command workflow.
- Validation evidence record lists follow-up issue IDs for any failed, blocked, or deferred behavior instead of fixing implementation inline.
- `atelier lint`, `atelier export --check`, `cargo fmt -- --check`, and relevant focused cargo tests pass or are explicitly classified with follow-up owners.

## Notes

This validation item should be performed by an agent that did not implement the core branch lifecycle changes.
