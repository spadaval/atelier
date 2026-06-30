---
created_at: "2026-06-29T18:20:30.160139983+00:00"
id: "atelier-45yt"
issue_type: "task"
labels:
- "cli"
- "dashboard"
- "mission"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:09:55.489261469+00:00"
status: "done"
title: "Rework work mission dashboard around operational questions"
updated_at: "2026-06-30T15:09:55.489261469+00:00"
---

## Description

Rewrite `atelier work mission <mission-id>` around what an orchestrator needs to know inside one mission: what is scoped, what is ready, what is blocked, what is done, and what command helps next. Keep the dashboard mission-scoped and avoid routing users to repo-wide queue firehoses for mission drill-down.

## Outcome

The mission dashboard header renders as `<id> [mission] <status> - <title>` and strips a redundant leading `Mission:` from display titles only. The summary shows concrete progress counts, blocked state, and backlog only when the workflow policy and scoped rows make backlog meaningful. The dashboard hides empty scoped buckets, removes default `Proof gaps` and `Health: needs proof` language, suppresses closeout readiness while scoped work remains nonterminal, and shows blocker IDs for blocked rows. Footer commands prefer mission-scoped filters, `issue show`, and transition inspection only when relevant.
