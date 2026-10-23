---
created_at: "2026-10-23T14:41:04.594628762+00:00"
id: "atelier-e8l6"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-mrq7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Do not let an inline backtick line open an evidence fence"
updated_at: "2026-10-23T14:41:04.594628762+00:00"
---

## Goal

Since atelier-dg3h, evidence body parsing tracks code fences across the whole body. A hand-written receipt with an unbalanced or inline triple-backtick line (for example ```` ```foo``` x ````) opens a fence that never closes and hides the later `## Stdout` and `## Stderr` headings. No committed receipt is affected; written receipts use balanced, length-safe fences.

## Done when

A records test shows a body with an inline triple-backtick line still yields its later stream sections; `opening_fence_len` rejects an opening line whose info string contains a backtick, as CommonMark does.

## Context

Review finding atelier-mrq7 on atelier-dg3h; `markdown_sections` and `opening_fence_len` in crates/atelier-records/src/lib.rs.
