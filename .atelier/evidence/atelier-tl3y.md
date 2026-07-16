---
created_at: "2026-07-06T17:54:30.492270228+00:00"
id: "atelier-tl3y"
evidence_type: "audit"
captured_at: "2026-07-06T17:54:15.907275314+00:00"
command: "sh -c 'set -eu\nprintf \"snapshot_utc=%s\\n\" \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"\nprintf \"commit=%s branch=%s\\n\" \"$(git rev-parse --short=12 HEAD)\" \"$(git branch --show-current)\"\nprintf \"canonical_issue_records=%s\\n\" \"$(find .atelier/issues -maxdepth 1 -type f -name \"*.md\" | wc -l)\"\nprintf \"canonical_evidence_records=%s\\n\" \"$(find .atelier/evidence -maxdepth 1 -type f -name \"*.md\" | wc -l)\"\nprintf \"mission_records=%s\\n\" \"$(rg --no-filename \"^issue_type:\" .atelier/issues/*.md | sed \"s/^issue_type: *//; s/\\\"//g\" | rg -c \"^mission$\")\"\nprintf \"epic_records=%s\\n\" \"$(rg --no-filename \"^issue_type:\" .atelier/issues/*.md | sed \"s/^issue_type: *//; s/\\\"//g\" | rg -c \"^epic$\")\"\nrg --no-filename \"^status:\" .atelier/issues/*.md | sed \"s/^status: *//; s/\\\"//g\" | sort | uniq -c\nprintf \"native_review_records=%s\\n\" \"$(find .atelier/reviews -maxdepth 1 -type f 2>/dev/null | wc -l)\"\nprintf \"activity_sidecar_dirs=%s\\n\" \"$(find .atelier/issues -mindepth 1 -maxdepth 1 -type d -name \"*.activity\" | wc -l)\"\nprintf \"activity_sidecar_files=%s\\n\" \"$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path \"*.activity/*.md\" | wc -l)\"\nprintf \"activity_sidecar_bytes=%s\\n\" \"$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path \"*.activity/*.md\" -printf \"%s\\n\" | awk \"{s+=\\$1} END{print s+0}\")\"\nprintf \"ignored_runtime_files=%s ignored_cache_files=%s\\n\" \"$(find .atelier/runtime -type f 2>/dev/null | wc -l)\" \"$(find .atelier/cache -type f 2>/dev/null | wc -l)\"\ndiag_root=${ATELIER_DIAGNOSTICS_DIR:-${ATELIER_HOME:-${XDG_STATE_HOME:-$HOME/.local/state}/atelier}/diagnostics}\nprintf \"diagnostic_logs=%s diagnostic_bytes=%s\\n\" \"$(find \"$diag_root/commands\" -maxdepth 1 -type f -name \"*.ndjson\" 2>/dev/null | wc -l)\" \"$(find \"$diag_root/commands\" -maxdepth 1 -type f -name \"*.ndjson\" -printf \"%s\\n\" 2>/dev/null | awk \"{s+=\\$1} END{print s+0}\")\"\nprintf \"local_branches=%s registered_worktrees=%s\\n\" \"$(git for-each-ref --format=\"%(refname:short)\" refs/heads | wc -l)\" \"$(git worktree list --porcelain | rg -c \"^worktree \")\"\nreport=$(target/debug/atelier prune)\nprintf \"default_diagnostics_candidates=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | sed -n \"/^Diagnostics Logs$/,/^Ignored Runtime/p\" | rg \"^  eligible diagnostics-log \" | wc -l)\"\nprintf \"local_protected_runtime=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg \"^  protected runtime-\" | wc -l)\"\nprintf \"git_eligible_branches=%s git_eligible_registrations=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg \"^  eligible branch \" | wc -l)\" \"$(printf \"%s\\n\" \"$report\" | rg \"^  eligible worktree-registration \" | wc -l)\"\nprintf \"git_protected_branches=%s git_protected_worktrees=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg \"^  protected branch \" | wc -l)\" \"$(printf \"%s\\n\" \"$report\" | rg \"^  protected worktree \" | wc -l)\"\nprintf \"canonical_issue_candidates=%s canonical_evidence_candidates=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg \"^  eligible issue \" | wc -l)\" \"$(printf \"%s\\n\" \"$report\" | rg \"^  eligible evidence-record \" | wc -l)\"\nprintf \"eligible_activity_sidecar_dirs=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg \"^    eligible activity-sidecars \" | wc -l)\"\nprintf \"%s\\n\" \"$report\" | rg \"^  eligible branch |^  eligible worktree-registration \"\necho classification_local_only=diagnostics_runtime_cache\n echo classification_git_cleanup=integrated_terminal_owner_branches_and_prunable_registrations\n echo classification_future_canonical=terminal_issues_evidence_and_activity_sidecars\n echo classification_protected=nonterminal_recent_proof_relevant_dirty_current_unmerged_unpushed_or_unowned\n echo dry_run_only=true'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-bd8j"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bd8j"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS: bounded artifact, Git, and dry-run prune inventory; no apply performed"
updated_at: "2026-07-06T17:54:37.095985396+00:00"
---

## Summary

PASS: bounded artifact, Git, and dry-run prune inventory; no apply performed

## Command

