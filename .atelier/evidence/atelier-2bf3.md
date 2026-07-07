---
created_at: "2026-07-06T21:56:05.405290861+00:00"
id: "atelier-2bf3"
evidence_type: "validation"
captured_at: "2026-07-06T21:56:02.900909486+00:00"
command: "bash -lc '\nset -euo pipefail\nprefix=\"$(sed '\"'\"'/^if \\[\\[ \\${1:-} ==/,$d'\"'\"' scripts/check_active_command_guidance.sh | sed '\"'\"'s|^repo_root=.*|repo_root=$(pwd)|'\"'\"')\"\neval \"$prefix\"\nexpect_hit() { output=$(printf \"%s\\n\" \"$1\" | active_content \"${2:-}\" | scan_content); test -n \"$output\"; }\nexpect_clear() { output=$(printf \"%s\\n\" \"$1\" | active_content \"${2:-}\" | scan_content); test -z \"$output\"; }\nlegacy=\"$repo_root/$legacy_callable_document\"\nexpect_clear $'\"'\"'## Legacy Queue Boundary\\nThe legacy `atelier work queue` is bounded here.'\"'\"' \"$legacy\"\nexpect_hit $'\"'\"'## Legacy Queue Boundary Extended\\nUse `atelier work queue` now.'\"'\"' \"$legacy\"\nexpect_hit $'\"'\"'## Legacy Queue Boundary\\nUse `atelier work queue` now.'\"'\"' \"$repo_root/docs/product/work-view-ordering.md\"\nexpect_hit $'\"'\"'## Legacy Queue Boundary\\nUse `atelier work queue` now.'\"'\"' \"$legacy.backup\"\nexpect_hit $'\"'\"'## Legacy Queue Boundary\\n`atelier work queue` is bounded.\\n## Live Guidance\\nUse `atelier work queue` now.'\"'\"' \"$legacy\"\nexpect_clear $'\"'\"'# Setup And Recovery\\nUse atelier doctor --fix here.'\"'\"'\nexpect_clear $'\"'\"'# Unrelated\\nUse atelier export --check here.'\"'\"' \"$repo_root/docs/product/command-audit/export.md\"\nexpect_hit $'\"'\"'# Diagnostics Extended\\nUse atelier doctor --fix now.'\"'\"'\nexpect_hit $'\"'\"'# Unrelated\\nUse atelier export --check now.'\"'\"' \"$repo_root/docs/product/command-audit/export.md.extended\"\nexpect_hit $'\"'\"'# Live Guidance\\nThe hidden API is gone; use atelier doctor --fix now.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nThis is not a recovery command; use atelier export --check now.'\"'\"'\nexpect_hit $'\"'\"'# Live Guidance\\nThe admin path differed; invoke atelier rebuild now.'\"'\"'\nfor fixture in \"Do not forget to run atelier start now.\" \"You must not avoid atelier start.\" \"Never skip atelier start.\" \"atelier start must not be skipped; use it now.\"; do\n  expect_hit \"# Live Guidance\n$fixture\"\n  expect_clear \"## Historical Commands (Non-Normative)\n$fixture\"\ndone\nprintf \"independent exact exception-scope production pipeline passed\\n\"\n'"
exit_status: "0"
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
title: "SUPPLEMENTAL ATELIER-WNRB REVALIDATION: PASS. Supersedes atelier-wnrb and the no-permissive-gap portions of atelier-nm5w and prior scanner evidence; all other mission classifications remain unchanged. Independent production-pipeline testing confirms exact c0mp document plus exact Legacy Queue Boundary heading allowance, and rejects extended/body/wrong-document/suffix spoofs, removed roots inside the boundary, and live re-entry. Restricted commands are allowed only under exact recognized headings or exact restricted audit documents; heading/path suffixes, wrong paths, mixed hidden/admin/recovery live recommendations, and live re-entry reject. Conservative removed-command policy and Historical/Rejected exclusions pass. All 144 self-tests/38 adversarial cases, main/wrapper inventory/live modes, independent 88-doc broad scan, help, focused 9 tests, fmt, master diff, exact c0mp preservation, and atelier check pass. C0mp implementation remains deferred/not-applicable to durs. No product/doc changes."
updated_at: "2026-07-06T21:56:50.438930504+00:00"
---

