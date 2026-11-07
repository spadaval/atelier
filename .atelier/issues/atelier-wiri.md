---
created_at: "2026-11-07T19:02:40.110194328+00:00"
id: "atelier-wiri"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Fix Atelier docs links to agent-factory files the skill cut removed"
updated_at: "2026-11-07T19:02:40.110194328+00:00"
---

## Goal

atelier-qj9d cut the agent-factory skill and removed `references/tracker-records.md` and the `plan.md#ready-missions` anchor. Four links in Atelier's docs now point at them: docs/product/mission-authoring.md:4 and docs/product/work-model.md:6 (tracker-records.md), docs/product/mission-authoring.md:15 and docs/adr/0024 line 52 (plan.md#ready-missions, now "Ready Work").

## Done when

- [ ] Each of the four links points at the section of the skill that now holds the material, or is removed where Atelier's own docs already say it.

## Context

Pick this up once atelier-qj9d's skill change has merged into spadaval/.agents main (finding atelier-93db).
