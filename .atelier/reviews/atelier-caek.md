---
schema: atelier.review
schema_version: 2
id: atelier-caek
title: 'crates/atelier-app/src/record_index.rs:89 — a malformed record renders as ''Error: Invalid record file .atelier/issues/X.md: Invalid YAML front matter in .atelier/issues/X.md: ...'' from issue show, status and every loader, giving the path and the failure twice, against ''states its failure once''. This predates the change and is outside the Goal''s enumerated instances; it is a candidate for the shared error work.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:43.405182023Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 32365e408530d872d6c2b830c60032e79771c4ce
    recorded_at: 2026-10-17T08:29:29.661670538Z
    follow_up: atelier-x68o
    reason: Pre-existing doubled path; atelier-x68o covers it.
    migrated_from: atelier-b5tg
---

crates/atelier-app/src/record_index.rs:89 — a malformed record renders as 'Error: Invalid record file .atelier/issues/X.md: Invalid YAML front matter in .atelier/issues/X.md: ...' from issue show, status and every loader, giving the path and the failure twice, against 'states its failure once'. This predates the change and is outside the Goal's enumerated instances; it is a candidate for the shared error work.
