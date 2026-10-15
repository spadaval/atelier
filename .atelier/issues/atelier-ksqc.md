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
closed_at: "2026-10-15T19:06:43.360629782+00:00"
status: "done"
title: "Name the actual record type in blocker validator errors"
updated_at: "2026-10-15T19:06:43.360629782+00:00"
---

## Goal

Blocked transition errors describe the blocked record by its actual type or neutrally, never calling an ordinary issue a mission.

## Done when

A test starting a blocked task asserts the message does not say "mission" and names the record correctly.

## Scope

Blocker validator messages.

## Context

Evaluation output: "validator blockers.transitive_none_open failed: mission atelier-4e8o has direct blocker path …" for a task.

## Close Reason

Blocker validator messages name the record's issue_type. Test blocked_start_names_the_record_type_not_mission asserts 'task <id> has direct blocker path' and no 'mission'.
