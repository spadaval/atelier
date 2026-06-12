---
created_at: "2026-06-12T01:54:58.558494612+00:00"
id: "atelier-bzts"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Specify an Atelier prime workflow command"
updated_at: "2026-06-12T03:24:23.831000963+00:00"
---

## Description

Consider an `atelier prime` command that prints the most important workflow
commands and recovery context for agents. Use `bd prime` as a reference point,
but adapt the shape to Atelier's mission-first workflow, Markdown-first tracker
state, workflow validators, and repository guidance.

## Outcome

- Decide whether `atelier prime` belongs in the product and how it differs from
  root `status`, help, docs, and Agent Factory guidance.
- If accepted, specify the output sections: context recovery, core rules,
  essential commands, common workflows, validation/closeout checklist, and
  repository-specific notes where appropriate.
- The command avoids becoming generic filler; every listed command has a
  concrete reason an agent would use it.
- The spec defines whether `prime` is static guidance, dynamic workspace state,
  or a small blend of both.
- Transcript examples compare the intended output against the useful parts of
  `bd prime`.

## Evidence

Evidence was not specified in the legacy issue record.
