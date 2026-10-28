---
schema: atelier.review
schema_version: 1
id: atelier-bi6e
title: finding on atelier-7v9q @575cf9d0
target: atelier-7v9q
identity: Independent reviewer (Claude subagent)
commit: 575cf9d0abbb3682dda04458261b4e1ced61d016
created_at: 2026-10-28T01:52:31.754704925Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/build.rs:5-6 — rerun-if-changed points at crates/.git (nonexistent), so local builds may stamp a stale commit. Pre-existing; CI/tag builds are fresh and unaffected.
