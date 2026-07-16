---
created_at: "2026-07-16T18:39:46.705192809+00:00"
id: "atelier-y13i"
evidence_type: "validation"
captured_at: "2026-07-16T18:39:46.130782496+00:00"
command: "bash -lc 'set -eu\nprintf \"claim.successors_terminal: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show \"$id\" | rg -q \"^Status:   done$\"; done; echo pass\nprintf \"claim.successors_evidence_complete: \"; test \"$(atelier issue show atelier-4h62 | sed -n \"s/^Evidence Gates: linked validating evidence \\([0-9][0-9]*\\); scoped issues without evidence \\([0-9][0-9]*\\)$/\\1 \\2/p\")\" = \"19 0\"; test \"$(atelier issue show atelier-hdff | sed -n \"s/^Evidence Gates: linked validating evidence \\([0-9][0-9]*\\); scoped issues without evidence \\([0-9][0-9]*\\)$/\\1 \\2/p\")\" = \"5 0\"; test \"$(atelier issue show atelier-oc4x | sed -n \"s/^Evidence Gates: linked validating evidence \\([0-9][0-9]*\\); scoped issues without evidence \\([0-9][0-9]*\\)$/\\1 \\2/p\")\" = \"4 0\"; echo pass\nprintf \"claim.successors_contained_in_master: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e \"master:.atelier/issues/$id.md\"; done; echo pass\nprintf \"claim.no_unique_implementation_diff: \"; test -z \"$(git diff --name-only master...HEAD | rg -v \"^\\.atelier/issues/atelier-ikuv(\\.activity/.*)?$\")\"; echo pass\nprintf \"claim.tracker_checks: \"; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass\nprintf \"claim.whitespace: \"; git diff --check master...HEAD; echo pass'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-ikuv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ikuv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -eu\nprintf \"claim.successors_terminal: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show \"$id\" | rg -q \"^Status:   done$\"; done; echo pass\nprintf \"claim.successors_evidence_complete: \"; test \"$(atelier issue show atelier-4h62 | sed -n \"s/^Evidence Gates: linked validating evidence \\([0-9][0-9]*\\); scoped issues without evidence \\([0-9][0-9]*\\)$/\\1 \\2/p\")\" = \"19 0\"; test \"$(atelier issue show atelier-hdff | sed -n \"s/^Evidence Gates: linked validating evidence \\([0-9][0-9]*\\); scoped issues without evidence \\([0-9][0-9]*\\)$/\\1 \\2/p\")\" = \"5 0\"; test \"$(atelier issue show atelier-oc4x | sed -n \"s/^Evidence Gates: linked validating evidence \\([0-9][0-9]*\\); scoped issues without evidence \\([0-9][0-9]*\\)$/\\1 \\2/p\")\" = \"4 0\"; echo pass\nprintf \"claim.successors_contained_in_master: \"; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e \"master:.atelier/issues/$id.md\"; done; echo pass\nprintf \"claim.no_unique_implementation_diff: \"; test -z \"$(git diff --name-only master...HEAD | rg -v \"^\\.atelier/issues/atelier-ikuv(\\.activity/.*)?$\")\"; echo pass\nprintf \"claim.tracker_checks: \"; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass\nprintf \"claim.whitespace: \"; git diff --check master...HEAD; echo pass'"
updated_at: "2026-07-16T18:39:51.824510923+00:00"
---

## Summary

bash -lc 'set -eu
printf "claim.successors_terminal: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show "$id" | rg -q "^Status:   done$"; done; echo pass
printf "claim.successors_evidence_complete: "; test "$(atelier issue show atelier-4h62 | sed -n "s/^Evidence Gates: linked validating evidence \([0-9][0-9]*\); scoped issues without evidence \([0-9][0-9]*\)$/\1 \2/p")" = "19 0"; test "$(atelier issue show atelier-hdff | sed -n "s/^Evidence Gates: linked validating evidence \([0-9][0-9]*\); scoped issues without evidence \([0-9][0-9]*\)$/\1 \2/p")" = "5 0"; test "$(atelier issue show atelier-oc4x | sed -n "s/^Evidence Gates: linked validating evidence \([0-9][0-9]*\); scoped issues without evidence \([0-9][0-9]*\)$/\1 \2/p")" = "4 0"; echo pass
printf "claim.successors_contained_in_master: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e "master:.atelier/issues/$id.md"; done; echo pass
printf "claim.no_unique_implementation_diff: "; test -z "$(git diff --name-only master...HEAD | rg -v "^\.atelier/issues/atelier-ikuv(\.activity/.*)?$")"; echo pass
printf "claim.tracker_checks: "; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass
printf "claim.whitespace: "; git diff --check master...HEAD; echo pass'

## Command

```console
bash -lc 'set -eu
printf "claim.successors_terminal: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do atelier issue show "$id" | rg -q "^Status:   done$"; done; echo pass
printf "claim.successors_evidence_complete: "; test "$(atelier issue show atelier-4h62 | sed -n "s/^Evidence Gates: linked validating evidence \([0-9][0-9]*\); scoped issues without evidence \([0-9][0-9]*\)$/\1 \2/p")" = "19 0"; test "$(atelier issue show atelier-hdff | sed -n "s/^Evidence Gates: linked validating evidence \([0-9][0-9]*\); scoped issues without evidence \([0-9][0-9]*\)$/\1 \2/p")" = "5 0"; test "$(atelier issue show atelier-oc4x | sed -n "s/^Evidence Gates: linked validating evidence \([0-9][0-9]*\); scoped issues without evidence \([0-9][0-9]*\)$/\1 \2/p")" = "4 0"; echo pass
printf "claim.successors_contained_in_master: "; for id in atelier-4h62 atelier-hdff atelier-oc4x; do git cat-file -e "master:.atelier/issues/$id.md"; done; echo pass
printf "claim.no_unique_implementation_diff: "; test -z "$(git diff --name-only master...HEAD | rg -v "^\.atelier/issues/atelier-ikuv(\.activity/.*)?$")"; echo pass
printf "claim.tracker_checks: "; atelier check atelier-ikuv >/dev/null; atelier check >/dev/null; echo pass
printf "claim.whitespace: "; git diff --check master...HEAD; echo pass'
```

Exit status: 1

## Stdout

Bytes: 153
Truncated: no

```text
claim.successors_terminal: pass
claim.successors_evidence_complete: pass
claim.successors_contained_in_master: pass
claim.no_unique_implementation_diff:
```

## Stderr

Bytes: 753
Truncated: no

```text

thread 'main' (447897) panicked at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'main' (447981) panicked at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'main' (448116) panicked at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