```console
sh -c 'set -eu
printf "snapshot_utc=%s\n" "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf "commit=%s branch=%s\n" "$(git rev-parse --short=12 HEAD)" "$(git branch --show-current)"
printf "canonical_issue_records=%s\n" "$(find .atelier/issues -maxdepth 1 -type f -name "*.md" | wc -l)"
printf "canonical_evidence_records=%s\n" "$(find .atelier/evidence -maxdepth 1 -type f -name "*.md" | wc -l)"
printf "mission_records=%s\n" "$(rg --no-filename "^issue_type:" .atelier/issues/*.md | sed "s/^issue_type: *//; s/\"//g" | rg -c "^mission$")"
printf "epic_records=%s\n" "$(rg --no-filename "^issue_type:" .atelier/issues/*.md | sed "s/^issue_type: *//; s/\"//g" | rg -c "^epic$")"
rg --no-filename "^status:" .atelier/issues/*.md | sed "s/^status: *//; s/\"//g" | sort | uniq -c
printf "native_review_records=%s\n" "$(find .atelier/reviews -maxdepth 1 -type f 2>/dev/null | wc -l)"
printf "activity_sidecar_dirs=%s\n" "$(find .atelier/issues -mindepth 1 -maxdepth 1 -type d -name "*.activity" | wc -l)"
printf "activity_sidecar_files=%s\n" "$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path "*.activity/*.md" | wc -l)"
printf "activity_sidecar_bytes=%s\n" "$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path "*.activity/*.md" -printf "%s\n" | awk "{s+=\$1} END{print s+0}")"
printf "ignored_runtime_files=%s ignored_cache_files=%s\n" "$(find .atelier/runtime -type f 2>/dev/null | wc -l)" "$(find .atelier/cache -type f 2>/dev/null | wc -l)"
diag_root=${ATELIER_DIAGNOSTICS_DIR:-${ATELIER_HOME:-${XDG_STATE_HOME:-$HOME/.local/state}/atelier}/diagnostics}
printf "diagnostic_logs=%s diagnostic_bytes=%s\n" "$(find "$diag_root/commands" -maxdepth 1 -type f -name "*.ndjson" 2>/dev/null | wc -l)" "$(find "$diag_root/commands" -maxdepth 1 -type f -name "*.ndjson" -printf "%s\n" 2>/dev/null | awk "{s+=\$1} END{print s+0}")"
printf "local_branches=%s registered_worktrees=%s\n" "$(git for-each-ref --format="%(refname:short)" refs/heads | wc -l)" "$(git worktree list --porcelain | rg -c "^worktree ")"
report=$(target/debug/atelier prune)
printf "default_diagnostics_candidates=%s\n" "$(printf "%s\n" "$report" | sed -n "/^Diagnostics Logs$/,/^Ignored Runtime/p" | rg "^  eligible diagnostics-log " | wc -l)"
printf "local_protected_runtime=%s\n" "$(printf "%s\n" "$report" | rg "^  protected runtime-" | wc -l)"
printf "git_eligible_branches=%s git_eligible_registrations=%s\n" "$(printf "%s\n" "$report" | rg "^  eligible branch " | wc -l)" "$(printf "%s\n" "$report" | rg "^  eligible worktree-registration " | wc -l)"
printf "git_protected_branches=%s git_protected_worktrees=%s\n" "$(printf "%s\n" "$report" | rg "^  protected branch " | wc -l)" "$(printf "%s\n" "$report" | rg "^  protected worktree " | wc -l)"
printf "canonical_issue_candidates=%s canonical_evidence_candidates=%s\n" "$(printf "%s\n" "$report" | rg "^  eligible issue " | wc -l)" "$(printf "%s\n" "$report" | rg "^  eligible evidence-record " | wc -l)"
printf "eligible_activity_sidecar_dirs=%s\n" "$(printf "%s\n" "$report" | rg "^    eligible activity-sidecars " | wc -l)"
printf "%s\n" "$report" | rg "^  eligible branch |^  eligible worktree-registration "
echo classification_local_only=diagnostics_runtime_cache
 echo classification_git_cleanup=integrated_terminal_owner_branches_and_prunable_registrations
 echo classification_future_canonical=terminal_issues_evidence_and_activity_sidecars
 echo classification_protected=nonterminal_recent_proof_relevant_dirty_current_unmerged_unpushed_or_unowned
 echo dry_run_only=true'
```

Exit status: 0

## Stdout

Bytes: 1264
Truncated: no

```text
snapshot_utc=2026-07-06T17:54:15Z
commit=4eb4d7ba42cc branch=epic/atelier-txf6
canonical_issue_records=926
canonical_evidence_records=781
mission_records=40
epic_records=184
     35 closed
    838 done
      2 in_progress
      1 publish_review
      2 ready
      1 superseded
     47 todo
native_review_records=0
activity_sidecar_dirs=780
activity_sidecar_files=4915
activity_sidecar_bytes=1813359
ignored_runtime_files=2 ignored_cache_files=0
diagnostic_logs=19 diagnostic_bytes=252504927
local_branches=125 registered_worktrees=21
default_diagnostics_candidates=0
local_protected_runtime=1
git_eligible_branches=2 git_eligible_registrations=1
git_protected_branches=123 git_protected_worktrees=20
canonical_issue_candidates=830 canonical_evidence_candidates=766
eligible_activity_sidecar_dirs=730
  eligible branch epic/atelier-li5h
  eligible branch epic/atelier-qu06
  eligible worktree-registration /tmp/atelier-jezn
classification_local_only=diagnostics_runtime_cache
classification_git_cleanup=integrated_terminal_owner_branches_and_prunable_registrations
classification_future_canonical=terminal_issues_evidence_and_activity_sidecars
classification_protected=nonterminal_recent_proof_relevant_dirty_current_unmerged_unpushed_or_unowned
dry_run_only=true
```

## Stderr

Bytes: 0
Truncated: no

```text
```

