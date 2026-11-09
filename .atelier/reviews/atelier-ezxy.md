---
schema: atelier.review
schema_version: 2
id: atelier-ezxy
title: finding on atelier-ygul @3d3d251c
target: atelier-ygul
identity: codex-reviewer (gpt-6.1-sol)
commit: 3d3d251cafb07a7f10e369cf20b8108039c63f8a
created_at: 2026-11-09T18:34:11.778328979Z
event:
  event: finding
  recommendation: fix_now
---

high: crates/atelier-app/src/review_gate.rs:1874 R2: Include the approved parent scope when deciding whether a mission strategy is unrelated. A child can advance a mission without its parent advancing it. Record a planning fix_now finding on that child, fix the plan, approve the parent (which legitimately covers the child), then edit that mission strategy directly on the base and mark the child done. land now passes, both for the new claim and after a later note-only commit, although this is the done item own mission strategy changed after its approval, which Done when says must remain stale. The installed binary refuses the same history. check passes on the repro. within contains the child, but this predicate checks only whether the mission itself is within, or whether the mission contains the parent target; it misses their shared child. base_stale_edge also always classifies against approval.target even when original_approval_valid is checking a child finding. Treat overlapping scopes as related, or classify freshness against the finding/claim actually being covered on every global path. Keep the historical graph lookup and add new-claim and historical regressions for parent approvals covering mission-linked children. Repro: $HOME/.cache/ygul-review-tmp/strategy-parent-c6_z6xhh.
