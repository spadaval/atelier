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
status: "backlog"
title: "List a child's reviews recorded against its branch owner"
updated_at: "2026-10-17T07:12:12.697349842+00:00"
---

## Goal

Reviews are recorded against the branch owner, so `review list <child-id>` prints nothing for a child whose parent owns the branch, with no hint that the reviews live on the parent. Found while implementing atelier-qrwr.

## Done when

- `review list <child-id>` shows the owner's reviews, or says they are recorded against `<owner-id>` and gives the command.
- A CLI test covers a child of a reviewed parent.
