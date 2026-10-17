---
schema: atelier.review
schema_version: 1
id: atelier-jqb7
title: 'crates/atelier-cli/src/commands/bundle.rs:1347 with docs/spec/bundle/schema.md:41-44 — the edited schema says bundle IDs fail with the same wrong-kind errors as command arguments, but an evidence ID in an issue''s blocked_by/blocks resolves as evidence: preview reports ''Bundle preview is valid'' with ''evidence/lr8o-try-p3by -> issue/c (blocks)'', and apply then fails with ''blocks relationship endpoints must both be issues'' (nothing installed). Pre-existing for full evidence IDs; this change also makes it reachable by short IDs. parent and advances already check the kind. Suggested: check blocks/blocked_by endpoints during validation and report the wrong-kind message.'
target: atelier-lr8o
identity: Independent reviewer (Claude subagent)
commit: 647d6b38852f63a1397ce2446734ab241bc1ef76
created_at: 2026-10-17T09:05:51.571320690Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/bundle.rs:1347 with docs/spec/bundle/schema.md:41-44 — the edited schema says bundle IDs fail with the same wrong-kind errors as command arguments, but an evidence ID in an issue's blocked_by/blocks resolves as evidence: preview reports 'Bundle preview is valid' with 'evidence/lr8o-try-p3by -> issue/c (blocks)', and apply then fails with 'blocks relationship endpoints must both be issues' (nothing installed). Pre-existing for full evidence IDs; this change also makes it reachable by short IDs. parent and advances already check the kind. Suggested: check blocks/blocked_by endpoints during validation and report the wrong-kind message.
