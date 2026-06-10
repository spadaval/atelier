---
acceptance: []
blocks:
- "atelier-000c"
- "atelier-000j"
- "atelier-000m"
- "atelier-000r"
created_at: "2026-06-09T17:30:35.769115263+00:00"
depends_on:
- "atelier-001c"
- "atelier-001d"
- "atelier-001e"
- "atelier-001g"
evidence_required: []
id: "atelier-001f"
issue_type: "task"
labels:
- "beads:type:validation"
- "cli"
- "validation"
links: []
parent: "atelier-001a"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Validate streamlined CLI surface"
updated_at: "2026-06-09T17:32:21.543651222+00:00"
---

Validate the streamlined public CLI surface after cleanup implementation lands.

## Acceptance

Help snapshots, CLI tests, or command transcript evidence prove the intended public surface. `cargo test`, `atelier lint`, `atelier export --check`, and `atelier doctor` are recorded in notes or linked evidence.
## Validation

- `cargo fmt -- --check`
- `cargo test` or a named focused substitute
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
