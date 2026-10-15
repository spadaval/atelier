---
created_at: "2026-10-15T19:53:46.436649795+00:00"
id: "atelier-kr30"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-kr30
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Record Atelier-only Agent Factory decision and sync the vendored skill"
updated_at: "2026-10-15T19:53:54.975277137+00:00"
---

## Goal

Record the decision that Agent Factory targets Atelier only (ADR 0020), and replace the vendored skill copy with the upstream version that absorbed this repo's Atelier edits, so the copy carries no local divergence.

## Done when

`diff -r` between `.agents/skills/agent-factory` and upstream `spadaval/.agents` reports no differences, links in product docs to the skill resolve, `atelier check` passes, and the ADR and doc edits have independent approval.

## Scope

ADR, vendored skill, and docs that describe Agent Factory record templates. No CLI changes.
