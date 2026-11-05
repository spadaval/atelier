---
created_at: "2026-10-17T17:51:42.072219114+00:00"
id: "atelier-s5x4"
issue_type: "bug"
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
    id: "atelier-mws5"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:20:17.311131050+00:00"
status: "done"
title: "Count done decision records as decided when the repository has no workflow policy"
updated_at: "2026-11-04T23:20:17.311131050+00:00"
---

## Goal

In a repository without .atelier/workflow.yaml, `decided()` in crates/atelier-app/src/decisions.rs returns false for every decision record, because it reads the status category from the policy. Done records then show under Open Decision Records, `work decisions -q` leaves them out, and a decided newer record never supersedes an older one. Found by review atelier-mws5 on atelier-fip8.

## Done when

With no workflow policy, a decision record with status `done` counts as decided (falling back to the status name), shown by a CLI test that removes workflow.yaml.

## Close Reason

With no workflow policy, a decision record whose status is `done` now counts as decided: it lists under Decision Records, `work decisions -q` includes it, and it supersedes the records it links. A CLI test that removes workflow.yaml shows this.

### Deviations

The fix also covers the mission timeline's done test in `record_history.rs`, which had the same flaw; both now use one `status_is_done` helper in `atelier-app`'s `workflow_policy` module.

### Decisions

None beyond the helper's fallback rule, which the Done when names (the status name stands in for its category).

### Surprises

None.

### Open items

Other places still read the category only through the policy (for example `issue_inventory.rs` category filters and the CLI's `issue_is_done`); they were not part of this item.
