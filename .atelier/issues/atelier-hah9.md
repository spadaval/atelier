---
created_at: "2026-06-12T05:12:17.327797507+00:00"
id: "atelier-hah9"
issue_type: "task"
labels:
- "agent-factory"
- "reliability"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define validation subagent proof protocol"
updated_at: "2026-06-12T05:12:17.327797507+00:00"
---

## Description

Define the validation subagent process so validation work proves outcomes rather
than rubber-stamping implementation claims.

## Outcome

- Validation agents verify each Outcome line against observable evidence.
- Validation agents inspect command help, docs, ignored tests, and stale tests
  where relevant.
- Validation reports distinguish pass, fail, blocked, and not-applicable with
  evidence IDs or command transcripts.
- Agent Factory validate guidance requires evidence attachment before declaring
  success.

## Evidence

- Update Agent Factory validate procedure.

- Run a process review against a representative mission or epic.

- Attach evidence showing the process catches at least one missing-proof case.
