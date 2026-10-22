---
created_at: "2026-10-22T22:42:57.061302606+00:00"
id: "atelier-o2pm"
issue_type: "task"
labels:
- "merge"
priority: "P3"
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
status: "todo"
title: "Make merge --sync's resume and stale-approval verdict read plainly"
updated_at: "2026-10-22T22:48:09.806085776+00:00"
---

## Goal

Found using `atelier merge --sync` on its own branch (atelier-s2ei). After a hand-resolved conflict, the second run says "already contains master; nothing to merge" although it is resuming after a hand merge and rerunning the receipts that were current before it. And the verdict is `merge: passed` while the report says the approval needs fresh review, which reads as a contradiction although `merge --check` will then refuse. The rerun of a whole workspace suite also shows no progress.

## Done when

A resumed sync says it is resuming after a merge made by hand and names the merge commit. A sync that leaves the approval stale ends in a verdict that says so, for example `merge: synced — approval needs fresh review`, documented in docs/product/human-cli-output.md. Long reruns print the command before they start. Tests pin the three outputs.
