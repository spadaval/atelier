---
schema: atelier.review
schema_version: 2
id: atelier-r7zq
title: 'crates/atelier-cli/src/commands/work.rs:435 — the new scope size in work parent counts the parent itself (ScopedWork::new inserts the objective at work.rs:505 for non-missions), so one scope gets two sizes in views the new Counts section says write alike. Fixture: a task parent with 2 children prints ''Work : 3 issues: 1 ready, 1 context, 1 backlog'' in work parent but ''Scope : 2 issues: 1 ready, 1 backlog'' in issue show --full. test_scoped_dashboard_counts_and_quiet_rows_exceed_display_limit pins ''25 issues'' for a parent with 24 children. Fix: leave the parent out of the parent view''s size and buckets (or name it apart), and assert the size matches issue show.'
target: atelier-krf4
identity: Independent reviewer (Claude subagent)
commit: 23527f8026ade3ebd70c6740ddb995c23d25fe87
created_at: 2026-10-17T07:38:20.498437913Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 4e8e51630991ab8cf13516ff566284c5e05eb760
    recorded_at: 2026-10-17T07:48:32.332501560Z
    resolution_commit: 5927bc001a2354c3e2a5e1c54f0953843e78bc94
    reason: work parent leaves the parent out of its size and state counts; a test matches issue show --full.
    migrated_from: atelier-gybk
---

crates/atelier-cli/src/commands/work.rs:435 — the new scope size in work parent counts the parent itself (ScopedWork::new inserts the objective at work.rs:505 for non-missions), so one scope gets two sizes in views the new Counts section says write alike. Fixture: a task parent with 2 children prints 'Work : 3 issues: 1 ready, 1 context, 1 backlog' in work parent but 'Scope : 2 issues: 1 ready, 1 backlog' in issue show --full. test_scoped_dashboard_counts_and_quiet_rows_exceed_display_limit pins '25 issues' for a parent with 24 children. Fix: leave the parent out of the parent view's size and buckets (or name it apart), and assert the size matches issue show.
