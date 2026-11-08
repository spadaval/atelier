---
created_at: "2026-11-08T05:35:32.950046101+00:00"
id: "atelier-zjgy"
issue_type: "task"
labels:
- "tooling"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Find out why a third of commits touch only tracker records, and cut what is not needed"
updated_at: "2026-11-08T05:35:32.950046101+00:00"
---

## Goal

In the 2026-11-08 audit, 59 of 183 non-merge commits in life-on-rails' Atelier era and 119 of 133 in atelier's mmpv window touched only `.atelier/`. Find which commands produce them and whether batching records would cut that without losing what review needs.

## Done when

- [ ] A note on this item breaks those commits down by command and proposes changes, each with its expected saving.