## Summary

SUPPLEMENTAL ATELIER-WNRB REVALIDATION: PASS. Supersedes atelier-wnrb and the no-permissive-gap portions of atelier-nm5w and prior scanner evidence; all other mission classifications remain unchanged. Independent production-pipeline testing confirms exact c0mp document plus exact Legacy Queue Boundary heading allowance, and rejects extended/body/wrong-document/suffix spoofs, removed roots inside the boundary, and live re-entry. Restricted commands are allowed only under exact recognized headings or exact restricted audit documents; heading/path suffixes, wrong paths, mixed hidden/admin/recovery live recommendations, and live re-entry reject. Conservative removed-command policy and Historical/Rejected exclusions pass. All 144 self-tests/38 adversarial cases, main/wrapper inventory/live modes, independent 88-doc broad scan, help, focused 9 tests, fmt, master diff, exact c0mp preservation, and atelier check pass. C0mp implementation remains deferred/not-applicable to durs. No product/doc changes.

## Command

```console
bash -lc '
set -euo pipefail
prefix="$(sed '"'"'/^if \[\[ \${1:-} ==/,$d'"'"' scripts/check_active_command_guidance.sh | sed '"'"'s|^repo_root=.*|repo_root=$(pwd)|'"'"')"
eval "$prefix"
expect_hit() { output=$(printf "%s\n" "$1" | active_content "${2:-}" | scan_content); test -n "$output"; }
expect_clear() { output=$(printf "%s\n" "$1" | active_content "${2:-}" | scan_content); test -z "$output"; }
legacy="$repo_root/$legacy_callable_document"
expect_clear $'"'"'## Legacy Queue Boundary\nThe legacy `atelier work queue` is bounded here.'"'"' "$legacy"
expect_hit $'"'"'## Legacy Queue Boundary Extended\nUse `atelier work queue` now.'"'"' "$legacy"
expect_hit $'"'"'## Legacy Queue Boundary\nUse `atelier work queue` now.'"'"' "$repo_root/docs/product/work-view-ordering.md"
expect_hit $'"'"'## Legacy Queue Boundary\nUse `atelier work queue` now.'"'"' "$legacy.backup"
expect_hit $'"'"'## Legacy Queue Boundary\n`atelier work queue` is bounded.\n## Live Guidance\nUse `atelier work queue` now.'"'"' "$legacy"
expect_clear $'"'"'# Setup And Recovery\nUse atelier doctor --fix here.'"'"'
expect_clear $'"'"'# Unrelated\nUse atelier export --check here.'"'"' "$repo_root/docs/product/command-audit/export.md"
expect_hit $'"'"'# Diagnostics Extended\nUse atelier doctor --fix now.'"'"'
expect_hit $'"'"'# Unrelated\nUse atelier export --check now.'"'"' "$repo_root/docs/product/command-audit/export.md.extended"
expect_hit $'"'"'# Live Guidance\nThe hidden API is gone; use atelier doctor --fix now.'"'"'
expect_hit $'"'"'# Live Guidance\nThis is not a recovery command; use atelier export --check now.'"'"'
expect_hit $'"'"'# Live Guidance\nThe admin path differed; invoke atelier rebuild now.'"'"'
for fixture in "Do not forget to run atelier start now." "You must not avoid atelier start." "Never skip atelier start." "atelier start must not be skipped; use it now."; do
  expect_hit "# Live Guidance
$fixture"
  expect_clear "## Historical Commands (Non-Normative)
$fixture"
done
printf "independent exact exception-scope production pipeline passed\n"
'
```

Exit status: 0

## Stdout

Bytes: 61
Truncated: no

```text
independent exact exception-scope production pipeline passed
```

## Stderr

Bytes: 0
Truncated: no

```text
```
