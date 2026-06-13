---
created_at: "2026-06-13T00:54:25.169887041+00:00"
id: "atelier-qqey"
issue_type: "task"
labels:
- "smoke"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair description length boundary validation"
updated_at: "2026-06-13T00:54:25.169887041+00:00"
---

## Description

Repair the broad-suite failure in smoke::adversarial::test_boundary_desc_exact_64k. Creating an issue with an exact 64 KiB description writes canonical Markdown but projection refresh rejects the record, indicating that length validation is counting generated issue-section framing instead of the user-provided Description content.

## Outcome

- An issue description at the documented 65536-byte boundary succeeds and leaves the projection current.
- An issue description over the boundary is still rejected.
- Boundary smoke tests cover both accepted and rejected description lengths.

## Evidence

- `cargo nextest run test_boundary_desc_exact_64k test_boundary_desc_over_64k issue_record_renders_and_parses_deterministically issue_parser_contract_accepts_sectioned_body_without_legacy_arrays` passes.
- cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet advances past the previous description-boundary failure.
- atelier lint, atelier export --check, and git diff --check pass.
