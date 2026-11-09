---
schema: atelier.review
schema_version: 2
id: atelier-p3e0
title: approval on atelier-uma8 @5d6af8d3
target: atelier-uma8
identity: gpt-6.1-sol-plan-r2
commit: 5d6af8d3916eedece9ee842ee21a9a3fe97ae824
created_at: 2026-11-09T21:05:27.971214554Z
event:
  event: approval
---

Verdict: merge. Approve the revised plan for implementation; the inbox is a required mission outcome and the proposed projection, route and page are proportionate.

Sizes: complexity medium (one app projection plus fleet API, React page and tests); value high (the mission needs a human notification queue); risk medium (a wrong queue reaches the human, but the dashboard is read-only and reversible).

Contract: pass at planning level. status_view(index, Some(policy)).work.human is precisely the source status.rs uses for Needs a human, without the board's extra apply_ready_gates. The plan covers repository grouping, the API and fixture-backed page, item and mission links, shared Inbox count on home and boards, and a host Playwright run with command and counts recorded in the Report. The Unknown explicitly permits restricting drafts to planning-approved missions; initial workflow status plus an existing approval event implements that decision without a new field.

Fitness: pass. atelier-app owns queue selection and mission membership; atelier-site adds fleet/base context and existing MissionSummary URLs/review while caching the projection inside Loaded; React lays out the API's facts. Reusing the current record index, cache and page components is the simplest construction meeting the contract. No new role, record field or write action is planned.

Findings: none. The final decision at .atelier/issues/atelier-uma8.md:67 resolves atelier-jx4p: union mission scope with typed-relation endpoints and both get_blockers/get_blocking endpoints, deduplicate and exclude superseded/declined missions, leaving get_typed_relations unchanged. record_index.rs puts blocked_by and legacy blocks edges only in the blocker graph, so the revised union reaches the done alpha-m2 target. The added hand-written expected IDs and review=debrief assertion can fail on an omitted target, unlike merely regenerating JSON or following only emitted links. The fixture decision, approved draft, unapproved draft and superseded-only item cover the other required cases; CLI comparisons, Vitest and fixture/host Playwright cover the remaining layers. atelier-3km4 becomes item-only rather than linking the unrelated superseded atelier-gg6b; the contract allows that fallback when records do not identify atelier-vt8q.

Open questions: none blocking implementation.

Residual risk and tests not run: this approves a plan, not delivered behavior. Rust/Vitest assertions and fixture/host browser checks must still be implemented and run, and the Report must state both Unknown decisions and the host command/counts. No implementation tests or browser validation were run in this review. No transient-Git or injected-fault hardening is requested.

Proof: ran atelier status first; read the full issue and revision diff, atelier-jx4p, atelier-ei8w, atelier-3km4 and atelier-gg6b, fleet-dashboard strategy and site architecture, and the status/classification, mission scope/review, relation indexing, site API/cache and fixture/frontend test code. Changes: this approval record only; no code changes or commits.
