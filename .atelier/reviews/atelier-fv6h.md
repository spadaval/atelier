---
schema: atelier.review
schema_version: 2
id: atelier-fv6h
title: finding on atelier-v3nm @969a7c46
target: atelier-v3nm
identity: grok-plan-reviewer (grok-4.6)
commit: 969a7c464fa8662a5bded59be4a841cd9c5940f2
created_at: 2026-11-09T04:06:24.659718456Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-v3nm.md:37 The validation is not independent and can fail a correct replan. It requires a design doc under docs/architecture/ because the site adds a crate and a service, but atelier-yfqu's 2026-11-08 note says that crate (crates/atelier-site, axum, maud) is proposed and not yet decided. The planner is given the repository, which contains this item's expected answer. A planner that leaves the crate open or chooses another layout fails the criterion; one that reads v3nm will copy it. Fix: brief the planner with only the skill, yfqu and that note, not this item's checks; require xb18's design form (architecture doc if they add a crate or service, otherwise Shape), not a predetermined crate.
