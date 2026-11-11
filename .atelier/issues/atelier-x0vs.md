---
created_at: "2026-10-23T14:36:49.448272142+00:00"
id: "atelier-x0vs"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
  - kind: "issue"
    id: "atelier-phfo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:41.405331364+00:00"
status: "declined"
title: "Show each mission decision once in issue show --full"
updated_at: "2026-11-11T03:55:41.405331364+00:00"
---

## Goal

`atelier issue show <mission> --full` lists each decision note twice in its Decisions section, although the record stores it once. Readers cannot tell whether two decisions were taken.

## Done when

A test shows a mission with N decision notes lists N lines under Decisions in `issue show --full`.

## Context

Seen on atelier-rc8j on 2026-10-23: five `--kind decision` notes, ten Decisions lines. Likely the mission view merges its own notes with decisions gathered from its scope, and the mission is in its own scope.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes how `issue show --full` prints decision notes, and the What is removed table removes decision records (Commands: decision records and notes are gone).
