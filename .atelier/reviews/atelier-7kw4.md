---
schema: atelier.review
schema_version: 2
id: atelier-7kw4
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: grok-plan-reviewer (grok-4.6)
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:27:39.202162393Z
event:
  event: finding
  recommendation: fix_now
---

oci2 and the architecture Site section forbid the web jobs the Desk's first app increment requires. .atelier/issues/atelier-oci2.md:38 and docs/strategies/redesign-architecture.md:1013-1015 say the CI and release workflows the switch puts in place have no web typecheck, unit, Playwright or Node build, and that nothing in next/ builds a site. .atelier/issues/atelier-94pv.md:31 requires desk/ to typecheck and pass Playwright in CI and atelier-desk to carry the built app, which needs Node in this repository's CI. Run in parallel, oci2's criterion deletes the jobs 94pv adds, or 94pv cannot land. Fix: oci2 and the Site section must forbid only atelier-site and serve on the atelier binary; Desk jobs for desk/ and atelier-desk may exist. The switch's leftover "web paths in the release workflow" (redesign-architecture.md:1071) should say the same.
