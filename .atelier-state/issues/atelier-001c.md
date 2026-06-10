---
acceptance: []
blocks:
- "atelier-001f"
created_at: "2026-06-09T17:30:35.613401408+00:00"
depends_on:
- "atelier-001b"
evidence_required: []
id: "atelier-001c"
issue_type: "task"
labels:
- "beads:type:feature"
- "cli"
- "legacy-drag"
links: []
parent: "atelier-001a"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Demote or remove inherited utility commands from primary help"
updated_at: "2026-06-09T18:56:01.276721478+00:00"
---

Demote or remove inherited utility commands from the primary public surface after the classification decision is recorded. Initial candidates are `cpitd`, `usage`, `timer`, `daemon`, and `archive`, unless classification intentionally retains one under an explicit non-core namespace.

## Acceptance

`atelier --help` no longer presents inherited utilities as peer core commands unless they are intentionally retained under an explicit integration, experimental, or compatibility tier. Help/output tests or transcript evidence prove the intended surface.
## Validation

- `cargo fmt -- --check`
- `cargo test` or a named focused substitute
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
