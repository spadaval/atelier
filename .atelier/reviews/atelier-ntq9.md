---
schema: atelier.review
schema_version: 2
id: atelier-ntq9
title: finding on atelier-yfqu @28ae3d61
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: 28ae3d61ecce47e8befb0e1469e16122d8addf04
created_at: 2026-11-09T00:51:50.280260076Z
event:
  event: finding
  recommendation: fix_now
---

medium: Small dependencies do not bound the new service's load on the phone's T3 host.
docs/strategies/fleet-dashboard.md:52-59,108-110 and .atelier/issues/atelier-zpmb.md:34-40.
The plan generates debriefs on each HTTP request across all repositories, but acceptance has no bound on simultaneous work, subprocesses, request duration or memory. The existing debrief traverses Git history (crates/atelier-app/src/mission_review.rs:356-357 and record_history.rs:56 onward). Concurrent refreshes can repeatedly run that work; a small HTTP library does not contain it. This conflicts with the stated requirement that the server stay light on the host that supplies phone access. Give the serve item measurable resource limits and a bounded/degraded response for excessive work, and validate concurrent requests against a large local mission while confirming T3 stays responsive. Specify deployment resource containment and verify the existing T3 publication still works after adding the route, rather than checking only the new site's 200.
Recommendation: FIX NOW; the ready server item does not make the explicit shared-host constraint testable or contain this current request path.
