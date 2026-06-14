---
created_at: "2026-06-14T16:31:25.550179352+00:00"
id: "atelier-ux3k"
issue_type: "feature"
labels:
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
status: "todo"
title: "Ignore canonical tracker files in clean-worktree closeout checks"
updated_at: "2026-06-14T16:31:25.550179352+00:00"
---

## Description

Clean-worktree validators should block on uncommitted source, docs, tests, and config changes that can invalidate proof, but not on canonical tracker records produced by tracker workflow commands.

## Outcome

- The clean-worktree closeout validator ignores canonical tracker records under
  `.atelier/` and ignored tracker runtime/cache paths.
- The same validator still blocks on dirty source, product docs, tests,
  workflow config, lockfiles, or other non-tracker files that can invalidate
  proof.
- Status output can report uncommitted tracker state as handoff guidance, but
  tracker canonical updates alone do not block workflow close transitions.

## Evidence

- Focused tests or transcripts cover dirty `.atelier/issues/*.md` and
  `.atelier/*activity*` records being ignored by the clean-worktree validator.
- Focused tests or transcripts cover dirty non-tracker files still blocking the
  validator.
- `git diff --check` and `atelier lint` pass.
