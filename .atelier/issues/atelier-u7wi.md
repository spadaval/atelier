---
created_at: "2026-06-23T16:21:20.085065091+00:00"
id: "atelier-u7wi"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T20:29:34.906384641+00:00"
status: "done"
title: "Update docs and command audit for cache terminology"
updated_at: "2026-07-06T20:29:34.906384641+00:00"
---

## Description

Update product, architecture, command audit, and operator docs to use cache terminology and the new storage/domain/cache model.

## Outcome

- Docs no longer describe ordinary behavior as canonical mutations or projection refreshes.

## Evidence

- Command: `rg -n -i 'projection|canonical mutation' docs crates` audits current
  record-file/domain-cache guidance, historical migration text, and the separate
  Mission Control JSON projection contract.
- Tests: `cargo nextest run -p atelier-records -p atelier-app` and `cargo nextest
  run -p atelier-cli --test cli_integration` prove updated help, diagnostics,
  and record/cache terminology across the affected command surface.
- Checks: `target/debug/atelier check --help`, `cargo fmt -- --check`, `git diff
  --check`, and `atelier check atelier-u7wi` prove docs/help parity, formatting,
  whitespace, and tracker validity.
