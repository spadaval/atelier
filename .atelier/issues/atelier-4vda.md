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
closed_at: "2026-11-05T01:23:48.449838626+00:00"
status: "done"
title: "Tighten and document the report-part exemption for earlier claims"
updated_at: "2026-11-05T01:23:48.449838626+00:00"
---

## Goal

Tighten and document the exemption that keeps done claims made before a policy required report parts valid (review of atelier-sfsq, findings atelier-u0i3, atelier-uxrs, atelier-0t22):

- atelier-u0i3: a branch can still skip parts by building a history whose merge-base with the base predates the policy (a side branch from before the base merge). No history-based rule tells it apart from a child honestly closed before the policy, so document it as a limit in docs/product/work-model.md and reviews-and-merge.md, or drop the exemption on work branches if the human prefers.
- atelier-uxrs: on a base branch with no upstream, land treats a claim one commit before HEAD as historical and skips its parts; check them against the policy at the parent that holds the claim.
- atelier-0t22: work-model.md still says parts are checked on claims committed beside a policy that required them; it is now the base's policy at the merge-base.

## Done when

work-model.md states the rule and its limit as implemented, and a session test shows the no-upstream base case checks report parts.

## Notes

- 2026-11-04 [decision] A held claim's report parts are judged by the base policy at the commit that made the claim, the same rule as current claims, instead of by the policy at the parent that holds it
  Reason: The parent's policy would refuse every pre-policy claim the base holds once the base requires parts, breaking the exemption; with the parent as base_commit the merge-base is the introducing commit, so one rule covers both
- 2026-11-04 [decision] Document the side-branch merge-base hole as a limit; keep the exemption on work branches
  Reason: Directed by the human via the orchestrator; no history-based rule tells such a claim from a child honestly closed before the policy

## Close Reason

`land` and `merge` now judge a held done claim's report parts by the base's policy at the commit that made the claim, the same rule as current claims, instead of skipping them, so a claim one commit back on a base with no upstream is checked (new session test). docs/product/work-model.md states the merge-base rule and its side-branch limit, and reviews-and-merge.md documents the limit for reviewers.

### Deviations
None. Per the human's direction the side-branch hole (atelier-u0i3) is documented as a limit; the exemption on work branches stays.

### Decisions
Held claims use the policy at the claim's introducing commit, not the policy at the parent that holds it, which would refuse every pre-policy claim the base holds (decision note). The policy-at-claim lookup is computed once per claim rather than once per missing part.

### Surprises
None.

### Open items
None.
