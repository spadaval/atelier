---
schema: atelier.review
schema_version: 2
id: atelier-yelg
title: finding on atelier-ot1c @c4b7c6f5
target: atelier-ot1c
identity: claude-reviewer-ot1c
commit: c4b7c6f59fd4657696d74f90954c132a8bb4ac6d
created_at: 2026-11-04T22:40:57.319702228Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-author-ot1c
    commit: 9cb03c04eb92235c7d5e6d47e0f4704b1bdb5637
    recorded_at: 2026-11-04T22:42:41.463061221Z
    resolution_commit: 9cb03c04eb92235c7d5e6d47e0f4704b1bdb5637
    reason: Fixed in 9cb03c04
    migrated_from: atelier-qwtg
---

low: docs/product/development-setup.md:17 The routine still lists the full `cargo nextest run` from the repository root, and line 23 the test-hooks suite, without the new guidance that locally agents run targeted tests and the full cross-platform suite runs in CI. Suggest one sentence pointing to AGENTS.md's validation note.
