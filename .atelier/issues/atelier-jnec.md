---
created_at: "2026-10-17T19:45:46.135278410+00:00"
id: "atelier-jnec"
issue_type: "bug"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "review"
    id: "atelier-2tbo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:25:05.165529848+00:00"
status: "done"
title: "Keep the debrief's history when HEAD is a work branch that merged the base"
updated_at: "2026-11-05T00:20:14.483473681+00:00"
---

## Goal

`issue_histories` in crates/atelier-app/src/record_history.rs walks HEAD's first-parent line, so on a work branch that merged master in, every master commit since the branch point collapses into that merge: the debrief moves the mission's start, and loses scope added after the start and reopened items, with no warning. Found by review atelier-2tbo on atelier-rx22.

## Done when

A debrief run from a work branch that merged the base reports the same start and deviations as one run from the base (walk the base's first-parent line, then the branch's own), shown by a CLI fixture.

## Notes

- 2026-11-04 [decision] Take the base from the workflow policy's base_branch and walk its first-parent line only up to the merge base with HEAD, then HEAD's own line (fork..HEAD); with no such branch, or on the base, walk HEAD alone as before
  Reason: The debrief describes HEAD, so base commits HEAD lacks stay out; the policy's base is what every debrief shares, and a missing ref keeps fixtures and detached checkouts working
- 2026-11-04 [decision] Keep the branch's own commits from before its merge of the base in the walk, after the base line
  Reason: They hold the branch's own record changes, which the merge's first-parent diff does not repeat; dropping them would lose those changes, and a stale state only misleads when the base and the branch both changed one record before the merge
- 2026-11-04 [decision] Start the branch's line at its first first-parent commit that holds the fork, counted as its change from the fork; branch commits before it are not walked on their own
  Reason: Their record states predate the base's newer versions, so walking them after the base line read as false reopens (finding atelier-uev7); their changes still arrive with the commit that brings in the fork. This replaces the note that kept them in the walk

## Close Reason

`issue_histories` first walks the base branch's first-parent line (from the workflow policy) up to the fork, the merge base with HEAD. It then walks HEAD's own first-parent line, starting at the first commit that holds the fork. That commit counts as its change from the fork, and later commits count as their change from their first parent. Positions continue across both lines. On the base, or when the base branch is missing, it walks HEAD alone as before.

The CLI fixture `a_debrief_from_a_branch_that_merged_the_base_keeps_the_base_history` forks a branch before the mission starts. The branch touches a scope record before it merges the base. On main, the mission starts, scope is added, one item ships and another is reopened. Then the branch merges main. The branch's debrief now shows the same start line, deviations and quiet counts as main's.

The fixture failed for both versions of the bug. Under the original walk, the added scope and the reopened item were missing. Under the first fix, the branch's pre-merge version of the shipped item showed a false reopen (finding atelier-uev7).

### Deviations
None.

### Decisions
Decision notes on this item:
- The base comes from `branch_policy.base_branch` and is walked only up to the merge base.
- The branch's line starts at its first commit that holds the fork, counted from the fork. This replaces the earlier note that kept pre-merge branch commits in the walk.

### Surprises
Review finding atelier-uev7 showed that walking the branch's pre-merge commits after the base line puts their older record states after newer base states.

### Open items
None.
