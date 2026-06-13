---
created_at: "2026-06-13T20:37:08.560368636+00:00"
id: "atelier-fyrm"
issue_type: "task"
labels:
- "data-model"
- "runtime"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Align runtime and cache paths with the product contract"
updated_at: "2026-06-13T20:37:08.560368636+00:00"
---

## Description

The current config and init path still place state.db directly under .atelier while docs increasingly describe .atelier/runtime/state.db and separate cache/runtime ownership. Resolve the target and migrate code, config, gitignore, docs, and tests so canonical records and ignored local state are not blurred.

## Outcome

- Runtime database, runtime directory, cache directory, diagnostics, locks, and temp rebuild paths have one documented layout.
- init, storage_layout, doctor, prime, rebuild, gitignore, tests, and Agent Factory guidance agree on that layout.
- Compatibility path handling is removed or explicitly bounded by an artifact decision.

## Evidence

- Fresh `atelier init` transcript and file inspection prove the expected directories and ignored files.
- Missing/stale runtime database `atelier rebuild` scenario passes from canonical Markdown.
- `rg` command output residue scan for `.atelier/state.db` and `runtime_dir` documents only intentional references.
