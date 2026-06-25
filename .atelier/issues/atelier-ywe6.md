---
created_at: "2026-06-13T00:20:47.457798158+00:00"
id: "atelier-ywe6"
issue_type: "task"
labels:
- "cli"
- "docs"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-trr2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T00:23:16.530191920+00:00"
status: "done"
title: "Repair SPEC workflow command drift"
updated_at: "2026-06-13T00:23:16.530191920+00:00"
---

## Description

Repair the SPEC command-surface drift found by the independent `atelier-trr2`
validation rerun. The product intent should describe the implemented normal
workflow surfaces instead of presenting old `atelier work start` /
`atelier work finish` commands or raw `atelier workflow validate` as normal
representative commands.
- `PRODUCT_INTENT.md` uses root `atelier start <issue-id>` and
  `atelier finish [issue-id]` for normal tracked work examples.
- `PRODUCT_INTENT.md` representative command examples include current signpost/domain
  surfaces such as `atelier status`, `atelier prime`, mission status/show,
  issue transition options, evidence capture, lint/export/doctor, and exclude
  raw `atelier workflow validate` as a normal user command.
- The workflow-validator product intent still describes validation policy
  without contradicting the hidden advanced/internal diagnostic boundary.
- Focused docs drift checks prove SPEC, product docs, help, tests, and Agent
  Factory guidance no longer disagree on the normal signpost surfaces.
- Focused residue search for stale normal command examples in `PRODUCT_INTENT.md`,
  product docs, Agent Factory guidance, and tracker signpost issues.
- Focused docs/help parity tests for workflow diagnostics and raw validator
  absence from normal guidance.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
