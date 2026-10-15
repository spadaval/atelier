---
created_at: "2026-10-15T19:30:05.901995967+00:00"
id: "atelier-yu2w"
evidence_type: "validation"
commit: "8f13d015eda0d0d702e77bf318fb686565e62955"
result: "pass"
captured_at: "2026-10-15T19:30:05.901490453+00:00"
agent_identity: "independent-validator-2026-10-15"
target:
  kind: "issue"
  id: "atelier-xz11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xz11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S6 PASS (independent-validator-2026-10-15, candidate 8f13d015). Tests read and run (PASS): session::prime_and_active_landing_are_fresh_and_read_only, canonical_changes_refuse_landing_in_every_git_state, ignored_canonical_files_still_refuse_landing, done_owner_requires_approval_and_active_failures_can_handoff, documented_hook_events_use_nested_cwd_and_stop_loop_protection, setup_is_atomic_idempotent_and_preserves_other_handlers. Dogfood in this repo at 8f13d015 (clean checkout): atelier prime exit 0 with branch, owner atelier-1ioo, mission, strategy, work, reviews, evidence freshness; land refused with land_records_uncommitted after a one-byte edit to .atelier/issues/atelier-28mz.md, restored bytes verified by sha256, git status empty afterwards. Land currently refuses on the candidate itself: land_invariants_unmet atelier-8ud1 evidence.no_newer_failure atelier-3onc (8ud1 pass zqkt went stale at code commit fdf8fd68). In a clean clone at 76cde2a9 the same sequence gave land pass -> refuse -> pass. setup claude --binary in scratch /tmp/xz11/setuprepo: installs SessionStart 'prime --hook claude' and Stop 'land --hook claude', keeps a foreign Stop hook and other keys, idempotent (byte-identical rerun); land --hook blocks with decision=block on an uncommitted issue edit and on workflow.yaml. Notes: uncommitted .atelier/config.toml edits are not refused by land; setup leaves settings.json mode 0600."
updated_at: "2026-10-15T19:30:05.903446984+00:00"
---

S6 PASS (independent-validator-2026-10-15, candidate 8f13d015). Tests read and run (PASS): session::prime_and_active_landing_are_fresh_and_read_only, canonical_changes_refuse_landing_in_every_git_state, ignored_canonical_files_still_refuse_landing, done_owner_requires_approval_and_active_failures_can_handoff, documented_hook_events_use_nested_cwd_and_stop_loop_protection, setup_is_atomic_idempotent_and_preserves_other_handlers. Dogfood in this repo at 8f13d015 (clean checkout): atelier prime exit 0 with branch, owner atelier-1ioo, mission, strategy, work, reviews, evidence freshness; land refused with land_records_uncommitted after a one-byte edit to .atelier/issues/atelier-28mz.md, restored bytes verified by sha256, git status empty afterwards. Land currently refuses on the candidate itself: land_invariants_unmet atelier-8ud1 evidence.no_newer_failure atelier-3onc (8ud1 pass zqkt went stale at code commit fdf8fd68). In a clean clone at 76cde2a9 the same sequence gave land pass -> refuse -> pass. setup claude --binary in scratch /tmp/xz11/setuprepo: installs SessionStart 'prime --hook claude' and Stop 'land --hook claude', keeps a foreign Stop hook and other keys, idempotent (byte-identical rerun); land --hook blocks with decision=block on an uncommitted issue edit and on workflow.yaml. Notes: uncommitted .atelier/config.toml edits are not refused by land; setup leaves settings.json mode 0600.
