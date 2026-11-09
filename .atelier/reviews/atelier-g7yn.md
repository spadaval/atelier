---
schema: atelier.review
schema_version: 2
id: atelier-g7yn
title: finding on atelier-yn69 @a9cd1265
target: atelier-yn69
identity: grok-4.6
commit: a9cd1265fafcb1a44917fc15c10033f3120137b2
created_at: 2026-11-09T14:42:00.707467161Z
event:
  event: finding
  recommendation: fix_now
---

zpmb and yn69 disagree on who owns crates/atelier-site/web/. atelier-zpmb generates TypeScript types into web/src/api/ and fixture JSON into web/src/fixtures/, both checked in, with a Rust test that fails when they are stale. atelier-yn69 then creates the Vite app in that same web/ directory and never says those generated files already exist and must be kept. A Vite scaffold or a clean web/ tree would delete or overwrite the API contract zpmb just landed, so the first items do not agree on generated types and fixture JSON. State that yn69 scaffolds around the existing api/ and fixtures/ trees, or have zpmb write them outside the app tree and yn69 import them.
