---
created_at: "2026-11-05T04:40:10.192246210+00:00"
id: "atelier-idx3"
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
    id: "atelier-gg6b"
    type: "advances"
  - kind: "issue"
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T21:04:21.315390231+00:00"
status: "superseded"
title: "Show a receipt's full summary and its kind, producer and commit in debrief cards"
updated_at: "2026-11-07T21:04:21.315390231+00:00"
---

## Goal

Receipt summaries are cut to their first sentence, so atelier-tum4, edle and owp3 read as 'S1.' / 'S2.' / 'S3.' in cards, Results chips and Validation rows; cards say 'Receipt' rather than the kind and give no producer, commit or URI.

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown and HTML debrief. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).

## Report

Receipt detail on debrief cards does not earn its place once receipts gate nothing (atelier-md0v; decision atelier-mzpn).
