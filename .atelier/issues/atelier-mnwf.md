---
created_at: "2026-06-16T15:46:54.347148694+00:00"
id: "atelier-mnwf"
issue_type: "feature"
labels:
- "branch"
- "close"
- "git"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0nv2"
  - kind: "issue"
    id: "atelier-8jaf"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make close commit tracker state and merge branch atomically"
updated_at: "2026-06-16T15:47:26.736815316+00:00"
---

## Description

Make terminal issue and epic close operations own the Git integration step. A close should not be a tracker-only state change when the work branch still needs to land in the configured base branch.

## Outcome

- Closing a child issue under an epic validates proof, writes terminal tracker state, records activity/evidence links, and creates a commit on the parent epic branch without merging that branch to base.
- Closing a standalone issue validates proof, writes terminal tracker state, commits that tracker state on the issue branch, switches to the configured base branch, and merges the issue branch with the configured merge strategy.
- Closing an epic validates child work and review/proof gates, writes terminal tracker state, commits that tracker state on the epic branch, switches to the configured base branch, and merges the epic branch with the configured merge strategy.
- Squash merge is the default integration strategy and the merge commit/message is deterministic enough for tests and audit.
- If tracker write, commit, checkout, or merge fails, the command exits non-zero and does not leave the target record closed without the corresponding branch integration.
- Close output reports the target, branch owner, source branch, base branch, merge strategy, created commit or merge result, and recovery guidance on failure.

## Evidence

- Failing-before/passing-after tests or transcripts cover child issue close on an epic branch without base merge.
- Tests or transcripts cover standalone issue close with squash merge to base.
- Tests or transcripts cover epic close with squash merge to base after child proof is complete.
- Failure-path tests cover merge conflict or commit failure and prove the target remains open or is rolled back with explicit recovery guidance.
- `git log --oneline` or equivalent transcript proves the default integration is squash merge.
- Focused tests, `cargo fmt -- --check`, `atelier lint atelier-mnwf`, and `atelier export --check` pass.

## Notes

This issue owns the atomic operator contract. If full transactional rollback is not possible for every Git failure mode, the implementation must define the exact intermediate states that can occur and print the recovery command that preserves tracker truth.
