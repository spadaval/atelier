---
created_at: "2026-06-13T21:58:03.385736554+00:00"
id: "atelier-kpm8"
issue_type: "task"
labels:
- "architecture"
- "stabilization"
- "tests"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Consolidate RecordStore-aware integration test fixtures"
updated_at: "2026-06-13T21:58:03.385736554+00:00"
---

## Description

Integration tests duplicate canonical record discovery, ID extraction, section
mutation, and front matter parsing. The large `tests/cli_integration.rs` file
and smoke harness use direct string searches and ad hoc Markdown rewrites,
which makes record format changes amplify across unrelated CLI behavior tests.

## Outcome

- Shared test fixture helpers expose RecordStore-aware operations for creating,
  locating, reading, and editing canonical records in integration tests.
- Tests that intentionally corrupt Markdown or projection state use explicit
  low-level fixture helpers named for that purpose.
- High-level CLI tests stop depending on incidental front matter formatting or
  stdout ID parsing when a fixture API can express the behavior directly.

## Evidence

- Test helper diff or review artifact identifies the shared fixture API and the
  intentionally low-level corruption helpers.
- Representative CLI integration and smoke tests are migrated to the shared
  helpers without broad behavior changes.
- Focused test command transcripts prove migrated tests still pass.
- `rg` command output for duplicated ID extraction, title scanning, evidence
  front matter parsing, and string-based issue section rewrites is attached or
  cited with retained exceptions.
- `cargo fmt -- --check` and relevant focused tests pass.

## Notes

Audit evidence: `tests/cli_integration.rs` is over 10k lines and duplicates ID
scanning, evidence front matter parsing, title-based record discovery, and
string replacement of issue Markdown; `tests/smoke/harness.rs` repeats stdout
and `.atelier/issues` assumptions.
