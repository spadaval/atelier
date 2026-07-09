---
created_at: "2026-07-06T23:44:26.643503209+00:00"
id: "atelier-sa0l"
evidence_type: "validation"
captured_at: "2026-07-06T23:44:24.321244711+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"; eval \"$prefix\"; failures=0\nhit(){ output=$(printf \"%s\\n\" \"$1\"|active_content|scan_content); if [[ -z \"$output\" ]]; then printf \"MISSED LIVE:\\n%s\\n\" \"$1\"; failures=$((failures+1)); fi; }\nclear(){ output=$(printf \"%s\\n\" \"$1\"|active_content|scan_content); if [[ -n \"$output\" ]]; then printf \"FALSE POSITIVE:\\n%s\\n=> %s\\n\" \"$1\" \"$output\"; failures=$((failures+1)); fi; }\nhit $'\"'\"'# Live\\nMove from the old local operator workflow command to `mission show atelier-demo`.'\"'\"'\nclear $'\"'\"'# Live\\nUse `mission` to continue to be used as the record type.'\"'\"'\nhit $'\"'\"'# Live\\nUse `mission` to show the transition status.'\"'\"'\nhit $'\"'\"'# Live\\n```console\\n(py) (git:main) user@host$ lint --all\\n```'\"'\"'\nhit $'\"'\"'# Live\\n```console\\nuser@host ~/repo $ doctor --fix\\n```'\"'\"'\nprintf \"declared-grammar boundary failures: %d\\n\" \"$failures\" >&2; ((failures==0))\n'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-durs"
    role: "validates"
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "FINAL DECLARED-GRAMMAR REVALIDATION FAIL at exact head daee4225. All prior 45 cases pass independently. One bounded fresh invariant matrix finds exactly five boundary escapes: a six-token migration complement is missed; a four-word passive descriptor is a false positive; an actionable show-transition-status suffix is incorrectly treated as data; a double-parenthesized environment plus host prompt is missed; and a spaced host/path prompt is missed. This invalidates only generalized scanner completeness. All 354 permanent tests, main and wrapper inventory/live modes, broad 88-doc scan, help, focused 9 Rust tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes."
updated_at: "2026-07-06T23:44:51.369673804+00:00"
---

## Summary

FINAL DECLARED-GRAMMAR REVALIDATION FAIL at exact head daee4225. All prior 45 cases pass independently. One bounded fresh invariant matrix finds exactly five boundary escapes: a six-token migration complement is missed; a four-word passive descriptor is a false positive; an actionable show-transition-status suffix is incorrectly treated as data; a double-parenthesized environment plus host prompt is missed; and a spaced host/path prompt is missed. This invalidates only generalized scanner completeness. All 354 permanent tests, main and wrapper inventory/live modes, broad 88-doc scan, help, focused 9 Rust tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"; eval "$prefix"; failures=0
hit(){ output=$(printf "%s\n" "$1"|active_content|scan_content); if [[ -z "$output" ]]; then printf "MISSED LIVE:\n%s\n" "$1"; failures=$((failures+1)); fi; }
clear(){ output=$(printf "%s\n" "$1"|active_content|scan_content); if [[ -n "$output" ]]; then printf "FALSE POSITIVE:\n%s\n=> %s\n" "$1" "$output"; failures=$((failures+1)); fi; }
hit $'"'"'# Live\nMove from the old local operator workflow command to `mission show atelier-demo`.'"'"'
clear $'"'"'# Live\nUse `mission` to continue to be used as the record type.'"'"'
hit $'"'"'# Live\nUse `mission` to show the transition status.'"'"'
hit $'"'"'# Live\n```console\n(py) (git:main) user@host$ lint --all\n```'"'"'
hit $'"'"'# Live\n```console\nuser@host ~/repo $ doctor --fix\n```'"'"'
printf "declared-grammar boundary failures: %d\n" "$failures" >&2; ((failures==0))
'
```

Exit status: 1

## Stdout

Bytes: 462
Truncated: no

```text
MISSED LIVE:
# Live
Move from the old local operator workflow command to `mission show atelier-demo`.
FALSE POSITIVE:
# Live
Use `mission` to continue to be used as the record type.
=> |2|0|# live :: Use `mission` to continue to be used as the record type.
MISSED LIVE:
# Live
Use `mission` to show the transition status.
MISSED LIVE:
# Live
```console
(py) (git:main) user@host$ lint --all
```
MISSED LIVE:
# Live
```console
user@host ~/repo $ doctor --fix
```
```

## Stderr

Bytes: 38
Truncated: no

```text
declared-grammar boundary failures: 5
```
