---
created_at: "2026-06-13T04:01:40.869256949+00:00"
id: "atelier-88ud"
issue_type: "task"
labels:
- "cli"
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T15:57:12.599042636+00:00"
status: "done"
title: "Detect stale installed CLI against canonical records"
updated_at: "2026-06-13T15:57:12.599042636+00:00"
---

## Description

Define and implement the operator behavior for installed atelier binaries that lag behind the canonical record schema or repository command contract. The goal is for agents to identify binary/schema drift quickly instead of switching blindly between installed atelier, target/debug/atelier, and cargo run.
- Atelier detects or clearly explains when the installed binary cannot read the
  current canonical record schema or repository command contract.
- Operator output distinguishes stale installed-binary/schema drift from
  malformed tracker records and ordinary command failures.
- Agent-facing docs state when to use installed `atelier`, when to rebuild and
  use `target/debug/atelier`, and when `cargo run -- ...` is appropriate.
- The normal tracker workflow still defaults to installed `atelier` unless local
  CLI behavior is being tested or binary/schema drift is proven.
- Command transcript or focused test demonstrates the stale-binary/schema-drift
  diagnostic or documented fallback behavior.
- File-change review shows Agent Factory and repo docs agree on installed versus
  local binary usage.
- Negative transcript or test shows malformed canonical records are still
  reported as record errors, not stale-binary errors.
- `atelier lint`, `atelier export --check`, docs whitespace checks, and relevant
  CLI tests pass.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
