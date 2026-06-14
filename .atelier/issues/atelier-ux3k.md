---
created_at: "2026-06-14T16:31:25.550179352+00:00"
id: "atelier-ux3k"
issue_type: "feature"
labels:
- "assignee:root"
- "tracker"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-tqjn"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T17:23:28.628579745+00:00"
status: "done"
title: "Scope clean-worktree closeout checks around tracker-generated state"
updated_at: "2026-06-14T17:23:28.628579745+00:00"
---

## Description

Clean-worktree validators should block on uncommitted source, docs, tests, workflow config, lockfiles, and hand-authored canonical tracker edits that can invalidate proof. They should not self-block solely because the current tracker transition produced deterministic activity or closeout bookkeeping.

## Outcome

- The clean-worktree closeout validator ignores ignored tracker runtime/cache
  paths and deterministic tracker-generated activity from the current
  transition or closeout operation.
- The validator still blocks on dirty source, product docs, tests, workflow
  config, lockfiles, and hand-authored canonical issue, mission, evidence, or
  workflow records that can invalidate proof or handoff.
- Status output reports uncommitted tracker canonical state clearly enough for
  handoff, even when a narrow tracker-generated exemption lets closeout
  proceed.

## Evidence

- Focused tests or transcripts cover tracker-generated activity from the
  current transition not self-blocking the clean-worktree validator.
- Focused tests or transcripts cover hand-authored dirty `.atelier/issues/*.md`,
  `.atelier/missions/*.md`, or `.atelier/workflow.yaml` changes still blocking
  unless they are explicitly classified by the implemented policy.
- Focused tests or transcripts cover dirty non-tracker files still blocking the
  validator.
- `git diff --check` and `atelier lint` pass.
