---
created_at: "2026-11-05T04:40:11.096997865+00:00"
id: "atelier-les9"
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
title: "Make debrief chips and cards fully accessible beyond contrast"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

Cards cannot be dismissed without moving focus (WCAG 1.4.13), are a fixed 340px and may overflow; the Results criterion text is only in a title tooltip; the h1 sits in aside and main starts at h2 with no headings on timeline items; focusable chips sit inside summary (nested interactive content).

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown debrief and the site's debrief page. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
