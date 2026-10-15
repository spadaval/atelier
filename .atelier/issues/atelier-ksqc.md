---
created_at: "2026-10-15T18:45:05.820843553+00:00"
id: "atelier-ksqc"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Name the actual record type in blocker validator errors"
updated_at: "2026-10-15T18:45:05.820843553+00:00"
---

## Goal

Blocked transition errors describe the blocked record by its actual type or neutrally, never calling an ordinary issue a mission.

## Done when

A test starting a blocked task asserts the message does not say "mission" and names the record correctly.

## Scope

Blocker validator messages.

## Context

Evaluation output: "validator blockers.transitive_none_open failed: mission atelier-4e8o has direct blocker path …" for a task.
