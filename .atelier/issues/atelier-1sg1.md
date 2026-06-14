---
created_at: "2026-06-14T02:51:59.862223035+00:00"
id: "atelier-1sg1"
issue_type: "task"
labels:
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T06:46:33.123744715+00:00"
status: "done"
title: "Add safe validation command recipes"
updated_at: "2026-06-14T06:46:33.123744715+00:00"
---

## Description

Document known-good validation command forms for cargo test, cargo nextest expressions, rg patterns, formatting, lint, export, doctor, and Python invocation.

## Outcome

Common shell mistakes from the mission logs have explicit safer alternatives.

## Evidence

Quality docs include recipes; examples avoid multiple positional cargo test filters and use python3 where relevant; git diff --check passes.
