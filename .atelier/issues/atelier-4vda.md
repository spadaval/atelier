---
created_at: "2026-10-17T18:27:30.799634465+00:00"
id: "atelier-4vda"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "review"
    id: "atelier-0t22"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-u0i3"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-uxrs"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Tighten and document the report-part exemption for earlier claims"
updated_at: "2026-10-17T18:27:30.799634465+00:00"
---

## Goal

Tighten and document the exemption that keeps done claims made before a policy required report parts valid (review of atelier-sfsq, findings atelier-u0i3, atelier-uxrs, atelier-0t22):

- atelier-u0i3: a branch can still skip parts by building a history whose merge-base with the base predates the policy (a side branch from before the base merge). No history-based rule tells it apart from a child honestly closed before the policy, so document it as a limit in docs/product/work-model.md and reviews-and-merge.md, or drop the exemption on work branches if the human prefers.
- atelier-uxrs: on a base branch with no upstream, land treats a claim one commit before HEAD as historical and skips its parts; check them against the policy at the parent that holds the claim.
- atelier-0t22: work-model.md still says parts are checked on claims committed beside a policy that required them; it is now the base's policy at the merge-base.

## Done when

work-model.md states the rule and its limit as implemented, and a session test shows the no-upstream base case checks report parts.
