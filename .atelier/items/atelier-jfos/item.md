---
schema: atelier.item
schema_version: 3
id: atelier-jfos
title: Have the pre-push hook's way out move main in the checkout that has main out
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T01:41:53.372495908Z
---

## Goal

The pre-push hook refuses a code commit on main and says: move it to a
branch with `atelier start --new "<title>"`, "then `git branch -f main
<sha>` takes them off main". In the checkout that has main out, which is
where an agent asked to "just push it" is, Git refuses that command:
`fatal: cannot force update the branch 'main' used by worktree at <repo>`.
And `start --new` there leaves the checkout on main. So the one way out the
refusal names fails at its second step, and the agent has to work out
`git switch <id>` (or `git reset --keep <sha>`) alone. Found by atelier-rt3o
while bringing next-refused-push's fixtures to the branch model, on
f4d29ba76.

## Done when

- [ ] In a checkout with main out and an unmerged code commit on it, the
  steps the pre-push refusal names, run as printed, move the commit to an
  item's branch and leave main at the commit given; a test runs them.

## Scope
