---
created_at: "2026-06-13T02:35:53.408915343+00:00"
id: "atelier-rc1v"
issue_type: "task"
labels:
- "assignee:root"
- "docs"
- "process"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ovs0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define contract-first workflow policy"
updated_at: "2026-06-13T04:11:52.365854305+00:00"
---

## Description

Clarify when work must start with docs, specs, command contracts, or tests before implementation. The policy should avoid ceremony for tiny local tasks while requiring contracts for public behavior and workflow policy.

## Outcome

- Docs-first expectations cover public CLI semantics, workflow policy, evidence schema, and Agent Factory rules.
- Test-first expectations cover CLI behavior, validators, projection/rebuild behavior, and regression fixes.
- The policy explains when strict TDD is optional versus required.

## Evidence

- File-change review of Agent Factory and quality docs shows the policy.
- Review artifact includes example work items demonstrating Outcome and Evidence wording for docs-first and test-first work.
- `atelier lint` and `atelier export --check` pass.
