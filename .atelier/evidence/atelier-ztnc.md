---
created_at: "2026-07-06T22:30:42.796189217+00:00"
id: "atelier-ztnc"
evidence_type: "validation"
captured_at: "2026-07-06T22:30:40.435015492+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nfailures=0\nexpect_hit() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -z \"$output\" ]]; then printf \"MISSED LIVE:\\n%s\\n\" \"$1\"; failures=$((failures+1)); fi; }\nexpect_clear() { output=$(printf \"%s\\n\" \"$1\" | active_content | scan_content); if [[ -n \"$output\" ]]; then printf \"FALSE POSITIVE:\\n%s\\n=> %s\\n\" \"$1\" \"$output\"; failures=$((failures+1)); fi; }\nexpect_hit $'\"'\"'# Live Guidance\\nPrefer `lint --all` for validation.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nThe current command is `doctor --fix`.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nCall `dep add atelier-demo atelier-blocker` to link records.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nNormal workflow: `mission show atelier-demo`.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `mission` as the record type.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `close` as the transition name.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `worker` as the role value.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\nUse `list` as a data label.'\"'\"'\nexpect_clear $'\"'\"'# Live Guidance\\n```\\nmission: atelier-demo\\nstatus: todo\\n```'\"'\"'\nprintf \"bare structural boundary failures: %d\\n\" \"$failures\" >&2\n((failures == 0))\n'"
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
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "SUPPLEMENTAL ATELIER-B8XW REVALIDATION: FAIL. Exact b8xw fixtures, authoritative-doc cleanup, inventory-derived fenced/list/prompt forms, prior section/document exceptions and re-entry, all 241 built-ins, live 88-doc/wrapper/inventory/broad scans, help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check pass. Independent production-pipeline boundary testing still finds four actionable inline misses: Prefer ; The current command is ; Call ; Normal workflow: . It also finds false positives for ordinary data/type/transition phrasing: Use  as the record type;  as the transition name;  as the role value;  as a data label; and untyped fenced YAML . This invalidates only atelier-bbh4 and prior bare-scanner completeness claims. No implementation changes."
updated_at: "2026-07-06T22:30:49.071630101+00:00"
---

## Summary

SUPPLEMENTAL ATELIER-B8XW REVALIDATION: FAIL. Exact b8xw fixtures, authoritative-doc cleanup, inventory-derived fenced/list/prompt forms, prior section/document exceptions and re-entry, all 241 built-ins, live 88-doc/wrapper/inventory/broad scans, help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check pass. Independent production-pipeline boundary testing still finds four actionable inline misses: Prefer ; The current command is ; Call ; Normal workflow: . It also finds false positives for ordinary data/type/transition phrasing: Use  as the record type;  as the transition name;  as the role value;  as a data label; and untyped fenced YAML . This invalidates only atelier-bbh4 and prior bare-scanner completeness claims. No implementation changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
failures=0
expect_hit() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -z "$output" ]]; then printf "MISSED LIVE:\n%s\n" "$1"; failures=$((failures+1)); fi; }
expect_clear() { output=$(printf "%s\n" "$1" | active_content | scan_content); if [[ -n "$output" ]]; then printf "FALSE POSITIVE:\n%s\n=> %s\n" "$1" "$output"; failures=$((failures+1)); fi; }
expect_hit $'"'"'# Live Guidance\nPrefer `lint --all` for validation.'"'"'
expect_hit $'"'"'# Live Guidance\nThe current command is `doctor --fix`.'"'"'
expect_hit $'"'"'# Live Guidance\nCall `dep add atelier-demo atelier-blocker` to link records.'"'"'
expect_hit $'"'"'# Live Guidance\nNormal workflow: `mission show atelier-demo`.'"'"'
expect_clear $'"'"'# Live Guidance\nUse `mission` as the record type.'"'"'
expect_clear $'"'"'# Live Guidance\nUse `close` as the transition name.'"'"'
expect_clear $'"'"'# Live Guidance\nUse `worker` as the role value.'"'"'
expect_clear $'"'"'# Live Guidance\nUse `list` as a data label.'"'"'
expect_clear $'"'"'# Live Guidance\n```\nmission: atelier-demo\nstatus: todo\n```'"'"'
printf "bare structural boundary failures: %d\n" "$failures" >&2
((failures == 0))
'
```

Exit status: 1

## Stdout

Bytes: 918
Truncated: no

```text
MISSED LIVE:
# Live Guidance
Prefer `lint --all` for validation.
MISSED LIVE:
# Live Guidance
The current command is `doctor --fix`.
MISSED LIVE:
# Live Guidance
Call `dep add atelier-demo atelier-blocker` to link records.
MISSED LIVE:
# Live Guidance
Normal workflow: `mission show atelier-demo`.
FALSE POSITIVE:
# Live Guidance
Use `mission` as the record type.
=> |2|0|# live guidance :: Use `mission` as the record type.
FALSE POSITIVE:
# Live Guidance
Use `close` as the transition name.
=> |2|0|# live guidance :: Use `close` as the transition name.
FALSE POSITIVE:
# Live Guidance
Use `worker` as the role value.
=> |2|0|# live guidance :: Use `worker` as the role value.
FALSE POSITIVE:
# Live Guidance
Use `list` as a data label.
=> |2|0|# live guidance :: Use `list` as a data label.
FALSE POSITIVE:
# Live Guidance
```
mission: atelier-demo
status: todo
```
=> |3|1|# live guidance :: mission: atelier-demo
```

## Stderr

Bytes: 37
Truncated: no

```text
bare structural boundary failures: 9
```
