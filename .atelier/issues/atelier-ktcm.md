---
created_at: "2026-06-13T04:01:39.751997298+00:00"
id: "atelier-ktcm"
issue_type: "task"
labels:
- "cli"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Improve wrong-kind record ID errors"
updated_at: "2026-06-13T04:01:39.751997298+00:00"
---

## Description

Make common read/orientation commands recognize when an existing Atelier ID belongs to a different record kind. A user or agent who passes a mission ID to an issue command should get a direct correction and suggested command, not a dead-end not-found error.

## Outcome

- Common read and orientation commands detect when a supplied ID exists as a
  different durable record kind.
- Wrong-kind errors name the actual record kind and suggest the correct command,
  for example a mission ID passed to `atelier issue show` suggests
  `atelier mission show <id>`.
- Not-found errors remain concise when the ID does not exist anywhere.
- Help text or product CLI docs describe the record-kind correction behavior.

## Evidence

- Focused CLI transcript or integration test covers a mission ID passed to an
  issue command and shows the suggested mission command.
- Negative transcript or test covers a genuinely unknown ID and proves the
  command does not imply a false match.
- Docs/help parity check confirms documented behavior matches command output.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.
