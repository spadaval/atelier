---
created_at: "2026-10-16T22:39:25.385564040+00:00"
id: "atelier-37uq"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check only changed claims in land, and add atelier doctor for the full check"
updated_at: "2026-10-16T22:39:25.385564040+00:00"
---

## Goal

`land` re-verifies every done item already merged, on every run, which costs about 4.5 of its 5 seconds here and grows with history. If every change reaching the base has its own changed claims and review files checked, the whole history holds by induction, so `land` checks only what changed against a trusted point. A new `atelier doctor` runs the full verification on demand (and in CI), for history that reached the base without the gates (direct pushes, host-side merges, force pushes).

## Done when

On a work branch, land evaluates only claims whose records differ from where the branch left the base and checks review append-only only over that range; on the base, only what changed since the upstream (all of it when there is no upstream); `atelier doctor` runs today's full verification of every retained claim and the whole review history and exits nonzero on a failure. Tests cover that land skips an unchanged old claim whose proof no longer verifies while doctor reports it, and that an edited old claim is still checked by land. land is measured before and after; docs updated (sessions-and-hooks, cli-surface, ADR note).

## Context

Decided by Sumeet on 2026-10-16. Supersedes most of atelier-7b6e.
