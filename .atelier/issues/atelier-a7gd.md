---
created_at: "2026-06-16T16:18:19.905930262+00:00"
id: "atelier-a7gd"
issue_type: "feature"
labels:
- "cli"
- "doctor"
- "projection"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-m1r7"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T17:47:58.406275002+00:00"
status: "done"
title: "Route projection repair through doctor"
updated_at: "2026-06-16T17:47:58.406275002+00:00"
---

## Description

Make `doctor` own local projection and runtime repair. Operators should not have to know whether stale state came from SQLite, projection metadata, a cache directory, or a rebuild gap.

## Outcome

- `atelier doctor` reports projection freshness and local runtime/cache health in product terms.
- `atelier doctor --fix` repairs ignored local projection/runtime/cache state by rebuilding from tracked Markdown, and it never edits tracked canonical records.
- Status, transition, close, and recovery diagnostics point to `doctor` or `doctor --fix` for local projection repair instead of `export`.
- `rebuild` is hidden, admin-framed, or folded behind `doctor --fix` according to the command-boundary contract.
- Failure messages distinguish canonical Markdown errors from local projection/cache problems and name the correct next command for each.

## Evidence

- CLI transcript or focused test proves a stale projection is repaired through `atelier doctor --fix` and tracked `.atelier/` files remain unchanged.
- CLI transcript or focused test proves invalid canonical Markdown is reported as a file repair problem and is not treated as projection repair.
- Search command transcript over status, transition, close, doctor, and rebuild messages proves normal recovery hints do not route through export.
- Help transcript proves `doctor --fix` explains local-only repair and any retained rebuild command is hidden or admin-framed.
- `atelier lint atelier-a7gd`, `atelier doctor`, focused command-surface tests, and `git diff --check` pass.

## Notes

This issue is about local state repair. It should not change canonical record schemas or workflow policy.
