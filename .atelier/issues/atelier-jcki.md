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

`atelier show <id>` prints an item's status, mission, blockers and each review's round and verdict, but not whether its approval still covers the branch: after an approving review and a later commit that changes `README.md`, `show` prints `Status: approved` and the old verdict with no stale mark, while `status` on the branch says the approval no longer covers `README.md` (review finding atelier-1d9u, reproduced on the new binary in a scratch clone). Agents then run `status` on the branch to learn what `show` should say.

## Done when

- [ ] `show <id>` marks an approval that no longer covers the head as stale, naming the first path outside the item's directory that changed since it, judged as `merge` judges it.
- [ ] CLI tests cover an approval that still covers the head and one made stale by a code change.

## Context

Dropped in decision atelier-mzpn: the `(base master)` suffix, since the base is the policy's (atelier-md0v), and a Proof line, since receipts gate nothing (atelier-md0v). Builds on the brief `issue show` (atelier-7u4o, done). Review freshness and findings logic exists for `merge --check` (`atelier_app::review_gate`).

Rewritten for the new tool in the cutover sort (atelier-5zre): `issue show` became `show`, which already prints the mission and each review; the branch is named for the item, and parents are gone, so only the approval's freshness remains.
