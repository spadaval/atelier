---
created_at: "2026-10-16T23:04:58.434061193+00:00"
id: "atelier-fpxc"
issue_type: "feature"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Search issues by text"
updated_at: "2026-10-17T00:31:28.999883829+00:00"
---

## Goal

Agents find issues with `issue list --limit 100 | grep -i <word>` because there is no text search, and the grep misses anything outside the title.

## Done when

`issue list --search <text>` matches case-insensitively against title and body and prints the usual inventory rows; it combines with existing filters. Tests cover a title match, a body-only match, and combination with `--status`.

## Scope

`issue list` and the inventory query.

## Context

Transcript evidence: `atelier issue list --status todo --limit 100 | grep` (9 runs), `issue list | grep -i adoption`.

## Notes

- 2026-10-17 [human] 2026-10-17: deferred to backlog by Sumeet: search changes no view read before, during, or after a mission. Plan it when transcript counts still show issue list piped through grep after atelier-7u4o and atelier-ihns land. Not scoped to a mission.
