---
created_at: "2026-06-15T16:06:37.575751842+00:00"
id: "atelier-4z86"
evidence_type: "validation"
captured_at: "2026-06-15T16:06:37.575704398+00:00"
target:
  kind: "issue"
  id: "atelier-t35w"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t35w"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent active issue and claim removal validation passed. Runtime transcript: fresh repo created and started an issue; deleting .atelier/runtime and .atelier/cache caused atelier status to rebuild runtime projection and still print Current work: 1 issue from canonical Markdown; explicit atelier rebuild preserved the same current-work output; root repair and abandon rejected as unrecognized subcommands; issue update --claim rejected as unexpected argument; evidence-backed issue close succeeded after proof. Source search found no root Commands::Abandon/Repair dispatch, repair_active, abandon_work_association, record_work_abandoned, finish_active_association, or issue claim flag/mutation path. Focused tests passed: test_status_recovers_when_runtime_directory_is_missing, test_root_active_pointer_cleanup_commands_are_removed, test_root_start_allows_multiple_current_work_issues_in_same_worktree, test_issue_help_uses_reduced_lifecycle_surface, test_non_lifecycle_issue_flows_use_explicit_homes, and test_command_result_json_mode_is_rejected_and_human_subset_works. cargo fmt -- --check, git diff --check, atelier lint atelier-t35w, and atelier export --check passed."
updated_at: "2026-06-15T16:06:39.753164210+00:00"
---

Independent active issue and claim removal validation passed. Runtime transcript: fresh repo created and started an issue; deleting .atelier/runtime and .atelier/cache caused atelier status to rebuild runtime projection and still print Current work: 1 issue from canonical Markdown; explicit atelier rebuild preserved the same current-work output; root repair and abandon rejected as unrecognized subcommands; issue update --claim rejected as unexpected argument; evidence-backed issue close succeeded after proof. Source search found no root Commands::Abandon/Repair dispatch, repair_active, abandon_work_association, record_work_abandoned, finish_active_association, or issue claim flag/mutation path. Focused tests passed: test_status_recovers_when_runtime_directory_is_missing, test_root_active_pointer_cleanup_commands_are_removed, test_root_start_allows_multiple_current_work_issues_in_same_worktree, test_issue_help_uses_reduced_lifecycle_surface, test_non_lifecycle_issue_flows_use_explicit_homes, and test_command_result_json_mode_is_rejected_and_human_subset_works. cargo fmt -- --check, git diff --check, atelier lint atelier-t35w, and atelier export --check passed.
