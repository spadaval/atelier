---
created_at: "2026-07-06T23:06:04.179257865+00:00"
id: "atelier-xs4r"
evidence_type: "validation"
captured_at: "2026-07-06T23:06:01.362157712+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nfailures=0\nexpect_hit() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -z \"$output\" ]]; then printf \"MISSED LIVE:\\n%s\\n\" \"$1\"; failures=$((failures+1)); fi; }\nexpect_clear() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -n \"$output\" ]]; then printf \"FALSE POSITIVE:\\n%s\\n=> %s\\n\" \"$1\" \"$output\"; failures=$((failures+1)); fi; }\nexpect_hit $'\"'\"'# Live Guidance\\nCanonical command: `lint --all`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nPrimary command: `doctor --fix`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nMigrate to `dep add atelier-demo atelier-blocker`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nFall back to `mission show atelier-demo`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n| Primary command | `lint --all` |'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `mission` to represent the record type.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nSelect `close` when setting the transition name.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n```console\\nhost$ doctor --fix\\n```'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n```console\\n% lint --all\\n```'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n```console\\nroot@host# doctor --fix\\n```'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\n```text\\n- mission\\n- task\\n```'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\n```markdown\\n- mission\\n- task\\n```'\"'\"'\nprintf \"bounded extension failures: %d\\n\" \"$failures\" >&2\n((failures == 0))\n'"
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
title: "FINAL INDEPENDENT REVALIDATION FAIL at exact head 9a213bf7. Prior pgwg/ztnc nine and exact igno 14 all pass. The required bounded independent extension finds twelve production-pipeline gaps: five canonical/primary/migrate-to/fall-back action and table forms are missed; two to-represent/when-setting data forms are false positives; three host, percent, and root console prompts are missed; and text plus markdown fenced scalar lists are false positives. This invalidates only final scanner completeness. All 294 permanent tests, main and wrapper inventory/live modes, independent 88-doc broad scan, help, focused 9 Rust tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes."
updated_at: "2026-07-06T23:06:29.054081293+00:00"
---

## Summary

FINAL INDEPENDENT REVALIDATION FAIL at exact head 9a213bf7. Prior pgwg/ztnc nine and exact igno 14 all pass. The required bounded independent extension finds twelve production-pipeline gaps: five canonical/primary/migrate-to/fall-back action and table forms are missed; two to-represent/when-setting data forms are false positives; three host, percent, and root console prompts are missed; and text plus markdown fenced scalar lists are false positives. This invalidates only final scanner completeness. All 294 permanent tests, main and wrapper inventory/live modes, independent 88-doc broad scan, help, focused 9 Rust tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
failures=0
expect_hit() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -z "$output" ]]; then printf "MISSED LIVE:\n%s\n" "$1"; failures=$((failures+1)); fi; }
expect_clear() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -n "$output" ]]; then printf "FALSE POSITIVE:\n%s\n=> %s\n" "$1" "$output"; failures=$((failures+1)); fi; }
expect_hit $'"'"'# Live Guidance\nCanonical command: `lint --all`.'"'"'
expect_hit $'"'"'# Live Guidance\nPrimary command: `doctor --fix`.'"'"'
expect_hit $'"'"'# Live Guidance\nMigrate to `dep add atelier-demo atelier-blocker`.'"'"'
expect_hit $'"'"'# Live Guidance\nFall back to `mission show atelier-demo`.'"'"'
expect_hit $'"'"'# Live Guidance\n| Primary command | `lint --all` |'"'"'
expect_clear $'"'"'# Live Guidance\nUse `mission` to represent the record type.'"'"'
expect_clear $'"'"'# Live Guidance\nSelect `close` when setting the transition name.'"'"'
expect_hit $'"'"'# Live Guidance\n```console\nhost$ doctor --fix\n```'"'"'
expect_hit $'"'"'# Live Guidance\n```console\n% lint --all\n```'"'"'
expect_hit $'"'"'# Live Guidance\n```console\nroot@host# doctor --fix\n```'"'"'
expect_clear $'"'"'# Live Guidance\n```text\n- mission\n- task\n```'"'"'
expect_clear $'"'"'# Live Guidance\n```markdown\n- mission\n- task\n```'"'"'
printf "bounded extension failures: %d\n" "$failures" >&2
((failures == 0))
'
```

Exit status: 1

## Stdout

Bytes: 1031
Truncated: no

```text
MISSED LIVE:
# Live Guidance
Canonical command: `lint --all`.
MISSED LIVE:
# Live Guidance
Primary command: `doctor --fix`.
MISSED LIVE:
# Live Guidance
Migrate to `dep add atelier-demo atelier-blocker`.
MISSED LIVE:
# Live Guidance
Fall back to `mission show atelier-demo`.
MISSED LIVE:
# Live Guidance
| Primary command | `lint --all` |
FALSE POSITIVE:
# Live Guidance
Use `mission` to represent the record type.
=> |2|0|# live guidance :: Use `mission` to represent the record type.
FALSE POSITIVE:
# Live Guidance
Select `close` when setting the transition name.
=> |2|0|# live guidance :: Select `close` when setting the transition name.
MISSED LIVE:
# Live Guidance
```console
host$ doctor --fix
```
MISSED LIVE:
# Live Guidance
```console
% lint --all
```
MISSED LIVE:
# Live Guidance
```console
root@host# doctor --fix
```
FALSE POSITIVE:
# Live Guidance
```text
- mission
- task
```
=> |3|0|# live guidance :: - mission
FALSE POSITIVE:
# Live Guidance
```markdown
- mission
- task
```
=> |3|0|# live guidance :: - mission
```

## Stderr

Bytes: 31
Truncated: no

```text
bounded extension failures: 12
```
