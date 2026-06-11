---
acceptance: []
blocks:
- "atelier-kaei"
created_at: "2026-06-11T02:45:00.803546168+00:00"
depends_on:
- "atelier-fspm"
evidence_required: []
id: "atelier-iv68"
issue_type: "task"
labels: []
links: []
parent: "atelier-zjb5"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Remove --json CLI options for command result rendering"
updated_at: "2026-06-11T02:45:00.803546168+00:00"
---

Remove command-result --json flags from relevant clap definitions and command dispatch while preserving unrelated diagnostic logging options if approved by the boundary decision. Acceptance: help output for representative commands no longer lists result --json; invoking removed flags fails with a clear standard CLI error; no command silently ignores the flag.
