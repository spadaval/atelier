---
created_at: "2026-06-14T17:24:43.416052656+00:00"
id: "atelier-qhdb"
evidence_type: "validation"
captured_at: "2026-06-14T17:24:43.415917246+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-tqjn"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tqjn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Focused regression proof: transition options stay read-only, tracker-generated closeout bookkeeping does not self-block, hand-edited tracker Markdown still blocks, and dirty non-tracker work still blocks. Commands passed: cargo test --test cli_integration test_issue_transition_options_do_not_write_but_blocked_transitions_do -- --nocapture; cargo test --test cli_integration test_mission_close_ignores_tracker_generated_issue_closeout_bookkeeping -- --nocapture; cargo test --test cli_integration test_mission_close_still_blocks_hand_edited_issue_markdown -- --nocapture; cargo test --test cli_integration test_dirty_worktree_blocks_mission_closeout -- --nocapture; git diff --check; target/debug/atelier lint."
updated_at: "2026-06-14T17:24:46.076778834+00:00"
---

Focused regression proof: transition options stay read-only, tracker-generated closeout bookkeeping does not self-block, hand-edited tracker Markdown still blocks, and dirty non-tracker work still blocks. Commands passed: cargo test --test cli_integration test_issue_transition_options_do_not_write_but_blocked_transitions_do -- --nocapture; cargo test --test cli_integration test_mission_close_ignores_tracker_generated_issue_closeout_bookkeeping -- --nocapture; cargo test --test cli_integration test_mission_close_still_blocks_hand_edited_issue_markdown -- --nocapture; cargo test --test cli_integration test_dirty_worktree_blocks_mission_closeout -- --nocapture; git diff --check; target/debug/atelier lint.
