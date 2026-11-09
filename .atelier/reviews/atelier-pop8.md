---
schema: atelier.review
schema_version: 2
id: atelier-pop8
title: finding on atelier-ah82 @661625da
target: atelier-ah82
identity: codex-reviewer (gpt-6.1-sol)
commit: 661625dad956b33c3967870c050ec88418166ebc
created_at: 2026-11-09T02:56:23.464647704Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/product/reviews-and-merge.md:707 R3: Recovery after merge_push_base_rejected pulls from forge.remote but finishes with atelier land --push, which pushes the base to its configured upstream, not forge.remote (crates/atelier-cli/src/commands/session.rs:806-831). With the explicitly supported SSH URL override and an HTTPS origin lacking credentials, recovery pushes to the wrong transport and fails; a different upstream can publish to the wrong repository. The earlier recovery steps at lines 679 and 693 also assume <remote>/<branch> refs exist, which is invalid for URL remotes, and step 2 has not fetched a rejected remote branch. Specify recovery that fetches and merges the actual configured remote head and publishes to that same forge.remote after the land judgment, for both named remotes and URLs, without relying on an existing upstream.
