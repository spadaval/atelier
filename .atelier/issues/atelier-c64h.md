---
created_at: "2026-06-13T17:29:11.073668955+00:00"
id: "atelier-c64h"
issue_type: "task"
labels:
- "artifact"
- "contract"
- "docs"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fmri"
  - kind: "issue"
    id: "atelier-lv4s"
  - kind: "issue"
    id: "atelier-n0p4"
  - kind: "issue"
    id: "atelier-y041"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define v1 workflow YAML contract"
updated_at: "2026-06-13T17:36:48.694293176+00:00"
---

## Description

Define the repo-owned workflow policy contract before parser or command implementation. The contract must use .atelier/workflow.yaml, explicit status objects with categories, issue-type workflow selection, named transitions, configured built-in validators with params, simple guidance templates, and strict configuration errors.

## Outcome

- Product docs define .atelier/workflow.yaml as the v1 policy source and remove config-selected-policy ambiguity for this mission.
- The schema documents statuses, categories, workflows, transitions, validator definitions and params, guidance templates, terminal done states, and issue-type mappings.
- Examples include a standard review/proof workflow and a lightweight spike workflow that does not require first-class evidence for low-risk closure but still records an inspectable close reason and uses the review path.
- Deferred features are explicit: custom issue types, custom validator execution, expression validators, hooks, triggers, post-functions, waivers, and workflow projection tables are out of v1.

## Evidence

- Product docs file changes contain complete standard and lightweight examples with strict error expectations.
- A reviewer can map every planned implementation issue to a documented schema behavior.
- atelier lint and atelier export --check pass after the contract update.
