---
schema: atelier.item
schema_version: 3
id: atelier-hj6b
title: 'Show a mission''s history on the Desk: the Activity tab, plan against actual, and when each item merged'
status: draft
mission: null
labels:
- feature
priority: low
blocked_by:
- atelier-8b0a
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T08:56:16.482897516Z
---

## Goal

Give the Desk's mission page the parts of the old debrief that need the history of acts: a timeline, what was planned at go against what joined after, and when each item merged. The Desk mission (atelier-z4cr) left these out because the view they need, the history that `atelier log` prints, was specified and not built (M5 in `docs/architecture/desk.md`).

## Done when

- [ ] Provisional: once `atelier-app` has the history view that atelier-8b0a builds, `atelier-desk` serves `GET /api/repos/{repo}/missions/{id}/history` from it and the mission page has an Activity tab; nothing in the Desk walks Git history itself.
- [ ] Each done item on the page shows how many files and lines its merge changed, read from its merge commit, so a finished mission's size is plain without opening a diff.

## Context

Decision note of 2026-11-11 on atelier-z4cr. The strategy `docs/strategies/desk.md` puts the feed in the Desk's next mission, where this item belongs when that mission is planned.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-z4cr (Build the Desk: the human's place to plan missions and read results, on the redesigned records)
