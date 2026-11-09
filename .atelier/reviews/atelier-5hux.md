---
schema: atelier.review
schema_version: 2
id: atelier-5hux
title: finding on atelier-yfqu @a9cd1265
target: atelier-yfqu
identity: grok-4.6
commit: a9cd1265fafcb1a44917fc15c10033f3120137b2
created_at: 2026-11-09T14:41:50.527030728Z
event:
  event: finding
  recommendation: fix_now
---

The API has no mission list, so the first pages cannot be built from the routes the plan names. Design (docs/strategies/fleet-dashboard.md:120-124) and atelier-zpmb expose GET /api/repos (repositories and their counts), /api/<repo>/mission/<id>/brief, /debrief and /strategy. atelier-yn69 requires / to list repositories with a link to each mission, including closed ones whose debriefs the criterion must open. Counts are not ids, titles or statuses; the board route that would list missions is atelier-ahpf, which yn69's Scope excludes. A worker finishing zpmb then yn69 has no contract for the home page's data. Put each repository's missions (id, title, status) on /api/repos or add GET /api/<repo>/missions, and have yn69 consume that list.
