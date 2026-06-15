---
created_at: "2026-06-15T05:11:30.387029683+00:00"
id: "atelier-fchz"
issue_type: "validation"
labels:
- "closeout"
- "rewrite"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T19:01:47.546497114+00:00"
status: "done"
title: "Validate and close out crate rewrite mission"
updated_at: "2026-06-15T19:01:47.546497114+00:00"
---

## Description

Independently validate and close out the in-place crate rewrite mission after all implementation epics and the absorbed active-work removal epic are complete. This issue does not implement fixes; failures become follow-up work.

## Outcome

- Every mission validation line is mapped to linked child work and attached evidence.
- CLI/help/docs parity, Markdown round trips, SQLite rebuild and stale recovery, active-work removal, and representative mission/issue/evidence workflows are independently classified.
- Repository health checks pass or deferred-check evidence records name residual risk and follow-up owners.
- The mission is ready for closeout only after blocker epics and validation gaps are resolved.

## Evidence

- Closeout evidence record maps mission outcomes to child issues and evidence IDs.
- Independent validation transcript includes `cargo fmt -- --check`, `cargo nextest run`, relevant extended ignored tests, `atelier lint`, `atelier export --check`, and `atelier doctor`.
- Scenario evidence record covers missing/stale SQLite DB recovery, status-derived current work, and representative CLI workflows.
