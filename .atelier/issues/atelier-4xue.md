---
created_at: "2026-06-19T22:42:56.487995393+00:00"
id: "atelier-4xue"
issue_type: "validation"
labels:
- "validation"
- "workflow-policy"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate workflow policy cleanup end to end"
updated_at: "2026-06-19T22:42:56.487995393+00:00"
---

## Description

Perform independent validation of the workflow cleanup. Prove custom issue types, actions, namespaced validators, review completion semantics, branch actions, and migrated workflow config work together from a clean checkout.

## Outcome

- Independent validation proves the cleaned-up workflow policy works from a clean checkout across schema, CLI, docs, migrated config, and transition behavior.
- Any remaining risks are explicitly recorded against the mission before closeout.

## Evidence

- Record command evidence for workflow check, lint, focused CLI integration tests, and full cargo nextest run or documented equivalent.
- Validation maps mission requirements to changed files, tests, and remaining risks.
