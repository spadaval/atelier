---
created_at: "2026-06-17T17:59:01.070772785+00:00"
id: "atelier-6375"
issue_type: "epic"
labels:
- "implementation"
- "sessions"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4clo"
  - kind: "issue"
    id: "atelier-98mo"
  - kind: "issue"
    id: "atelier-c5oz"
  - kind: "issue"
    id: "atelier-cglp"
  - kind: "issue"
    id: "atelier-e7oj"
  - kind: "issue"
    id: "atelier-mpah"
  - kind: "issue"
    id: "atelier-p7oa"
  - kind: "issue"
    id: "atelier-udny"
  - kind: "issue"
    id: "atelier-vg25"
  - kind: "issue"
    id: "atelier-yrwm"
  children:
  - kind: "issue"
    id: "atelier-95wv"
  - kind: "issue"
    id: "atelier-o97w"
  - kind: "issue"
    id: "atelier-vvs3"
  - kind: "issue"
    id: "atelier-y31v"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T00:21:02.116197031+00:00"
status: "done"
title: "Epic: Add durable optional sessions"
updated_at: "2026-06-18T00:21:02.116197031+00:00"
---

## Description

Add durable optional sessions that provide attribution, handoff history, and
bounded command summaries without becoming the current-work authority.

## Outcome

- Session records live in canonical Markdown and have explicit lifecycle
  commands.
- `atelier start` auto-creates sessions by default and fails on active
  mutating-session conflicts unless reuse is explicit.
- Status, man, and history surfaces can show active sessions while preserving
  issue workflow status as current work.

## Evidence

- Focused CLI tests cover session begin/show/list/end, start auto-create,
  conflict failure, `--no-session`, and explicit reuse.
- Command transcript shows targeted session tests plus `atelier lint` pass.
