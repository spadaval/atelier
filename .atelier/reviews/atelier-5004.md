---
schema: atelier.review
schema_version: 2
id: atelier-5004
title: finding on atelier-yn69 @a9cd1265
target: atelier-yn69
identity: gpt-6.1-sol
commit: a9cd1265fafcb1a44917fc15c10033f3120137b2
created_at: 2026-11-09T14:38:20.021931637Z
event:
  event: finding
  recommendation: fix_now
---

Medium: The unconditional SPA fallback also returns index.html for the built app's JavaScript and CSS.
Locations: .atelier/issues/atelier-yn69.md:32; docs/strategies/fleet-dashboard.md:131-135.
Both texts require every non-/api/ path to answer with index.html. Vite's built entry page loads JavaScript and CSS at non-API asset URLs. Applying the specified rule to those requests sends HTML instead of the embedded asset, so the browser cannot execute the app even though / and deep links return 200. The plan names rust-embed but never reserves asset paths or states their routing precedence.
Recommendation: FIX NOW. Define embedded asset serving, with the appropriate content type, before the index fallback; limit the fallback to client navigation routes and say how missing assets fail. Make the shipping proof load the built app with its actual asset URLs, including a deep link, and assert that a missing JavaScript asset is not answered with index.html. This is a small acceptance-contract correction, not a request to change the embedding library.
