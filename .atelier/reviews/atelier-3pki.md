---
schema: atelier.review
schema_version: 2
id: atelier-3pki
title: finding on atelier-zpmb @a9cd1265
target: atelier-zpmb
identity: gpt-6.1-sol
commit: a9cd1265fafcb1a44917fc15c10033f3120137b2
created_at: 2026-11-09T14:38:19.745672496Z
event:
  event: finding
  recommendation: fix_now
---

Medium: The first app cannot select brief versus debrief from the promised API view models.
Locations: .atelier/issues/atelier-zpmb.md:35-36; .atelier/issues/atelier-yn69.md:33; docs/strategies/fleet-dashboard.md:120-134.
The app must show the brief exactly while a mission is in its workflow's initial status. The API is specified as the existing brief/debrief views with only links and a commit added. Brief and Debrief expose the mission's status and status_category, but neither exposes the workflow's initial_status (mission_review.rs:258-303,1343-1363; Issue in atelier-core/src/lib.rs:224-250 has no such field). Category cannot supply it: this repository maps draft, backlog and todo all to category todo, but only draft is the mission workflow's initial status (.atelier/workflow.yaml:24-42). Hard-coding draft would break a valid custom workflow and move work semantics into the frontend. The same handoff leaves the mission enumeration for the required home-page links unspecified: /api/repos is described only as repositories and counts, and all mission routes require an already-known id.
Recommendation: FIX NOW. State which shared app view/API response supplies the initial-status/default-review decision (a server-chosen review link is sufficient), and the mission list needed by the first home page. Require the generated types and fixture JSON to include that contract, with a fixture whose initial status is not draft and whose later status shares its category. The two workers should not have to independently invent the missing interface.
