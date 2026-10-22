---
created_at: "2026-10-16T23:43:14.236220229+00:00"
id: "atelier-jjvc"
issue_type: "task"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Remove the deprecated prime alias"
updated_at: "2026-10-17T03:34:53.055620141+00:00"
---

## Goal

ADR 0022 keeps `prime` as an alias for `status` so adopters' installed SessionStart hooks keep working. Remove it once known adopters' hooks no longer call it.

## Done when

`atelier prime` is gone from the CLI and help, `atelier check` or `land` names a `.claude/settings.json` hook that still calls `prime --hook claude` with the fix (`atelier setup claude`), and the CHANGELOG records the removal. Plan this into `todo` when neither this repository's nor astromech's committed `.claude/settings.json` contains `prime --hook claude`; the project makes no numbered releases.

## Context

ADR 0022 Decision 1; review finding atelier-1ia7 on atelier-bw0b.

## Notes

- 2026-10-17 [note] 2026-10-16: atelier-sdb0 merged; this repository's .claude/settings.json now runs 'atelier status --hook claude'. astromech's committed hook (checked in /home/supadava/projects/astromech-atelier at dd28efe) still runs 'atelier prime --hook claude', so the plan condition is unmet; stays in backlog until astromech reruns 'atelier setup claude'.
