---
created_at: "2026-07-06T22:50:44.118786204+00:00"
id: "atelier-igno"
evidence_type: "validation"
captured_at: "2026-07-06T22:50:41.427647103+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nfailures=0\nexpect_hit() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -z \"$output\" ]]; then printf \"MISSED LIVE:\\n%s\\n\" \"$1\"; failures=$((failures+1)); fi; }\nexpect_clear() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -n \"$output\" ]]; then printf \"FALSE POSITIVE:\\n%s\\n=> %s\\n\" \"$1\" \"$output\"; failures=$((failures+1)); fi; }\nexpect_hit $'\"'\"'# Live Guidance\\nAdopt `lint --all` for validation.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nPick `doctor --fix` for repair.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nSwitch to `dep add atelier-demo atelier-blocker`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nDefault command: `mission show atelier-demo`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nStandard workflow: `mission show atelier-demo`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n| Default command | `lint --all` |'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `mission` for the record type.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `close` for the transition name.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nSelect `worker` for the role value.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nPrefer `list` for the data label.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n```console\\nuser@host$ doctor --fix\\n```'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\n```console\\n(venv) $ lint --all\\n```'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\n```yaml\\n- mission\\n- task\\n```'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\n```\\n- mission\\n- task\\n```'\"'\"'\nprintf \"systematic structural failures: %d\\n\" \"$failures\" >&2\n((failures == 0))\n'"
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
title: "FINAL STRUCTURAL BARE-SCANNER REVALIDATION FAIL at head a22cd39f. Exact prior nine cases pass. Systematic production-pipeline testing finds six actionable inline misses: Adopt lint; Pick doctor; Switch to dep add; Default command mission show; Standard workflow mission show; and a Default command table cell. It finds four false positives for for-the-type, transition, role, and label data syntax; misses two realistic console prompts; and falsely flags YAML scalar lists in both labeled and untyped fences. This invalidates only final bare-scanner completeness claims. All 269 built-ins, live 88-doc main/wrapper/inventory/broad scans, help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes."
updated_at: "2026-07-06T22:51:09.851046707+00:00"
---

## Summary

FINAL STRUCTURAL BARE-SCANNER REVALIDATION FAIL at head a22cd39f. Exact prior nine cases pass. Systematic production-pipeline testing finds six actionable inline misses: Adopt lint; Pick doctor; Switch to dep add; Default command mission show; Standard workflow mission show; and a Default command table cell. It finds four false positives for for-the-type, transition, role, and label data syntax; misses two realistic console prompts; and falsely flags YAML scalar lists in both labeled and untyped fences. This invalidates only final bare-scanner completeness claims. All 269 built-ins, live 88-doc main/wrapper/inventory/broad scans, help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check pass. No implementation changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
failures=0
expect_hit() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -z "$output" ]]; then printf "MISSED LIVE:\n%s\n" "$1"; failures=$((failures+1)); fi; }
expect_clear() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -n "$output" ]]; then printf "FALSE POSITIVE:\n%s\n=> %s\n" "$1" "$output"; failures=$((failures+1)); fi; }
expect_hit $'"'"'# Live Guidance\nAdopt `lint --all` for validation.'"'"'
expect_hit $'"'"'# Live Guidance\nPick `doctor --fix` for repair.'"'"'
expect_hit $'"'"'# Live Guidance\nSwitch to `dep add atelier-demo atelier-blocker`.'"'"'
expect_hit $'"'"'# Live Guidance\nDefault command: `mission show atelier-demo`.'"'"'
expect_hit $'"'"'# Live Guidance\nStandard workflow: `mission show atelier-demo`.'"'"'
expect_hit $'"'"'# Live Guidance\n| Default command | `lint --all` |'"'"'
expect_clear $'"'"'# Live Guidance\nUse `mission` for the record type.'"'"'
expect_clear $'"'"'# Live Guidance\nUse `close` for the transition name.'"'"'
expect_clear $'"'"'# Live Guidance\nSelect `worker` for the role value.'"'"'
expect_clear $'"'"'# Live Guidance\nPrefer `list` for the data label.'"'"'
expect_hit $'"'"'# Live Guidance\n```console\nuser@host$ doctor --fix\n```'"'"'
expect_hit $'"'"'# Live Guidance\n```console\n(venv) $ lint --all\n```'"'"'
expect_clear $'"'"'# Live Guidance\n```yaml\n- mission\n- task\n```'"'"'
expect_clear $'"'"'# Live Guidance\n```\n- mission\n- task\n```'"'"'
printf "systematic structural failures: %d\n" "$failures" >&2
((failures == 0))
'
```

Exit status: 1

## Stdout

Bytes: 1264
Truncated: no

```text
MISSED LIVE:
# Live Guidance
Adopt `lint --all` for validation.
MISSED LIVE:
# Live Guidance
Pick `doctor --fix` for repair.
MISSED LIVE:
# Live Guidance
Switch to `dep add atelier-demo atelier-blocker`.
MISSED LIVE:
# Live Guidance
Default command: `mission show atelier-demo`.
MISSED LIVE:
# Live Guidance
Standard workflow: `mission show atelier-demo`.
MISSED LIVE:
# Live Guidance
| Default command | `lint --all` |
FALSE POSITIVE:
# Live Guidance
Use `mission` for the record type.
=> |2|0|# live guidance :: Use `mission` for the record type.
FALSE POSITIVE:
# Live Guidance
Use `close` for the transition name.
=> |2|0|# live guidance :: Use `close` for the transition name.
FALSE POSITIVE:
# Live Guidance
Select `worker` for the role value.
=> |2|0|# live guidance :: Select `worker` for the role value.
FALSE POSITIVE:
# Live Guidance
Prefer `list` for the data label.
=> |2|0|# live guidance :: Prefer `list` for the data label.
MISSED LIVE:
# Live Guidance
```console
user@host$ doctor --fix
```
MISSED LIVE:
# Live Guidance
```console
(venv) $ lint --all
```
FALSE POSITIVE:
# Live Guidance
```yaml
- mission
- task
```
=> |3|0|# live guidance :: - mission
FALSE POSITIVE:
# Live Guidance
```
- mission
- task
```
=> |3|2|# live guidance :: - mission
```

## Stderr

Bytes: 35
Truncated: no

```text
systematic structural failures: 14
```
