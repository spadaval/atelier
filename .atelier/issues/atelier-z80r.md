---
created_at: "2026-06-13T02:39:43.398190553+00:00"
id: "atelier-z80r"
issue_type: "task"
labels:
- "assignee:root"
- "process"
- "proof"
- "validation"
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
closed_at: "2026-06-13T04:11:53.593274943+00:00"
status: "done"
title: "Define qualitative and quantitative validation standards"
updated_at: "2026-06-13T04:11:53.593274943+00:00"
---

## Description

Define how Atelier handles subjective and quantitative validation. Some work, such as improving information hierarchy, requires expert judgment and should not be over-specified before implementation. Other work, such as performance improvement, should use hard numbers whenever practical.
- Validation guidance explicitly allows qualitative pass/fail judgment for subjective product, UX, documentation, and information-architecture claims.
- Subjective validation records enough evaluator context, scenario or baseline, decision rationale, and captured evidence for the result to be inspectable.
- Quantitative validation is required or strongly preferred for numerical claims such as performance, latency, count reduction, size, coverage, or error-rate changes.
- Work-item guidance warns against over-specifying subjective output before implementation while still requiring enough evidence for review.
- File-change review of validation and work-item authoring guidance shows qualitative and quantitative validation rules.
- Review artifact includes examples for `mission list` information hierarchy and a performance improvement task.
- `atelier lint`, `atelier export --check`, and docs whitespace check commands pass.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
