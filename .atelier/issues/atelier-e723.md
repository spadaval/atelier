---
created_at: "2026-06-13T20:44:48.184079325+00:00"
id: "atelier-e723"
issue_type: "task"
labels:
- "quality"
- "readiness"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-10qm"
    type: "related"
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Introduce Rust quality hazard scans"
updated_at: "2026-06-13T23:05:41.446593569+00:00"
---

## Description

Factory-style readiness highlights missing complexity, dead-code, duplicate-code, TODO/debt, and unused-dependency detection. Decide and add a pragmatic Rust-focused guardrail set for Atelier without making normal work painfully slow.

## Outcome

- The repo has documented commands for complexity or large-function review, dead-code or unused-code detection, TODO/FIXME debt scanning, and unused dependency checks where practical.
- The selected checks are classified as required handoff checks, extended checks, or advisory diagnostics.
- Findings from the first run are fixed or converted into explicit tracker issues.

## Evidence

- Tooling/config file change or documentation file change names the selected commands and routing.
- Command transcript from an rg debt-marker scan captures the baseline or confirms none.
- Focused command transcripts for the selected Rust quality scans are attached, with follow-up issue IDs for failures.
