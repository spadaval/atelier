---
created_at: "2026-07-16T20:46:56.836789309+00:00"
id: "atelier-l3c8"
evidence_type: "validation"
captured_at: "2026-07-16T20:46:56.836779214+00:00"
agent_identity: "agent-factory.implement"
target:
  kind: "issue"
  id: "atelier-3v2d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3v2d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS: atomic independent mission-plan cutover, live invariants, rollback, idempotence, and full suite"
updated_at: "2026-07-16T20:46:56.839116162+00:00"
---

PASS: atomic mission-plan cutover migrated 42 missions: 41 terminal missions remained byte-identical and sole active mission atelier-p4z2 received one grandfather receipt. Clean base 4cd313db and the reconstructed post-start/pre-migration state both independently produced graph revision mission-graph-v2:sha256:3c9b41232de36e6dc5e50bc7670f4ddc0ddfa8c74aa5105e3c6861b844d609c0. The supplied historical 74d72298 revision did not reproduce; its cause is not asserted. The manifest and exact receipt bind 3c9b, and full tracker validation proves FreshGrandfather because the legacy start lacks bound approval metadata. A second migration invocation made no canonical changes. Individual SHA-256 comparisons covered every terminal mission record and activity sidecar with no differences. Five focused migration fixtures passed, including every legacy class, no-active omission, byte idempotence, collision and unknown-state refusal, and injected rollback. Full cargo nextest passed 747 of 747 tests. Recovery is to repair the named canonical file, run atelier check --fix, and retry migration; SQLite is never a migration source.
