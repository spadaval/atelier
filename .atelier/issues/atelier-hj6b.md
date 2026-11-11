---
created_at: "2026-11-11T08:56:16.482897516+00:00"
id: "atelier-hj6b"
issue_type: "feature"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-8b0a"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show a mission's history on the Desk: the Activity tab, plan against actual, and when each item merged"
updated_at: "2026-11-11T08:56:16.482897516+00:00"
---

## Goal

Give the Desk's mission page the parts of the old debrief that need the history of acts: a timeline, what was planned at go against what joined after, and when each item merged. The Desk mission (atelier-z4cr) left these out because the view they need, the history that `atelier log` prints, was specified and not built (M5 in `docs/architecture/desk.md`).

## Done when

- [ ] Provisional: once `atelier-app` has the history view that atelier-8b0a builds, `atelier-desk` serves `GET /api/repos/{repo}/missions/{id}/history` from it and the mission page has an Activity tab; nothing in the Desk walks Git history itself.

## Context

Decision note of 2026-11-11 on atelier-z4cr. The strategy `docs/strategies/desk.md` puts the feed in the Desk's next mission, where this item belongs when that mission is planned.
