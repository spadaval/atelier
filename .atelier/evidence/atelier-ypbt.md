---
created_at: "2026-07-06T23:28:09.485759604+00:00"
id: "atelier-ypbt"
evidence_type: "validation"
captured_at: "2026-07-06T23:28:06.887176721+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nfailures=0\nexpect_hit() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -z \"$output\" ]]; then printf \"MISSED LIVE:\\n%s\\n\" \"$1\"; failures=$((failures+1)); fi; }\nexpect_clear() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -n \"$output\" ]]; then printf \"FALSE POSITIVE:\\n%s\\n=> %s\\n\" \"$1\" \"$output\"; failures=$((failures+1)); fi; }\nexpect_hit $'\"'\"'# Live Guidance\\nMigrate from check to `dep add atelier-demo atelier-blocker`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nSwitch the workflow to `mission show atelier-demo`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nFall back from check to `doctor --fix`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n`lint --all` is responsible for normal validation.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `mission` to be used as the record type.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nPrefer `worker` for use as the role label.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nSelect `close` when used as the transition value.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n```console\\n(venv) user@host$ doctor --fix\\n```'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n```console\\nroot@host # lint --all\\n```'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\n```\\nmission atelier-demo\\n  advances epic atelier-child\\n```'\"'\"'\nprintf \"bounded category-invariant failures: %d\\n\" \"$failures\" >&2\n((failures == 0))\n'"
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
title: "FINAL CATEGORY-INVARIANT REVALIDATION FAIL at exact head b493a433. Prior pgwg/ztnc nine, igno 14, and xs4r 12 all pass independently. The required bounded fresh property-style matrix finds ten production-pipeline gaps: migration with an intervening source or workflow object misses three cases; responsible-for ownership misses one; passive/multiword explicit data suffixes cause three false positives; combined environment-host and spaced root prompts miss two cases; and untyped record-graph data is falsely classified as a command. This invalidates only generalized scanner completeness. All 330 permanent tests, main and wrapper inventory/live modes, independent 88-doc broad scan, help, focused 9 Rust tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes."
updated_at: "2026-07-06T23:28:34.476243502+00:00"
---

## Summary

FINAL CATEGORY-INVARIANT REVALIDATION FAIL at exact head b493a433. Prior pgwg/ztnc nine, igno 14, and xs4r 12 all pass independently. The required bounded fresh property-style matrix finds ten production-pipeline gaps: migration with an intervening source or workflow object misses three cases; responsible-for ownership misses one; passive/multiword explicit data suffixes cause three false positives; combined environment-host and spaced root prompts miss two cases; and untyped record-graph data is falsely classified as a command. This invalidates only generalized scanner completeness. All 330 permanent tests, main and wrapper inventory/live modes, independent 88-doc broad scan, help, focused 9 Rust tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
failures=0
expect_hit() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -z "$output" ]]; then printf "MISSED LIVE:\n%s\n" "$1"; failures=$((failures+1)); fi; }
expect_clear() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -n "$output" ]]; then printf "FALSE POSITIVE:\n%s\n=> %s\n" "$1" "$output"; failures=$((failures+1)); fi; }
expect_hit $'"'"'# Live Guidance\nMigrate from check to `dep add atelier-demo atelier-blocker`.'"'"'
expect_hit $'"'"'# Live Guidance\nSwitch the workflow to `mission show atelier-demo`.'"'"'
expect_hit $'"'"'# Live Guidance\nFall back from check to `doctor --fix`.'"'"'
expect_hit $'"'"'# Live Guidance\n`lint --all` is responsible for normal validation.'"'"'
expect_clear $'"'"'# Live Guidance\nUse `mission` to be used as the record type.'"'"'
expect_clear $'"'"'# Live Guidance\nPrefer `worker` for use as the role label.'"'"'
expect_clear $'"'"'# Live Guidance\nSelect `close` when used as the transition value.'"'"'
expect_hit $'"'"'# Live Guidance\n```console\n(venv) user@host$ doctor --fix\n```'"'"'
expect_hit $'"'"'# Live Guidance\n```console\nroot@host # lint --all\n```'"'"'
expect_clear $'"'"'# Live Guidance\n```\nmission atelier-demo\n  advances epic atelier-child\n```'"'"'
printf "bounded category-invariant failures: %d\n" "$failures" >&2
((failures == 0))
'
```

Exit status: 1

## Stdout

Bytes: 1055
Truncated: no

```text
MISSED LIVE:
# Live Guidance
Migrate from check to `dep add atelier-demo atelier-blocker`.
MISSED LIVE:
# Live Guidance
Switch the workflow to `mission show atelier-demo`.
MISSED LIVE:
# Live Guidance
Fall back from check to `doctor --fix`.
MISSED LIVE:
# Live Guidance
`lint --all` is responsible for normal validation.
FALSE POSITIVE:
# Live Guidance
Use `mission` to be used as the record type.
=> |2|0|# live guidance :: Use `mission` to be used as the record type.
FALSE POSITIVE:
# Live Guidance
Prefer `worker` for use as the role label.
=> |2|0|# live guidance :: Prefer `worker` for use as the role label.
FALSE POSITIVE:
# Live Guidance
Select `close` when used as the transition value.
=> |2|0|# live guidance :: Select `close` when used as the transition value.
MISSED LIVE:
# Live Guidance
```console
(venv) user@host$ doctor --fix
```
MISSED LIVE:
# Live Guidance
```console
root@host # lint --all
```
FALSE POSITIVE:
# Live Guidance
```
mission atelier-demo
  advances epic atelier-child
```
=> |3|2|# live guidance :: mission atelier-demo
```

## Stderr

Bytes: 40
Truncated: no

```text
bounded category-invariant failures: 10
```
