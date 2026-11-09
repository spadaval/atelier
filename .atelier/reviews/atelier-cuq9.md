---
schema: atelier.review
schema_version: 2
id: atelier-cuq9
title: finding on atelier-zpmb @c2642b25
target: atelier-zpmb
identity: gpt-6.1-sol-r1
commit: c2642b25f3d6053b00dfea9b623a50e9362dfb71
created_at: 2026-11-09T16:56:41.307568581Z
event:
  event: finding
  recommendation: fix_now
---

R2 medium: crates/atelier-site/src/lib.rs:215-219 caches every view Reply, including Git execution failures, until the base moves or the server restarts. The same problem affects failed OnceLock loads at lines 185-187 and failed CommitFacts at lines 254-260. Reproduced against a valid repository: first warm GET /api/repos (200), then inject exactly one failing git log, and GET the debrief twice. Both requests return the identical 500 although the one-shot failure is gone; an uncached brief returns 200. A transient process or IO failure is not determined by the base commit, so caching it violates the Done when cache constraint and prevents recovery on a healthy unchanged base. Cache successful results and deterministic commit-content errors only, or leave errors uncached throughout these three layers; add recovery tests that succeed on retry without advancing refs or restarting the server.
