---
created_at: "2026-08-05T18:46:58.763039135+00:00"
id: "atelier-f9hk"
issue_type: "task"
labels:
- "validation-finding"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-08-05T19:12:13.020584255+00:00"
status: "done"
title: "Retire open descendants of superseded outcomes"
updated_at: "2026-08-05T19:12:13.020584255+00:00"
---

## Description

Repair the independent validation finding that superseding an epic leaves its open speculative descendants executable. Preserve completed descendants and evidence while treating terminal ancestry as a hard boundary for ready selection and focused objective health.

## Outcome

- Open descendants of a terminal or superseded ancestor cannot enter global, mission, or epic ready projections.
- Completed descendants and their evidence remain inspectable after parent supersession.
- Fresh CLI regressions exercise mixed completed and open descendants beneath a superseded epic.

## Evidence

- Global, mission, and epic views retire open descendants while preserving completed history and evidence.
