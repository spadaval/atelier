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
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show a receipt's full summary and its kind, producer and commit in debrief cards"
updated_at: "2026-11-05T04:40:10.192246210+00:00"
---

## Goal

Receipt summaries are cut to their first sentence, so atelier-tum4, edle and owp3 read as 'S1.' / 'S2.' / 'S3.' in cards, Results chips and Validation rows; cards say 'Receipt' rather than the kind and give no producer, commit or URI.

## Done when

The debrief of atelier-vt8q no longer shows the problem, and a fixture test covers it.

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
