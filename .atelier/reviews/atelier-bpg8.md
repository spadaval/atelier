---
schema: atelier.review
schema_version: 2
id: atelier-bpg8
title: 'G9: Git-valid numeric diff.renames boolean forms (0x0, 0x1, 0k, 1k) are rejected by the new Rust integer fallback. Reproduced independently in /tmp/session-review-config-0w8ypfk1/repo; preserve Git configuration semantics before accepting the guard slice.'
target: atelier-1ioo
identity: session-reviewer
commit: 6f5188dd5bc990c88733a0027ee703eaaef9f737
created_at: 2026-10-02T00:37:03.518393881Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator
    commit: 5277b54c6b3a787a36bb927928ec1771218da0cb
    recorded_at: 2026-10-02T01:03:03.102458213Z
    resolution_commit: 019c18726c74f808d6d5847bd7fbf6cd3ba5928e
    reason: 'G9 independently verified: native Git boolean parsing preserves numeric, empty and case forms without accepting invalid whitespace.'
    migrated_from: atelier-ecn0
---

G9: Git-valid numeric diff.renames boolean forms (0x0, 0x1, 0k, 1k) are rejected by the new Rust integer fallback. Reproduced independently in /tmp/session-review-config-0w8ypfk1/repo; preserve Git configuration semantics before accepting the guard slice.
