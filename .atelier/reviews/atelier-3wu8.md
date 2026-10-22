---
schema: atelier.review
schema_version: 1
id: atelier-3wu8
title: 'low: crates/atelier-cli/src/commands/merge.rs:342 The MAX_ATTEMPTS exhaustion path (''kept moving across 3 attempts'') and a retry whose new base touches the item''s own records (refusal expected from base_records as merge_base_records, as reviews-and-merge.md states) have no test; the base_mover helper moves the base only once. Both paths are short and read correctly, but a regression there would go unnoticed. Fix: a mover that advances the base by a records-only commit on every run, asserting three runs and merge_snapshot_changed; and one whose records commit edits the owner''s record.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: f5733347e44e5cbc88ce25b5680ea8e91d92ddfc
created_at: 2026-10-22T20:52:28.119561176Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/merge.rs:342 The MAX_ATTEMPTS exhaustion path ('kept moving across 3 attempts') and a retry whose new base touches the item's own records (refusal expected from base_records as merge_base_records, as reviews-and-merge.md states) have no test; the base_mover helper moves the base only once. Both paths are short and read correctly, but a regression there would go unnoticed. Fix: a mover that advances the base by a records-only commit on every run, asserting three runs and merge_snapshot_changed; and one whose records commit edits the owner's record.
