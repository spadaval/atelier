---
acceptance: []
blocks:
- "atelier-001y"
created_at: "2026-06-09T19:47:39.911679674+00:00"
depends_on:
- "atelier-0006"
- "atelier-001w"
evidence_required: []
id: "atelier-001x"
issue_type: "task"
labels:
- "mission-control"
- "task"
- "tui"
- "ui"
links: []
parent: "atelier-001o"
priority: "P3"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Implement read-only Mission Control TUI views"
updated_at: "2026-06-09T19:47:39.911679674+00:00"
---

Implement the first read-only terminal UI that consumes Mission Control projection output and provides mission, plan, backlog, blocker, validation, evidence, and detail views.

Acceptance: the TUI loads projection JSON, supports keyboard navigation/search/filtering, handles missing fields, and does not mutate tracker state.
