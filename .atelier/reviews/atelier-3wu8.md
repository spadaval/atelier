---
schema: atelier.review
schema_version: 2
id: atelier-3wu8
title: 'low: crates/atelier-cli/src/commands/merge.rs:342 The MAX_ATTEMPTS exhaustion path (''kept moving across 3 attempts'') and a retry whose new base touches the item''s own records (refusal expected from base_records as merge_base_records, as reviews-and-merge.md states) have no test; the base_mover helper moves the base only once. Both paths are short and read correctly, but a regression there would go unnoticed. Fix: a mover that advances the base by a records-only commit on every run, asserting three runs and merge_snapshot_changed; and one whose records commit edits the owner''s record.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: f5733347e44e5cbc88ce25b5680ea8e91d92ddfc
created_at: 2026-10-22T20:52:28.119561176Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 6f7bffe48d815a32a546430881469d7c0bbf5791
    recorded_at: 2026-10-22T20:56:06.155624273Z
    resolution_commit: 2c18ab441b211bbe673db2fa821112da5c50d42e
    reason: Fixed in 2c18ab44
    migrated_from: atelier-g3ao
---

low: crates/atelier-cli/src/commands/merge.rs:342 The MAX_ATTEMPTS exhaustion path ('kept moving across 3 attempts') and a retry whose new base touches the item's own records (refusal expected from base_records as merge_base_records, as reviews-and-merge.md states) have no test; the base_mover helper moves the base only once. Both paths are short and read correctly, but a regression there would go unnoticed. Fix: a mover that advances the base by a records-only commit on every run, asserting three runs and merge_snapshot_changed; and one whose records commit edits the owner's record.
