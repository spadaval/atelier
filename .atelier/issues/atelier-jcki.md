---
created_at: "2026-10-17T01:10:57.965792795+00:00"
id: "atelier-jcki"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show the branch, review state and mission in issue show"
updated_at: "2026-11-07T21:03:28.142255576+00:00"
---

## Goal

`atelier issue show` does not say whether an item is started, how its review stands or which mission it serves:
- No owner branch: a started item's `workflow_branch` is not shown.
- No review state: after several review events on one item, `issue show` mentions none of them.
- No inherited mission: a child whose parent advances a mission shows no mission, although the inventory doc says "A linked parent includes its children in mission execution context".

Agents run `review list`, `git branch` or the parent's show to fill the gaps.

## Done when

- [ ] `issue show` prints, from any branch: `Branch: atelier-28mz`, or `Branch: owned by atelier-xxxx (atelier-xxxx)` for a child; `Mission: atelier-1w8c "…"`, directly or `via parent atelier-xxxx`; and `Review: ✓ approved atelier-hleo @c24af21`, `⚠ stale since <commit>`, `⚠ 1 open finding` or `none`.
- [ ] Review state is judged as `merge --check` judges it: an approval is stale only when code changed since its commit (atelier-3jdm), and a finding is open when it is a `fix_now` finding with no approval recorded after it, or a `defer` or `no_action` finding with no decision (atelier-md0v).
- [ ] CLI tests cover an owner with reviews, a child with an inherited mission, and an item with neither.

## Context

Dropped in decision atelier-mzpn: the `(base master)` suffix, since the base is the policy's (atelier-md0v), and a Proof line, since receipts gate nothing (atelier-md0v). Builds on the brief `issue show` (atelier-7u4o, done). Review freshness and findings logic exists for `merge --check` (`atelier_app::review_gate`).
