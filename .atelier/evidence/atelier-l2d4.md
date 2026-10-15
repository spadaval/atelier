---
created_at: "2026-10-15T19:29:30.550516613+00:00"
id: "atelier-l2d4"
evidence_type: "validation"
commit: "8f13d015eda0d0d702e77bf318fb686565e62955"
result: "pass"
captured_at: "2026-10-15T19:29:30.550178943+00:00"
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
title: "S1 PASS (validator independent-validator-2026-10-15, candidate 8f13d015, git 2.53.0). Tests read and run: crates/atelier-cli/tests/merge_gates.rs (missing_self_approval_and_git_email_aliases_are_refused, findings_require_dispositions_and_author_cannot_clear_fix_now, fixed_finding_requires_a_resolution_commit_and_current_independent_approval, trivial_exemption_is_explicit_visible_and_pinned, scoped_children_and_open_transitive_blockers_prevent_merge, code_policy_acceptance_and_notes_changes_require_fresh_approval) all PASS in cargo nextest run --features test-hooks (592/592, 0 skipped) and cargo test --no-fail-fast (588 passed, 0 failed, 0 ignored), run on a git-archive export of 8f13d015 (and earlier 76cde2a9). Independent CLI scenario /tmp/xz11/scen.sh (epic+child, non-trivial f.rs): merge --check refused with review.current for no approval and for self-approval by 'Author <author@example.test>'; findings.disposed for an undisposed fix_now finding and after an author-only no_action disposition; blockers.none_open + blockers.transitive_none_open for an open blocked_by link; passed only after independent approval and reviewer disposition."
updated_at: "2026-10-15T19:29:30.551870900+00:00"
---

S1 PASS (validator independent-validator-2026-10-15, candidate 8f13d015, git 2.53.0). Tests read and run: crates/atelier-cli/tests/merge_gates.rs (missing_self_approval_and_git_email_aliases_are_refused, findings_require_dispositions_and_author_cannot_clear_fix_now, fixed_finding_requires_a_resolution_commit_and_current_independent_approval, trivial_exemption_is_explicit_visible_and_pinned, scoped_children_and_open_transitive_blockers_prevent_merge, code_policy_acceptance_and_notes_changes_require_fresh_approval) all PASS in cargo nextest run --features test-hooks (592/592, 0 skipped) and cargo test --no-fail-fast (588 passed, 0 failed, 0 ignored), run on a git-archive export of 8f13d015 (and earlier 76cde2a9). Independent CLI scenario /tmp/xz11/scen.sh (epic+child, non-trivial f.rs): merge --check refused with review.current for no approval and for self-approval by 'Author <author@example.test>'; findings.disposed for an undisposed fix_now finding and after an author-only no_action disposition; blockers.none_open + blockers.transitive_none_open for an open blocked_by link; passed only after independent approval and reviewer disposition.
