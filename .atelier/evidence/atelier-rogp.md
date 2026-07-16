---
created_at: "2026-07-06T17:56:30.566723523+00:00"
id: "atelier-rogp"
evidence_type: "audit"
captured_at: "2026-07-06T17:56:20.391802358+00:00"
command: "sh -c 'set -eu\nout=$(target/debug/atelier prune --quiet)\nprintf \"quiet_bytes=%s quiet_lines=%s\\n\" \"$(printf \"%s\\n\" \"$out\" | wc -c)\" \"$(printf \"%s\\n\" \"$out\" | wc -l)\"\nprintf \"quiet_record_detail_lines=%s\\n\" \"$(printf \"%s\\n\" \"$out\" | rg \"^  eligible issue |^  eligible evidence-record |^  protected branch |^  protected worktree \" | wc -l)\"\nprintf \"%s\\n\" \"$out\" | sed -n \"1,18p\"\necho apply_invoked=false'"
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
title: "FINDING: prune quiet mode still emits record-level dry-run detail"
updated_at: "2026-07-06T17:56:37.007081818+00:00"
---

## Summary

FINDING: prune quiet mode still emits record-level dry-run detail

## Command

```console
sh -c 'set -eu
out=$(target/debug/atelier prune --quiet)
printf "quiet_bytes=%s quiet_lines=%s\n" "$(printf "%s\n" "$out" | wc -c)" "$(printf "%s\n" "$out" | wc -l)"
printf "quiet_record_detail_lines=%s\n" "$(printf "%s\n" "$out" | rg "^  eligible issue |^  eligible evidence-record |^  protected branch |^  protected worktree " | wc -l)"
printf "%s\n" "$out" | sed -n "1,18p"
echo apply_invoked=false'
```

Exit status: 0

## Stdout

Bytes: 575
Truncated: no

```text
quiet_bytes=754918 quiet_lines=6211
quiet_record_detail_lines=1739
Prune
=====
Mode: dry-run

Diagnostics Logs
----------------
Retention: 30 day(s)
Cutoff:    before 2026-06-06
Path:      /root/.local/state/atelier/diagnostics/commands
Candidates: none

Ignored Runtime, Cache, And Projection Artifacts
-----------------------------------------------
Candidates: 1
  protected runtime-lock .atelier/runtime/.state.db.rebuild.lock - locked by a running or interrupted command; inspect before removal

Git Branches And Worktrees
--------------------------
apply_invoked=false
```

## Stderr

Bytes: 0
Truncated: no

```text
```

