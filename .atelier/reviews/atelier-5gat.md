---
schema: atelier.review
schema_version: 1
id: atelier-5gat
title: 'low: crates/atelier-app/src/review_gate.rs:798: unwrap_or_else swallows every original_independent error, not just missing history: a ''review_append_only: ... differs from its original event'' bail, a git read failure, or require_issue/scope failures now surface as ''could not be traced to the commit that introduced it: <err>''. Still fails closed (gate fails, never accepted) and the full error text is embedded, and append_only_history aborts separately for rewritten records, so nothing real is hidden. Only the label is imprecise; no change needed.'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: 2c839460af8fe7d9b811edfc3da3c62945008b69
created_at: 2026-10-22T20:55:37.765488240Z
event:
  event: finding
  recommendation: no_action
---

low: crates/atelier-app/src/review_gate.rs:798: unwrap_or_else swallows every original_independent error, not just missing history: a 'review_append_only: ... differs from its original event' bail, a git read failure, or require_issue/scope failures now surface as 'could not be traced to the commit that introduced it: <err>'. Still fails closed (gate fails, never accepted) and the full error text is embedded, and append_only_history aborts separately for rewritten records, so nothing real is hidden. Only the label is imprecise; no change needed.
