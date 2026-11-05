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
status: "in_progress"
title: "Settle Close Reason and report-part follow-ups"
updated_at: "2026-11-05T01:15:51.813095344+00:00"
---

## Goal

Clear the small Close Reason and criteria follow-ups: a reused Close Reason on reclose (atelier-ypks), the report-part exemption rule and its docs (atelier-4vda), and the dead read-time criteria helper (atelier-pth2). One branch and one review. Starts after the decision-records branch merges, since both edit docs/product/work-model.md.

## Done when

Every child is terminal with its own Done when met and proven, and one independent review of the branch is approved here.
