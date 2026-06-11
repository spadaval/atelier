---
acceptance: []
blocks:
- "atelier-vfqo"
created_at: "2026-06-11T02:45:01.078632625+00:00"
depends_on:
- "atelier-iv68"
evidence_required: []
id: "atelier-kaei"
issue_type: "task"
labels: []
links: []
parent: "atelier-zjb5"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Delete or simplify JSON result formatter code"
updated_at: "2026-06-11T02:45:01.078632625+00:00"
---

Remove output-format plumbing, JSON serialization branches, and command result renderers that exist only for command-result JSON mode. Acceptance: remaining formatter abstractions are human/quiet focused; dead code and unused dependencies are removed; compiler warnings stay clean.
