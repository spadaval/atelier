---
created_at: "2026-07-06T21:18:14.761355933+00:00"
id: "atelier-ypgn"
evidence_type: "validation"
captured_at: "2026-07-06T21:18:12.391001839+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nfailures=0\nfor example in \\\n  \"Do not forget to run atelier start now.\" \\\n  \"You must not avoid atelier start.\" \\\n  \"Never skip atelier start.\" \\\n  \"atelier start must not be skipped; use it now.\"\ndo\n  output=$(printf \"# Live Guidance\\n%s\\n\" \"$example\" | active_content | scan_content)\n  if [[ -z \"$output\" ]]; then\n    printf \"MISSED LIVE GUIDANCE: %s\\n\" \"$example\"\n    failures=$((failures + 1))\n  fi\ndone\nprintf \"command-local false negatives: %d\\n\" \"$failures\" >&2\n((failures == 0))\n'"
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
    id: "atelier-p0am"
    role: "validates"
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "SUPPLEMENTAL ATELIER-Z6Q1 REVALIDATION: FAIL. The four original review fixtures now reject correctly, but systematic command-local allowance testing through the actual production active_content|scan_content pipeline finds four clear live recommendations still accepted: 'Do not forget to run atelier start now', 'You must not avoid atelier start', 'Never skip atelier start', and 'atelier start must not be skipped; use it now'. Direct-negation handling mistakes negative predicates on forget/avoid/skip for prohibition of the command, and post-command passive 'must not be skipped' is likewise allowed. This invalidates only atelier-e3x2/atelier-cyt7 no-permissive-gap claims. All 129 generated tests, 23 built-in adversarial cases, inventory/live/wrapper modes, broad indexed scan, historical/rejected re-entry, branch help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check otherwise pass. No implementation changes."
updated_at: "2026-07-06T21:18:54.367489360+00:00"
---

## Summary

SUPPLEMENTAL ATELIER-Z6Q1 REVALIDATION: FAIL. The four original review fixtures now reject correctly, but systematic command-local allowance testing through the actual production active_content|scan_content pipeline finds four clear live recommendations still accepted: 'Do not forget to run atelier start now', 'You must not avoid atelier start', 'Never skip atelier start', and 'atelier start must not be skipped; use it now'. Direct-negation handling mistakes negative predicates on forget/avoid/skip for prohibition of the command, and post-command passive 'must not be skipped' is likewise allowed. This invalidates only atelier-e3x2/atelier-cyt7 no-permissive-gap claims. All 129 generated tests, 23 built-in adversarial cases, inventory/live/wrapper modes, broad indexed scan, historical/rejected re-entry, branch help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check otherwise pass. No implementation changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
failures=0
for example in \
  "Do not forget to run atelier start now." \
  "You must not avoid atelier start." \
  "Never skip atelier start." \
  "atelier start must not be skipped; use it now."
do
  output=$(printf "# Live Guidance\n%s\n" "$example" | active_content | scan_content)
  if [[ -z "$output" ]]; then
    printf "MISSED LIVE GUIDANCE: %s\n" "$example"
    failures=$((failures + 1))
  fi
done
printf "command-local false negatives: %d\n" "$failures" >&2
((failures == 0))
'
```

Exit status: 1

## Stdout

Bytes: 235
Truncated: no

```text
MISSED LIVE GUIDANCE: Do not forget to run atelier start now.
MISSED LIVE GUIDANCE: You must not avoid atelier start.
MISSED LIVE GUIDANCE: Never skip atelier start.
MISSED LIVE GUIDANCE: atelier start must not be skipped; use it now.
```

## Stderr

Bytes: 33
Truncated: no

```text
command-local false negatives: 4
```
