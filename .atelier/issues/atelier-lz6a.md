---
created_at: "2026-06-30T21:33:31.445683005+00:00"
id: "atelier-lz6a"
issue_type: "mission"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-c4s7"
    type: "advances"
  - kind: "issue"
    id: "atelier-otth"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "draft"
title: "Mission: Mission-isolated execution and publish"
updated_at: "2026-06-30T21:33:31.445683005+00:00"
---

## Description

Make mission execution independent from the repository base branch after mission
start. A mission worktree should be able to run scoped implementation,
validation, review, and closeout without checking out `master`; the only
interaction with `master` should be an explicit publish review/merge step.

This also tightens relationship semantics so `validates` is evidence-only, not
an issue-to-issue pseudo-field.

## Outcome

Atelier supports mission-isolated execution:

- Mission start creates or checks out `mission/<mission-id>` from the configured
  base branch, then subsequent mission lifecycle work can continue from the
  mission branch without checking out `master`.
- All mission-scoped branch-owning work derives its branch base, review target,
  and integration target from the mission branch unless a more specific owner
  branch applies.
- Mission publish opens or reuses a review artifact from `mission/<mission-id>`
  to the configured base branch and moves the mission to a review state instead
  of merging automatically.
- Mission close requires the publish review to be complete and can sync the
  configured base branch without switching away from the current checkout.
- Two missions can run simultaneously in different worktrees without colliding
  on checkout requirements.
- Issue-to-issue `validates` links are rejected; `validates` is reserved for
  evidence attachments.

## Evidence

- Focused tests cover mission lifecycle transitions without checking out
  `master` after start, concurrent mission worktrees, mission-scoped validation
  branch targets, publish review behavior, and `git.sync` without checkout.
- Relationship tests prove issue-to-issue `validates` is rejected through both
  CLI and canonical rebuild paths while evidence `validates` remains valid.
- Baseline checks pass: `cargo fmt -- --check`, `cargo nextest run`,
  `git diff --check`, and `atelier check`.
