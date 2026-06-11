---
acceptance: []
blocks: []
created_at: "2026-06-11T02:01:39.746628354+00:00"
depends_on: []
evidence_required: []
id: "atelier-wnf1"
issue_type: "task"
labels:
- "assignee:root"
- "cli-output"
links: []
parent: null
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Humanize mission and issue detail output"
updated_at: "2026-06-11T02:07:13.369793033+00:00"
---

Scope: Update default non-JSON detail views for mission show and issue show. Remove raw suffix fields and full machine timestamps from human output; render relation/blocker/dependency/recent activity context with readable labels and reusable helpers. Preserve JSON schemas and quiet mode boundaries.

Acceptance: mission show linked work does not append raw relation/open_blockers fields; issue show timestamps are readable; dependency rows avoid shouty raw markers; recent activity field changes are summarized for humans. Add or update focused tests.
