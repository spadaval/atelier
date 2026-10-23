---
created_at: "2026-10-17T07:12:12.697349842+00:00"
id: "atelier-jw1n"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T14:35:50.359336106+00:00"
status: "done"
title: "List a child's reviews recorded against its branch owner"
updated_at: "2026-10-23T14:35:50.359336106+00:00"
---

## Goal

Reviews are recorded against the branch owner, so `review list <child-id>` prints nothing for a child whose parent owns the branch, with no hint that the reviews live on the parent. Found while implementing atelier-qrwr.

## Done when

- `review list <child-id>` shows the owner's reviews, or says they are recorded against `<owner-id>` and gives the command.
- A CLI test covers a child of a reviewed parent.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. review list <child> prints nothing, exit 0. review_request.rs gather maps child->parent; status uses review_gate::scope.
- 2026-10-23 [decision] review list <child> shows the owner's full view after the line '<child> is reviewed with its branch owner <owner>.'; a child that has its own branch ownership is its own owner.
  Reason: Reviews are recorded against the branch owner; showing them saves a second command.
  Rejected: Only pointing to atelier review list <owner>

## Close Reason

`atelier review list <child>` prints `<child> is reviewed with its branch owner <owner>.` and then the owner's full view (rows, approval state, findings); a child with its own branch ownership is its own owner.

### Deviations
None.

### Decisions
See the decision note on this item (show the owner's view rather than only pointing to it).

### Surprises
None.

### Open items
None.
