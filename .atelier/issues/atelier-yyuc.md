---
created_at: "2026-06-15T06:20:43.787628365+00:00"
id: "atelier-yyuc"
issue_type: "bug"
labels:
- "state-model"
- "validation"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Rebuild must recreate missing runtime directory"
updated_at: "2026-06-15T06:20:43.787628365+00:00"
---

## Description

Validation of atelier-t35w deleted the entire .atelier/runtime directory in a disposable mission tracker copy. Both `atelier status` and `atelier rebuild` failed with unable to open `.atelier/runtime/state.db` because the runtime directory was missing. Runtime/cache state is documented as rebuildable, so direct rebuild/health recovery must recreate the missing directory before opening the SQLite database.

## Outcome

- `target/debug/atelier rebuild` succeeds after `.atelier/runtime/` and `.atelier/cache/` are deleted from a checkout with valid canonical `.atelier/` Markdown.
- `target/debug/atelier status` either recovers through the documented health path or reports actionable `doctor --fix` guidance instead of a raw SQLite open error.
- Current work remains derived from canonical `status: "in_progress"` Markdown after recovery.

## Evidence

- Disposable-repo transcript deletes `.atelier/runtime` and `.atelier/cache`, runs `target/debug/atelier rebuild`, then `target/debug/atelier status`; both succeed or the documented health path succeeds with current work visible.
- Add or update a focused integration test that removes the runtime directory itself, not only `state.db` inside an existing directory.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.
