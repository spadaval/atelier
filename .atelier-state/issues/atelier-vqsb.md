---
acceptance: []
blocks:
- "atelier-2h0d"
created_at: "2026-06-11T02:45:43.526136929+00:00"
depends_on:
- "atelier-vfqo"
evidence_required: []
id: "atelier-vqsb"
issue_type: "validation"
labels: []
links: []
parent: "atelier-eq2d"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add removal coverage for representative commands"
updated_at: "2026-06-11T04:31:02.124296409+00:00"
---

Add or update integration tests proving command-result --json mode is absent from representative list/show/mutation/workflow commands and help output. Acceptance: tests fail if removed JSON result mode is accidentally reintroduced.
