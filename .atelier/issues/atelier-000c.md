---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000c"
issue_type: "epic"
labels:
- "epic"
- "milestone"
- "mission-control"
- "spec"
- "validator"
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0003"
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-001m"
  - kind: "issue"
    id: "atelier-001o"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Mission status and CLI control surfaces"
updated_at: "2026-06-11T21:18:57.261718121+00:00"
---

## Description

Expose enough mission-control functionality through existing CLI workflows before building any separate projection or UI. Scope: mission status/control commands, mission list/show status signals, workflow/config health summaries, blockers, evidence gaps, validator failures, closeout-needed states, and concrete next CLI actions. Out of scope for this mission: a full Mission Control JSON projection, read-only TUI, live agent-run tracking, retry queues, or session metrics. Acceptance: CLI surfaces make active mission health legible for agents and orchestrators without command-result JSON; mission status covers ready/blocked/done/backlog work, evidence gaps, validator failures, tracker freshness, active work/worktree context where available, and closeout-needed states; tests cover representative mission states and quiet output.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
