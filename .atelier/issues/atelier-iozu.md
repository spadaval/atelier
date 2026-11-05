---
created_at: "2026-11-04T23:12:48.603623032+00:00"
id: "atelier-iozu"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-iozu
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-4vda"
  - kind: "issue"
    id: "atelier-pth2"
  - kind: "issue"
    id: "atelier-ypks"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2bjz"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T01:40:35.218464317+00:00"
status: "done"
title: "Settle Close Reason and report-part follow-ups"
updated_at: "2026-11-05T01:40:35.218464317+00:00"
---

## Goal

Clear the small Close Reason and criteria follow-ups: a reused Close Reason on reclose (atelier-ypks), the report-part exemption rule and its docs (atelier-4vda), and the dead read-time criteria helper (atelier-pth2). One branch and one review. Starts after the decision-records branch merges, since both edit docs/product/work-model.md.

## Done when

Every child is terminal with its own Done when met and proven, and one independent review of the branch is approved here.

## Close Reason

Close Reason follow-ups are settled: a close that reuses a record's Close Reason says so and supersede no longer keeps a close's report parts (atelier-ypks), report parts on claims the base already holds are checked against the base's policy where the claim was made, with the side-branch limit documented (atelier-4vda), and the unused read-time criteria helper and its stale comments are gone (atelier-pth2).

### Deviations
ypks took the "say which Close Reason it used" option because policy refuses transitions out of done; 4vda judges held claims by the policy at the claiming commit rather than at its parent (decision notes on both).

### Decisions
On the children (`atelier work decisions atelier-r6bt`).

### Surprises
None beyond the children's.

### Open items
None.
