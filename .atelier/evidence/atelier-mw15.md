---
created_at: "2026-07-06T17:55:35.565240753+00:00"
id: "atelier-mw15"
evidence_type: "audit"
captured_at: "2026-07-06T17:55:14.953611477+00:00"
command: "sh -c 'set -eu\nreport=$(target/debug/atelier prune --retention-days 7)\ndiag=$(printf \"%s\\n\" \"$report\" | sed -n \"/^Diagnostics Logs$/,/^Ignored Runtime/p\")\nprintf \"diagnostics_7d_candidates=%s diagnostics_7d_bytes=%s\\n\" \"$(printf \"%s\\n\" \"$diag\" | rg \"^  eligible diagnostics-log \" | wc -l)\" \"$(printf \"%s\\n\" \"$diag\" | sed -n \"s/.*(date [^,]*, \\([0-9][0-9]*\\) bytes).*/\\1/p\" | awk \"{s+=\\$1} END{print s+0}\")\"\nprintf \"branch_protected_no_owner=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected branch .*no terminal owner record association\" || true)\"\nprintf \"branch_protected_unmerged=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected branch .*contains commits not integrated\" || true)\"\nprintf \"branch_protected_unpushed=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected branch .*has no upstream\" || true)\"\nprintf \"branch_protected_dirty=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected branch .*dirty worktree\" || true)\"\nprintf \"branch_protected_current=%s branch_protected_base=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected branch .*current checkout\" || true)\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected branch .*configured base branch\" || true)\"\nprintf \"worktree_protected_dirty=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected worktree .*dirty worktree\" || true)\"\nprintf \"worktree_protected_no_owner=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected worktree .*no terminal owner record association\" || true)\"\nprintf \"worktree_protected_detached=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected worktree .*detached worktree\" || true)\"\nprintf \"worktree_protected_unmerged=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected worktree .*branch commits are not integrated\" || true)\"\nprintf \"worktree_protected_current=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | rg -c \"^  protected worktree .*current checkout\" || true)\"\nprintf \"eligible_activity_sidecar_files=%s\\n\" \"$(printf \"%s\\n\" \"$report\" | sed -n \"s/.*eligible activity-sidecars.*(\\([0-9][0-9]*\\) file.*/\\1/p\" | awk \"{s+=\\$1} END{print s+0}\")\"\necho apply_invoked=false'"
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
title: "PASS: protected Git reasons and explicit seven-day local diagnostics inventory"
updated_at: "2026-07-06T17:55:44.771104770+00:00"
---

## Summary

PASS: protected Git reasons and explicit seven-day local diagnostics inventory

## Command

```console
sh -c 'set -eu
report=$(target/debug/atelier prune --retention-days 7)
diag=$(printf "%s\n" "$report" | sed -n "/^Diagnostics Logs$/,/^Ignored Runtime/p")
printf "diagnostics_7d_candidates=%s diagnostics_7d_bytes=%s\n" "$(printf "%s\n" "$diag" | rg "^  eligible diagnostics-log " | wc -l)" "$(printf "%s\n" "$diag" | sed -n "s/.*(date [^,]*, \([0-9][0-9]*\) bytes).*/\1/p" | awk "{s+=\$1} END{print s+0}")"
printf "branch_protected_no_owner=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected branch .*no terminal owner record association" || true)"
printf "branch_protected_unmerged=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected branch .*contains commits not integrated" || true)"
printf "branch_protected_unpushed=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected branch .*has no upstream" || true)"
printf "branch_protected_dirty=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected branch .*dirty worktree" || true)"
printf "branch_protected_current=%s branch_protected_base=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected branch .*current checkout" || true)" "$(printf "%s\n" "$report" | rg -c "^  protected branch .*configured base branch" || true)"
printf "worktree_protected_dirty=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected worktree .*dirty worktree" || true)"
printf "worktree_protected_no_owner=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected worktree .*no terminal owner record association" || true)"
printf "worktree_protected_detached=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected worktree .*detached worktree" || true)"
printf "worktree_protected_unmerged=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected worktree .*branch commits are not integrated" || true)"
printf "worktree_protected_current=%s\n" "$(printf "%s\n" "$report" | rg -c "^  protected worktree .*current checkout" || true)"
printf "eligible_activity_sidecar_files=%s\n" "$(printf "%s\n" "$report" | sed -n "s/.*eligible activity-sidecars.*(\([0-9][0-9]*\) file.*/\1/p" | awk "{s+=\$1} END{print s+0}")"
echo apply_invoked=false'
```

Exit status: 0

## Stdout

Bytes: 425
Truncated: no

```text
diagnostics_7d_candidates=15 diagnostics_7d_bytes=216949081
branch_protected_no_owner=44
branch_protected_unmerged=61
branch_protected_unpushed=9
branch_protected_dirty=7
branch_protected_current=1 branch_protected_base=1
worktree_protected_dirty=8
worktree_protected_no_owner=5
worktree_protected_detached=3
worktree_protected_unmerged=3
worktree_protected_current=1
eligible_activity_sidecar_files=4427
apply_invoked=false
```

## Stderr

Bytes: 0
Truncated: no

```text
```

