---
schema: atelier.review
schema_version: 2
id: atelier-tzvy
title: finding on atelier-4nsl @23dae74d
target: atelier-4nsl
identity: independent reviewer subagent (claude-opus-5-5), atelier-4nsl
commit: 23dae74d3e56985d9c442e71ce68c29a2dc542b4
created_at: 2026-11-07T11:16:56.387356393Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/src/commands/prune.rs:710 protect_referenced_candidates reads only index.reviews(), and RecordIndex::record_links is built without branch reviews, so a deferred finding about a branch does not protect its follow-up issue. Once that follow-up is terminal and past retention, prune's dry run lists it as eligible and every 'prune --apply' refuses as a whole (record_invalid: missing follow-up), so nothing is pruned. Reproduced in a scratch repo: defer finding on a light branch with a follow-up, merge, supersede and age the follow-up, prune --apply refuses. Branch records are also never prune candidates (review_candidates at :911), unlike issue reviews. Fix: add branch reviews' declared references to the edges (and decide whether they are candidates), with a prune test.
