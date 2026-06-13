---
created_at: "2026-06-13T02:39:43.398190553+00:00"
id: "atelier-z80r"
issue_type: "task"
labels:
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
status: "open"
title: "Define qualitative and quantitative validation standards"
updated_at: "2026-06-13T02:39:43.398190553+00:00"
---

## Description

Define how Atelier handles subjective and quantitative validation. Some work, such as improving information hierarchy, requires expert judgment and should not be over-specified before implementation. Other work, such as performance improvement, should use hard numbers whenever practical.

## Outcome

- Validation guidance explicitly allows qualitative pass/fail judgment for subjective product, UX, documentation, and information-architecture claims.
- Subjective validation still names the evaluator role, scenario, comparison baseline, decision criteria, and captured evidence so the result is inspectable.
- Quantitative validation is required or strongly preferred for numerical claims such as performance, latency, count reduction, size, coverage, or error-rate changes.
- Work-item guidance warns against over-specifying subjective output before implementation while still requiring enough evidence for review.

## Evidence

- File-change review of validation and work-item authoring guidance shows qualitative and quantitative validation rules.
- Review artifact includes examples for `mission list` information hierarchy and a performance improvement task.
- `atelier lint`, `atelier export --check`, and docs whitespace check commands pass.
