---
created_at: "2026-07-06T21:36:13.685952083+00:00"
id: "atelier-wnrb"
evidence_type: "validation"
captured_at: "2026-07-06T21:36:11.690996978+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nfailures=0\ncheck_missed() {\n  local input=$1\n  local output\n  output=$(printf \"%s\\n\" \"$input\" | active_content | scan_content)\n  if [[ -z \"$output\" ]]; then\n    printf \"MISSED LIVE GUIDANCE:\\n%s\\n\" \"$input\"\n    failures=$((failures + 1))\n  fi\n}\ncheck_missed $'\"'\"'## Legacy Queue Boundary Extended\\nUse `atelier work queue` now.'\"'\"'\ncheck_missed $'\"'\"'# Live Guidance\\nThe hidden API is gone; use atelier doctor --fix now.'\"'\"'\ncheck_missed $'\"'\"'# Live Guidance\\nThis is not a recovery command; use atelier export --check now.'\"'\"'\ncheck_missed $'\"'\"'# Live Guidance\\nThe admin path differed; invoke atelier rebuild now.'\"'\"'\nprintf \"exception-scope false negatives: %d\\n\" \"$failures\" >&2\n((failures == 0))\n'"
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
title: "SUPPLEMENTAL ATELIER-YPGN REVALIDATION: FAIL. Conservative removed-command behavior passes the exact four double-negative fixtures and all prior active negative controls, with Historical/Rejected section allowance and live re-entry. However exception scoping remains permissive through the actual production active_content|scan_content pipeline: heading 'Legacy Queue Boundary Extended' receives the c0mp work-queue exception; 'The hidden API is gone; use atelier doctor --fix now', 'This is not a recovery command; use atelier export --check now', and 'The admin path differed; invoke atelier rebuild now' are accepted because restricted context is line-wide rather than command-local. This invalidates only atelier-nm5w and prior no-permissive-gap scanner claims. All 139 self-tests/33 built-ins, live 88-doc scan, wrappers/inventory, broad scan, help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check otherwise pass. No implementation changes."
updated_at: "2026-07-06T21:36:57.006015102+00:00"
---

## Summary

SUPPLEMENTAL ATELIER-YPGN REVALIDATION: FAIL. Conservative removed-command behavior passes the exact four double-negative fixtures and all prior active negative controls, with Historical/Rejected section allowance and live re-entry. However exception scoping remains permissive through the actual production active_content|scan_content pipeline: heading 'Legacy Queue Boundary Extended' receives the c0mp work-queue exception; 'The hidden API is gone; use atelier doctor --fix now', 'This is not a recovery command; use atelier export --check now', and 'The admin path differed; invoke atelier rebuild now' are accepted because restricted context is line-wide rather than command-local. This invalidates only atelier-nm5w and prior no-permissive-gap scanner claims. All 139 self-tests/33 built-ins, live 88-doc scan, wrappers/inventory, broad scan, help, focused 9 tests, fmt, master diff, c0mp preservation, and atelier check otherwise pass. No implementation changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
failures=0
check_missed() {
  local input=$1
  local output
  output=$(printf "%s\n" "$input" | active_content | scan_content)
  if [[ -z "$output" ]]; then
    printf "MISSED LIVE GUIDANCE:\n%s\n" "$input"
    failures=$((failures + 1))
  fi
}
check_missed $'"'"'## Legacy Queue Boundary Extended\nUse `atelier work queue` now.'"'"'
check_missed $'"'"'# Live Guidance\nThe hidden API is gone; use atelier doctor --fix now.'"'"'
check_missed $'"'"'# Live Guidance\nThis is not a recovery command; use atelier export --check now.'"'"'
check_missed $'"'"'# Live Guidance\nThe admin path differed; invoke atelier rebuild now.'"'"'
printf "exception-scope false negatives: %d\n" "$failures" >&2
((failures == 0))
'
```

Exit status: 1

## Stdout

Bytes: 371
Truncated: no

```text
MISSED LIVE GUIDANCE:
## Legacy Queue Boundary Extended
Use `atelier work queue` now.
MISSED LIVE GUIDANCE:
# Live Guidance
The hidden API is gone; use atelier doctor --fix now.
MISSED LIVE GUIDANCE:
# Live Guidance
This is not a recovery command; use atelier export --check now.
MISSED LIVE GUIDANCE:
# Live Guidance
The admin path differed; invoke atelier rebuild now.
```

## Stderr

Bytes: 35
Truncated: no

```text
exception-scope false negatives: 4
```
