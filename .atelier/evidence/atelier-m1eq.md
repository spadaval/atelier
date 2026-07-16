---
created_at: "2026-07-06T17:52:38.154553708+00:00"
id: "atelier-m1eq"
evidence_type: "audit"
captured_at: "2026-07-06T17:52:26.599004698+00:00"
command: "bash -lc 'set -euo pipefail\nprintf \"snapshot_utc=%s\\n\" \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"\nprintf \"commit=%s\\n\" \"$(git rev-parse --short=12 HEAD)\"\nprintf \"branch=%s\\n\" \"$(git branch --show-current)\"\nprintf \"canonical_issue_records=%s\\n\" \"$(find .atelier/issues -maxdepth 1 -type f -name \"*.md\" | wc -l)\"\nprintf \"mission_records=%s\\n\" \"$(rg --no-filename \"^issue_type:\" .atelier/issues/*.md | sed \"s/^issue_type: *//; s/\\\"//g\" | rg -c \"^mission$\")\"\nprintf \"epic_records=%s\\n\" \"$(rg --no-filename \"^issue_type:\" .atelier/issues/*.md | sed \"s/^issue_type: *//; s/\\\"//g\" | rg -c \"^epic$\")\"\nprintf \"issue_status_counts=%s\\n\" \"$(rg --no-filename \"^status:\" .atelier/issues/*.md | sed \"s/^status: *//; s/\\\"//g\" | sort | uniq -c | awk \"{printf \\\"%s:%s,\\\", \\\\$2, \\\\$1}\")\"\nprintf \"canonical_evidence_records=%s\\n\" \"$(find .atelier/evidence -maxdepth 1 -type f -name \"*.md\" | wc -l)\"\nprintf \"evidence_type_counts=%s\\n\" \"$(awk \"FNR==1{front=0} /^---$/{front++; next} front==1 && /^evidence_type:/{gsub(/\\\\\\\"/,\\\"\\\",\\\"\\\" \\\\$2); print \\\\$2}\" .atelier/evidence/*.md | sort | uniq -c | awk \"{printf \\\"%s:%s,\\\", \\\\$2, \\\\$1}\")\"\nprintf \"native_review_records=%s\\n\" \"$(find .atelier/reviews -maxdepth 1 -type f 2>/dev/null | wc -l)\"\nprintf \"activity_sidecar_dirs=%s\\n\" \"$(find .atelier/issues -mindepth 1 -maxdepth 1 -type d -name \"*.activity\" | wc -l)\"\nprintf \"activity_sidecar_files=%s\\n\" \"$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path \"*.activity/*.md\" | wc -l)\"\nprintf \"activity_sidecar_bytes=%s\\n\" \"$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path \"*.activity/*.md\" -printf \"%s\\n\" | awk \"{s+=\\\\$1} END{print s+0}\")\"\nprintf \"ignored_runtime_files=%s\\n\" \"$(find .atelier/runtime -type f 2>/dev/null | wc -l)\"\nprintf \"ignored_runtime_bytes=%s\\n\" \"$(find .atelier/runtime -type f -printf \"%s\\n\" 2>/dev/null | awk \"{s+=\\\\$1} END{print s+0}\")\"\nprintf \"ignored_cache_files=%s\\n\" \"$(find .atelier/cache -type f 2>/dev/null | wc -l)\"\ndiag_root=${ATELIER_DIAGNOSTICS_DIR:-${ATELIER_HOME:-${XDG_STATE_HOME:-$HOME/.local/state}/atelier}/diagnostics}\nprintf \"diagnostics_root=%s\\n\" \"$diag_root\"\nprintf \"diagnostic_logs=%s\\n\" \"$(find \"$diag_root/commands\" -maxdepth 1 -type f -name \"*.ndjson\" 2>/dev/null | wc -l)\"\nprintf \"diagnostic_bytes=%s\\n\" \"$(find \"$diag_root/commands\" -maxdepth 1 -type f -name \"*.ndjson\" -printf \"%s\\n\" 2>/dev/null | awk \"{s+=\\\\$1} END{print s+0}\")\"\nprintf \"local_branches=%s\\n\" \"$(git for-each-ref --format=\"%(refname:short)\" refs/heads | wc -l)\"\nprintf \"registered_worktrees=%s\\n\" \"$(git worktree list --porcelain | rg -c \"^worktree \")\"\nreport=$(target/debug/atelier prune)\nprintf \"%s\\n\" \"$report\" | awk \"/^Diagnostics Logs$/{section=\\\"diagnostics\\\"} /^Ignored Runtime, Cache, And Projection Artifacts$/{section=\\\"local\\\"} /^Git Branches And Worktrees$/{section=\\\"git\\\"} /^Canonical Records$/{section=\\\"canonical\\\"} /^Retention:|^Cutoff:|^Candidates:/{print section \\\"_\\\" \\\\$0} /^  eligible branch /{eb++} /^  eligible worktree-registration /{ew++} /^  protected branch /{pb++} /^  protected worktree /{pw++} /^  eligible issue /{ei++} /^  eligible evidence-record /{ee++} /^    eligible activity-sidecars /{ad++; n=\\\\$4; gsub(/[^0-9]/,\\\"\\\",n); af+=n} END{print \\\"git_eligible_branches=\\\" eb+0; print \\\"git_eligible_worktree_registrations=\\\" ew+0; print \\\"git_protected_branches=\\\" pb+0; print \\\"git_protected_worktrees=\\\" pw+0; print \\\"canonical_issue_candidates=\\\" ei+0; print \\\"canonical_evidence_candidates=\\\" ee+0; print \\\"eligible_activity_sidecar_dirs=\\\" ad+0; print \\\"eligible_activity_sidecar_files=\\\" af+0}\"\nprintf \"%s\\n\" \"$report\" | awk \"/^  eligible branch /{print \\\"eligible_branch=\\\" \\\\$3} /^  eligible worktree-registration /{print \\\"eligible_worktree_registration=\\\" \\\\$3}\"\necho classification_local_only=diagnostics/runtime/cache\n echo classification_git_only=integrated_terminal_owner_branches_and_prunable_registrations\n echo classification_future_canonical=terminal_issues_evidence_and_their_activity_sidecars\n echo classification_protected=nonterminal_recent_proof_relevant_dirty_current_unmerged_unpushed_or_unowned\n echo dry_run_only=true'"
exit_status: "1"
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
title: "Read-only retention inventory and prune dry-run classification"
updated_at: "2026-07-06T17:52:44.459937855+00:00"
---

