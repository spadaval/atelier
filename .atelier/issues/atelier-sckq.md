---
created_at: "2026-06-12T20:29:29.176933505+00:00"
id: "atelier-sckq"
issue_type: "task"
labels:
- "cli"
- "implementation"
- "status"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-trr2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Repair root and mission status signpost surfaces"
updated_at: "2026-06-12T22:10:25.441696940+00:00"
---

## Description

Repair root and mission status surfaces according to the closed signpost
contracts in `atelier-rqvv` and `atelier-v02t`. Status commands should orient
agents without dumping full mission records or raw validator internals.

## Outcome

- Root `atelier status` reports local state, active work, active mission,
  tracker health, bounded ready work, blockers, recent activity, and
  intent-labeled next actions.
- `atelier mission status <id>` owns mission advancement and closeout readiness:
  linked work, blockers, evidence gaps, workflow failures, and next commands.
- `atelier mission show <id>` remains the durable record view and does not
  become the operational closeout dashboard.
- Empty, active, blocked, dirty, stale, and no-active-mission states render
  bounded, actionable output.
- Docs and Agent Factory guidance route agents to the right status drill-down.

## Evidence

- CLI transcript tests for root status and mission status in empty, active,
  blocked, dirty, stale, and closeout-ready states.
- Transcript or snapshot proving `mission show` remains a document view.
- Docs/help parity check against `atelier-rqvv` and `atelier-v02t`.
