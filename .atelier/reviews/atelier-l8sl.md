---
schema: atelier.review
schema_version: 2
id: atelier-l8sl
title: finding on atelier-yn69 @a9cd1265
target: atelier-yn69
identity: gpt-6.1-sol
commit: a9cd1265fafcb1a44917fc15c10033f3120137b2
created_at: 2026-11-09T14:38:20.300413692Z
event:
  event: finding
  recommendation: fix_now
---

Medium: The web-job path rule misses changes to the Rust views that are the API contract.
Locations: .atelier/issues/atelier-yn69.md:36; docs/strategies/fleet-dashboard.md:120-126,148-154; scripts/ci-jobs.sh:20-59; .github/workflows/ci.yml:30-91.
The ready item's observable trigger requires the web job when crates/atelier-site/ changes. The strategy also promises it when the API changes, but explicitly makes that API the shared atelier-app view models. Brief, Debrief, criteria and their nested types live under crates/atelier-app and crates/atelier-core, not atelier-site. A behavior or serialization change there can therefore merge with the web job skipped under the item's path rule; unchanged fixture snapshots/types on the existing fixture cases do not guarantee that Playwright against the real server, or components handling another input, still work. Cargo.lock and CI/build wiring can also change the executable without touching the site directory. Today the selector has only a rust output, and its no-base branch prints only that output, so both the dependency rule and its conservative fallback need an explicit web contract.
Recommendation: FIX NOW. Require the selector to run web checks for its Rust API/model dependencies and relevant build/CI inputs, or choose the simpler rule of running it for all Rust changes. Test those paths and the missing/zero/unknown-base fallback in ci-jobs.test.sh. This makes the implementation item's proof agree with the strategy's cross-language testing promise.
