---
created_at: "2026-06-29T17:39:04.246375805+00:00"
id: "atelier-97p7"
issue_type: "task"
labels:
- "mission"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:17:11.413488857+00:00"
status: "done"
title: "Add draft-to-ready mission workflow"
updated_at: "2026-06-30T15:17:11.413488857+00:00"
---

## Description

The configured mission workflow starts new missions in draft, provides an explicit transition from draft to ready, and uses workflow policy to express when a mission can become active or close.

## Outcome

The mission workflow config includes a `draft` starting state and explicit transitions that move a mission from drafted planning into ready execution and then active work or closeout according to policy.
