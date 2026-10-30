---
created_at: "2026-10-22T22:30:14.044062479+00:00"
id: "atelier-efnf"
issue_type: "task"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-77lm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show open fix-now findings in branch readiness and cover the remaining readiness cases"
updated_at: "2026-10-30T06:40:25.713236166+00:00"
---

## Goal

The Branches row's ✓ approved reflects review.current alone, so a branch with an open fix-now finding reads approved although merge refuses it (finding atelier-77lm). The readiness tests also miss a self-approved row, a trivial exemption, readiness unknown, superseded children exempt from proof, and the five-path cap on Shared files (finding atelier-eqyw). Both were found in review of atelier-tciw.

## Done when

A done owner with an unresolved fix-now finding shows it in its readiness facts, for example `⚠ 1 fix-now open`. The cases listed above have assertions in crates/atelier-cli/tests/session.rs. docs/product/human-cli-output.md#branches says what the approval mark covers.
