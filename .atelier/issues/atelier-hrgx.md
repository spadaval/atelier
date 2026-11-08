---
created_at: "2026-11-08T05:52:16.020750104+00:00"
id: "atelier-hrgx"
issue_type: "loose_end"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-45gr"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-nz6a"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T23:41:02.597713539+00:00"
status: "done"
title: "Check that npx skills add installs the published skill in a fresh home"
updated_at: "2026-11-08T23:41:02.597713539+00:00"
---

## Goal

Prove the published install line works for someone who is not Sumeet, once master is pushed.

## Done when

- [x] In a scratch `HOME`, `npx skills add spadaval/atelier --skill agent-factory -g` installs the skill from GitHub `master`, the installed SKILL.md starts with the new opening, and `atelier doctor` with that `HOME` reports the skill current.

## Report

HOME=~/.cache/hrgx-home: npx skills add spadaval/atelier --skill agent-factory -g installed ~/.agents/skills/agent-factory (plus the ~/.claude link) from GitHub master 9509222a; diff -r against master's skills/agent-factory is empty; SKILL.md opens with 'Run atelier status first' and both install lines; atelier doctor with that HOME prints 'Agent Factory skill: current.'
