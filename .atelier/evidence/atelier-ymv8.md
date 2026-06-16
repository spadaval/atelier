---
created_at: "2026-06-16T17:24:26.694268002+00:00"
id: "atelier-ymv8"
evidence_type: "validation"
captured_at: "2026-06-16T17:24:26.694148367+00:00"
command: null
exit_status: null
path: "/tmp/atelier-0nv2-validation-20260616T132306/transcript.md"
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-0nv2"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0nv2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Independent validation PASS for automatic branch lifecycle and close integration. Artifact transcript: /tmp/atelier-0nv2-validation-20260616T132306/transcript.md; docs/help search: /tmp/atelier-0nv2-validation-20260616T132306/docs-help-search.md. Setup: disposable git repos initialized with target/debug/atelier init; default scenarios used base main and squash; alternate scenario used base trunk, merge_commit, epic template review/{{ issue.id }}, issue template work/{{ issue.type }}/{{ issue.id }}. Outcome 1 PASS: atelier start prepared child issue on epic/<epic-id>, standalone issue on codex/<issue-id>, and epic work on epic/<epic-id>; transcripts include git branch --show-current, issue show, status, transition --options, and git log. Outcome 2 PASS: child close under an epic committed terminal tracker state on the epic branch, stayed on that branch, printed Merge result deferred to epic close, and main_before equaled main_after. Outcome 3 PASS: standalone close and epic close committed terminal tracker state and squash-merged to base; current branch returned to main and git log showed Squash merge codex/<id> into main and Squash merge epic/<id> into main. Outcome 4 PASS: alternate configured base and merge strategy were honored; start used work/task/<id> from trunk, close printed Base branch trunk, Merge strategy merge_commit, and git log showed a merge commit on trunk. Outcome 5 PASS: induced squash merge conflict exited nonzero with recovery guidance, rolled back the close commit, issue show remained Status in_progress, status reported clean/current, and transition --options still allowed close with --reason. Outcome 6 PASS: docs/help/Agent Factory search found normal workflow guidance teaches atelier start/issue close; explicit branch helpers appear only as advanced diagnostic/repair/internal surfaces. Checks PASS: target/debug/atelier lint; target/debug/atelier export --check; cargo fmt -- --check; cargo test -p atelier-cli --test cli_integration test_start_prepares_child_standalone_and_epic_owner_branches_before_transition; test_child_issue_close_commits_on_epic_branch_without_base_merge; test_standalone_issue_close_squash_merges_to_base; test_epic_close_squash_merges_to_base_after_child_proof; test_issue_close_merge_failure_rolls_back_terminal_tracker_state; cargo test -p atelier-app configured_branch_lifecycle_resolution_surfaces_base_strategy_and_templates. Follow-ups: none."
updated_at: "2026-06-16T17:24:30.336922393+00:00"
---

Independent validation PASS for automatic branch lifecycle and close integration. Artifact transcript: /tmp/atelier-0nv2-validation-20260616T132306/transcript.md; docs/help search: /tmp/atelier-0nv2-validation-20260616T132306/docs-help-search.md. Setup: disposable git repos initialized with target/debug/atelier init; default scenarios used base main and squash; alternate scenario used base trunk, merge_commit, epic template review/{{ issue.id }}, issue template work/{{ issue.type }}/{{ issue.id }}. Outcome 1 PASS: atelier start prepared child issue on epic/<epic-id>, standalone issue on codex/<issue-id>, and epic work on epic/<epic-id>; transcripts include git branch --show-current, issue show, status, transition --options, and git log. Outcome 2 PASS: child close under an epic committed terminal tracker state on the epic branch, stayed on that branch, printed Merge result deferred to epic close, and main_before equaled main_after. Outcome 3 PASS: standalone close and epic close committed terminal tracker state and squash-merged to base; current branch returned to main and git log showed Squash merge codex/<id> into main and Squash merge epic/<id> into main. Outcome 4 PASS: alternate configured base and merge strategy were honored; start used work/task/<id> from trunk, close printed Base branch trunk, Merge strategy merge_commit, and git log showed a merge commit on trunk. Outcome 5 PASS: induced squash merge conflict exited nonzero with recovery guidance, rolled back the close commit, issue show remained Status in_progress, status reported clean/current, and transition --options still allowed close with --reason. Outcome 6 PASS: docs/help/Agent Factory search found normal workflow guidance teaches atelier start/issue close; explicit branch helpers appear only as advanced diagnostic/repair/internal surfaces. Checks PASS: target/debug/atelier lint; target/debug/atelier export --check; cargo fmt -- --check; cargo test -p atelier-cli --test cli_integration test_start_prepares_child_standalone_and_epic_owner_branches_before_transition; test_child_issue_close_commits_on_epic_branch_without_base_merge; test_standalone_issue_close_squash_merges_to_base; test_epic_close_squash_merges_to_base_after_child_proof; test_issue_close_merge_failure_rolls_back_terminal_tracker_state; cargo test -p atelier-app configured_branch_lifecycle_resolution_surfaces_base_strategy_and_templates. Follow-ups: none.
