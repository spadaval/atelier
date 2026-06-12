---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-0003"
issue_type: "task"
labels:
- "closeout"
- "spec"
- "task"
- "validation"
- "validator"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate mission status CLI surfaces"
updated_at: "2026-06-11T21:18:42.241144611+00:00"
---

## Description

Closeout validation for the mission status and CLI control surfaces epic. Scope: prove `atelier mission status [<id>]`, `mission list`, and `mission show` give agents enough mission health, blocker, evidence-gap, validator-freshness, closeout-readiness, and next-action context without command-result JSON. Acceptance: focused CLI integration tests and real repository command output cover dashboard mode, per-mission mode, quiet output, blocked missions, evidence gaps, stale tracker/validator failure state, and closeout-needed state; docs and Agent Factory binding point agents to `atelier mission status <id>`.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
