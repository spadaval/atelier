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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Remove the deprecated prime alias"
updated_at: "2026-10-17T03:34:53.055620141+00:00"
---

## Goal

The new binary has no `prime`, `land` or `status --hook`, but a clone whose `.claude/settings.json` still runs `atelier status --hook claude` and `atelier land --hook claude`, as this repository's does today, gets no word of it from `atelier check` (checked 2026-11-11 with the new binary in a converted scratch clone). Each session there would then start and stop on errors until someone reruns `atelier setup claude`.

## Done when

- [ ] `atelier check` names a `.claude/settings.json` hook that calls a command or flag the binary does not have, with the fix (`atelier setup claude`).
- [ ] A test covers a settings file with an old `land --hook claude` hook.

## Context

ADR 0022 Decision 1; review finding atelier-1ia7 on atelier-bw0b.

Rewritten for the new tool in the cutover sort (atelier-5zre): the new binary already has no `prime`; what remains is naming old hooks, which now include `status --hook` and `land --hook`. The CHANGELOG line and the plan condition went with the alias.

## Notes

- 2026-10-17 [note] 2026-10-16: atelier-sdb0 merged; this repository's .claude/settings.json now runs 'atelier status --hook claude'. astromech's committed hook (checked in /home/supadava/projects/astromech-atelier at dd28efe) still runs 'atelier prime --hook claude', so the plan condition is unmet; stays in backlog until astromech reruns 'atelier setup claude'.
