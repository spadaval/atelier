---
schema: atelier.review
schema_version: 2
id: atelier-5pk7
title: finding on atelier-ygul @64b292b1
target: atelier-ygul
identity: codex-reviewer (gpt-6.1-sol)
commit: 64b292b1a01e130538784b91e09c8ff9c30c1ea2
created_at: 2026-11-09T18:14:47.398051499Z
event:
  event: finding
  recommendation: fix_now
---

high: crates/atelier-app/src/review_gate.rs:1836 R1: Resolve strategy ownership from history, rather than the current RecordIndex. The exemption is recomputed from present mission records for every historical edge, so record-only changes can both resurrect a stale approval and invalidate an accepted one. Reproduced with this branch binary: create a mission and linked validation task, record a planning fix_now finding, fix and approve it, edit its own mission strategy on main, and mark the task done. Land correctly refuses, but issue unlink <item> <mission> --role advances followed by a record-only commit makes land pass without a new approval. Conversely, in the passing other-mission scenario, complete that other mission with an old closed_at, run prune --apply --retention-days 0, and commit the removed record: land now refuses the retained task for the previously excused strategy commit. Scratch repros are $HOME/.cache/ygul-review-tmp/strategy-unlink-3h0dk8cb and strategy-retirement-i9prjvo4. Classify strategy paths and mission scope using committed history at the relevant change, preserving relatedness across later record edits and preserving unrelated planning across pruning. Add regressions for both record-only transitions.
