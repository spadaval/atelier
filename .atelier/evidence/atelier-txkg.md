---
created_at: "2026-07-06T20:04:26.475499961+00:00"
id: "atelier-txkg"
evidence_type: "test"
captured_at: "2026-07-06T20:04:21.882894792+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor f32d45d2 HEAD\nscripts/check_active_quality_command_guidance.sh --self-test\nscripts/check_active_quality_command_guidance.sh --inventory\nscripts/check_active_quality_command_guidance.sh\nbash -n scripts/check_active_quality_command_guidance.sh\nroot_help=\"$(target/debug/atelier --help)\"\nprintf \"%s\\n\" \"$root_help\" | rg -q \"review\"\nprintf \"%s\\n\" \"$root_help\" | rg -q \"history\"\nprintf \"%s\\n\" \"$root_help\" | rg -q \"issue\"\nif printf \"%s\\n\" \"$root_help\" | rg -q \"^[[:space:]]+maintenance([[:space:]]|$)\"; then exit 1; fi\nfor retired in evidence provider recovery revert supersede rework modify delete prune update worker orchestrator; do\n  if printf \"%s\\n\" \"$root_help\" | rg -q \"^[[:space:]]+${retired}([[:space:]]|$)\"; then exit 1; fi\ndone\ntarget/debug/atelier review --help | rg -q \"inspect|submit|resolve\"\ntarget/debug/atelier history --help | rg -q \"issue\"\ntarget/debug/atelier issue --help | rg -q \"show|transition\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ntarget/debug/atelier check\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\ntest -z \"$(git status --porcelain)\"\n'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-p0am"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "atelier-3ezp implementation proof (not independent validation): verdict-derived maintenance prohibition, coverage meta-test, broad categorized inventory, indexed quality scan, branch-built help parity, focused tests, formatting, branch diff, tracker health, and c0mp preservation pass"
updated_at: "2026-07-06T20:04:32.817947462+00:00"
---

## Summary

atelier-3ezp implementation proof (not independent validation): verdict-derived maintenance prohibition, coverage meta-test, broad categorized inventory, indexed quality scan, branch-built help parity, focused tests, formatting, branch diff, tracker health, and c0mp preservation pass

## Command

```console
bash -lc '
set -euo pipefail
git merge-base --is-ancestor f32d45d2 HEAD
scripts/check_active_quality_command_guidance.sh --self-test
scripts/check_active_quality_command_guidance.sh --inventory
scripts/check_active_quality_command_guidance.sh
bash -n scripts/check_active_quality_command_guidance.sh
root_help="$(target/debug/atelier --help)"
printf "%s\n" "$root_help" | rg -q "review"
printf "%s\n" "$root_help" | rg -q "history"
printf "%s\n" "$root_help" | rg -q "issue"
if printf "%s\n" "$root_help" | rg -q "^[[:space:]]+maintenance([[:space:]]|$)"; then exit 1; fi
for retired in evidence provider recovery revert supersede rework modify delete prune update worker orchestrator; do
  if printf "%s\n" "$root_help" | rg -q "^[[:space:]]+${retired}([[:space:]]|$)"; then exit 1; fi
done
target/debug/atelier review --help | rg -q "inspect|submit|resolve"
target/debug/atelier history --help | rg -q "issue"
target/debug/atelier issue --help | rg -q "show|transition"
cargo nextest run -p atelier-app command_surface
cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases
cargo fmt -- --check
git diff --check master...HEAD
target/debug/atelier check
git merge-base --is-ancestor 60634f2b HEAD
git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md
test -z "$(git status --porcelain)"
'
```

Exit status: 1

## Stdout

Bytes: 232
Truncated: no

```text
active quality guidance self-test passed: 103 prohibited/context-restricted example(s)
command inventory passed: 11 visible, 10 hidden, 44 removed, 30 audit-token root(s)
active quality guidance check passed: 10 indexed document(s)
```

## Stderr

Bytes: 0
Truncated: no

```text
```