## Summary

Read-only retention inventory and prune dry-run classification

## Command

```console
bash -lc 'set -euo pipefail
printf "snapshot_utc=%s\n" "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf "commit=%s\n" "$(git rev-parse --short=12 HEAD)"
printf "branch=%s\n" "$(git branch --show-current)"
printf "canonical_issue_records=%s\n" "$(find .atelier/issues -maxdepth 1 -type f -name "*.md" | wc -l)"
printf "mission_records=%s\n" "$(rg --no-filename "^issue_type:" .atelier/issues/*.md | sed "s/^issue_type: *//; s/\"//g" | rg -c "^mission$")"
printf "epic_records=%s\n" "$(rg --no-filename "^issue_type:" .atelier/issues/*.md | sed "s/^issue_type: *//; s/\"//g" | rg -c "^epic$")"
printf "issue_status_counts=%s\n" "$(rg --no-filename "^status:" .atelier/issues/*.md | sed "s/^status: *//; s/\"//g" | sort | uniq -c | awk "{printf \"%s:%s,\", \\$2, \\$1}")"
printf "canonical_evidence_records=%s\n" "$(find .atelier/evidence -maxdepth 1 -type f -name "*.md" | wc -l)"
printf "evidence_type_counts=%s\n" "$(awk "FNR==1{front=0} /^---$/{front++; next} front==1 && /^evidence_type:/{gsub(/\\\"/,\"\",\"\" \\$2); print \\$2}" .atelier/evidence/*.md | sort | uniq -c | awk "{printf \"%s:%s,\", \\$2, \\$1}")"
printf "native_review_records=%s\n" "$(find .atelier/reviews -maxdepth 1 -type f 2>/dev/null | wc -l)"
printf "activity_sidecar_dirs=%s\n" "$(find .atelier/issues -mindepth 1 -maxdepth 1 -type d -name "*.activity" | wc -l)"
printf "activity_sidecar_files=%s\n" "$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path "*.activity/*.md" | wc -l)"
printf "activity_sidecar_bytes=%s\n" "$(find .atelier/issues -mindepth 2 -maxdepth 2 -type f -path "*.activity/*.md" -printf "%s\n" | awk "{s+=\\$1} END{print s+0}")"
printf "ignored_runtime_files=%s\n" "$(find .atelier/runtime -type f 2>/dev/null | wc -l)"
printf "ignored_runtime_bytes=%s\n" "$(find .atelier/runtime -type f -printf "%s\n" 2>/dev/null | awk "{s+=\\$1} END{print s+0}")"
printf "ignored_cache_files=%s\n" "$(find .atelier/cache -type f 2>/dev/null | wc -l)"
diag_root=${ATELIER_DIAGNOSTICS_DIR:-${ATELIER_HOME:-${XDG_STATE_HOME:-$HOME/.local/state}/atelier}/diagnostics}
printf "diagnostics_root=%s\n" "$diag_root"
printf "diagnostic_logs=%s\n" "$(find "$diag_root/commands" -maxdepth 1 -type f -name "*.ndjson" 2>/dev/null | wc -l)"
printf "diagnostic_bytes=%s\n" "$(find "$diag_root/commands" -maxdepth 1 -type f -name "*.ndjson" -printf "%s\n" 2>/dev/null | awk "{s+=\\$1} END{print s+0}")"
printf "local_branches=%s\n" "$(git for-each-ref --format="%(refname:short)" refs/heads | wc -l)"
printf "registered_worktrees=%s\n" "$(git worktree list --porcelain | rg -c "^worktree ")"
report=$(target/debug/atelier prune)
printf "%s\n" "$report" | awk "/^Diagnostics Logs$/{section=\"diagnostics\"} /^Ignored Runtime, Cache, And Projection Artifacts$/{section=\"local\"} /^Git Branches And Worktrees$/{section=\"git\"} /^Canonical Records$/{section=\"canonical\"} /^Retention:|^Cutoff:|^Candidates:/{print section \"_\" \\$0} /^  eligible branch /{eb++} /^  eligible worktree-registration /{ew++} /^  protected branch /{pb++} /^  protected worktree /{pw++} /^  eligible issue /{ei++} /^  eligible evidence-record /{ee++} /^    eligible activity-sidecars /{ad++; n=\\$4; gsub(/[^0-9]/,\"\",n); af+=n} END{print \"git_eligible_branches=\" eb+0; print \"git_eligible_worktree_registrations=\" ew+0; print \"git_protected_branches=\" pb+0; print \"git_protected_worktrees=\" pw+0; print \"canonical_issue_candidates=\" ei+0; print \"canonical_evidence_candidates=\" ee+0; print \"eligible_activity_sidecar_dirs=\" ad+0; print \"eligible_activity_sidecar_files=\" af+0}"
printf "%s\n" "$report" | awk "/^  eligible branch /{print \"eligible_branch=\" \\$3} /^  eligible worktree-registration /{print \"eligible_worktree_registration=\" \\$3}"
echo classification_local_only=diagnostics/runtime/cache
 echo classification_git_only=integrated_terminal_owner_branches_and_prunable_registrations
 echo classification_future_canonical=terminal_issues_evidence_and_their_activity_sidecars
 echo classification_protected=nonterminal_recent_proof_relevant_dirty_current_unmerged_unpushed_or_unowned
 echo dry_run_only=true'
```

Exit status: 1

## Stdout

Bytes: 524
Truncated: no

```text
snapshot_utc=2026-07-06T17:52:26Z
commit=4eb4d7ba42cc
branch=epic/atelier-txf6
canonical_issue_records=926
mission_records=40
epic_records=184
issue_status_counts=
canonical_evidence_records=780
evidence_type_counts=
native_review_records=0
activity_sidecar_dirs=780
activity_sidecar_files=4914
activity_sidecar_bytes=
ignored_runtime_files=2
ignored_runtime_bytes=
ignored_cache_files=0
diagnostics_root=/root/.local/state/atelier/diagnostics
diagnostic_logs=19
diagnostic_bytes=
local_branches=125
registered_worktrees=21
```

## Stderr

Bytes: 251
Truncated: no

```text
bash: line 8: $2: unbound variable
bash: line 10: $2: unbound variable
bash: line 10: $2: unbound variable
bash: line 14: $1: unbound variable
bash: line 16: $1: unbound variable
bash: line 21: $1: unbound variable
bash: line 25: $4: unbound variable
```

