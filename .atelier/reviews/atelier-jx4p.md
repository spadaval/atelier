---
schema: atelier.review
schema_version: 2
id: atelier-jx4p
title: finding on atelier-uma8 @9c3da9d7
target: atelier-uma8
identity: gpt-6.1-sol-plan
commit: 9c3da9d7ab1bd4e7bfc9b69bb05da2526f7ed70f
created_at: 2026-11-09T21:02:50.371508183Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: The mission-link decision names a helper that omits dependency links. In .atelier/issues/atelier-uma8.md:55-56, direct mission targets come from get_typed_relations, explicitly including a loose end linked by blocked_by; the plan at line 66 relies on that rule. But crates/atelier-app/src/record_index.rs:916-918 routes BLOCKED_BY into declared_blockers and continues before adding a typed relation, and legacy relationships.blocks edges also live only in the blocker graph. Therefore the planned needs-human loose end blocked_by done mission alpha-m2 gets no mission link with the prescribed algorithm, failing the second Done when line and the decision. Fix the decision and design before implementation: union mission scope and ordinary typed relations with direct get_blockers/get_blocking endpoints, deduplicate, then filter superseded and declined missions. Do not change the shared meaning of get_typed_relations. Require the Rust fixture assertion to expect alpha-m2 in this entry’s mission IDs and review=debrief; generated JSON or loading only emitted links cannot detect an omitted target.

Verdict: merge after this plan correction; the inbox serves a required mission outcome and does not need shrinking. Sizes: complexity medium (one app projection plus API, UI and tests), value high (the notification queue is required by the mission), risk medium (a wrong queue reaches the human but changes no repository data). Contract: fail as written because the named link algorithm misses the planned loose-end target. Fitness: otherwise pass; selection belongs in atelier-app, fleet composition and URL context in atelier-site, and layout in React, reusing Loaded and the existing cache. status_view(...).work.human is the exact source status.rs:80 and :93 use; selecting initial-status missions with a recorded approval settles the draft-noise Unknown within its allowance. Filtering retired missions makes atelier-3km4 item-only, which the Unknown explicitly permits when records cannot identify vt8q. The two-repository CLI comparison, API JSON, Vitest and fixture/host Playwright layers are suitable once expected target IDs are asserted independently of generated output. No new role or record field is needed.

Open questions: none beyond correcting R1. Proof: ran atelier status first and read the issue contract and notes, fleet-dashboard strategy, site architecture, referenced records and finding atelier-ei8w, and the selection, relation indexing, API/cache and fixture/test code. This is a planning review; implementation, browser validation and test execution remain for the implementer. No transient-Git or injected-fault hardening is requested.
