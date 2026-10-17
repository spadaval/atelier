---
created_at: "2026-10-17T07:17:41.053385208+00:00"
id: "atelier-zqee"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-zqee
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T07:19:55.496382336+00:00"
status: "done"
title: "Describe the merge_commit strategy in the Agent Factory skill"
updated_at: "2026-10-17T07:19:55.496382336+00:00"
---

## Goal

atelier-ydv0 added `branch_policy.merge_strategy: merge_commit`. The vendored Agent Factory skill still says Atelier never creates commits and that `atelier merge` only fast-forwards, and the orchestrate landing guidance (atelier-w56o) tells orchestrators to merge the base into every branch before it lands, even when merge would check the merged tree itself.

## Done when

- Upstream spadaval/.agents and the vendored `.agents/skills/agent-factory` say merge creates a merge commit under `merge_commit` after checking the merged tree, and that a clean branch then needs no catch-up merge; catch-up stays needed for conflicts, regenerated files, and base-side records about the item.
- The vendored copy equals upstream verbatim (ADR 0020).

## Close Reason

Upstream spadaval/.agents (451c0c9) and the vendored skill now say: atelier merge creates no commits except, under merge_commit, a merge commit made after checking the merged tree (references/atelier.md); under merge_commit a branch that merges cleanly can skip the catch-up merge provided the merge checks run the proof step 3 would rerun, and still catches up for conflicts, merge-check failures on the merged tree (such as a stale lockfile), and base-side records about the item (orchestrate.md). Vendored copy equals upstream (diff -rq clean).
