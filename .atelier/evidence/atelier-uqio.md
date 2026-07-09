---
created_at: "2026-07-06T20:05:08.310280214+00:00"
id: "atelier-uqio"
evidence_type: "test"
captured_at: "2026-07-06T20:05:03.411246794+00:00"
command: "bash -lc '\nset -euo pipefail\ngit merge-base --is-ancestor f32d45d2 HEAD\nscripts/check_active_quality_command_guidance.sh --self-test\nscripts/check_active_quality_command_guidance.sh --inventory\nscripts/check_active_quality_command_guidance.sh\nbash -n scripts/check_active_quality_command_guidance.sh\nroot_help=\"$(target/debug/atelier --help)\"\nrg -q \"review\" <<< \"$root_help\"\nrg -q \"history\" <<< \"$root_help\"\nrg -q \"issue\" <<< \"$root_help\"\nif rg -q \"^[[:space:]]+maintenance([[:space:]]|$)\" <<< \"$root_help\"; then exit 1; fi\nfor retired in evidence provider recovery revert supersede rework modify delete prune update worker orchestrator; do\n  if rg -q \"^[[:space:]]+${retired}([[:space:]]|$)\" <<< \"$root_help\"; then exit 1; fi\ndone\nreview_help=\"$(target/debug/atelier review --help)\"\nhistory_help=\"$(target/debug/atelier history --help)\"\nissue_help=\"$(target/debug/atelier issue --help)\"\nrg -q \"inspect|submit|resolve\" <<< \"$review_help\"\nrg -q \"issue\" <<< \"$history_help\"\nrg -q \"show|transition\" <<< \"$issue_help\"\ncargo nextest run -p atelier-app command_surface\ncargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases\ncargo fmt -- --check\ngit diff --check master...HEAD\ntarget/debug/atelier check\ngit merge-base --is-ancestor 60634f2b HEAD\ngit diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md\n'"
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
updated_at: "2026-07-06T20:05:14.733740326+00:00"
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
rg -q "review" <<< "$root_help"
rg -q "history" <<< "$root_help"
rg -q "issue" <<< "$root_help"
if rg -q "^[[:space:]]+maintenance([[:space:]]|$)" <<< "$root_help"; then exit 1; fi
for retired in evidence provider recovery revert supersede rework modify delete prune update worker orchestrator; do
  if rg -q "^[[:space:]]+${retired}([[:space:]]|$)" <<< "$root_help"; then exit 1; fi
done
review_help="$(target/debug/atelier review --help)"
history_help="$(target/debug/atelier history --help)"
issue_help="$(target/debug/atelier issue --help)"
rg -q "inspect|submit|resolve" <<< "$review_help"
rg -q "issue" <<< "$history_help"
rg -q "show|transition" <<< "$issue_help"
cargo nextest run -p atelier-app command_surface
cargo nextest run -p atelier-cli test_obsolete_command_surfaces_are_removed_without_aliases
cargo fmt -- --check
git diff --check master...HEAD
target/debug/atelier check
git merge-base --is-ancestor 60634f2b HEAD
git diff --exit-code 60634f2b -- docs/product/issue-inventory-and-mission-overview.md
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
