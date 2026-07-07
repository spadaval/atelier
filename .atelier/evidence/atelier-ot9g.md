---
created_at: "2026-07-06T18:32:44.771447622+00:00"
id: "atelier-ot9g"
evidence_type: "validation"
captured_at: "2026-07-06T18:32:44.540110485+00:00"
command: "bash -lc 'set -euo pipefail; target/debug/atelier review --help; target/debug/atelier review open --help; for verb in status comments link comment approve request-changes; do if target/debug/atelier review \"$verb\" --help >/dev/null 2>&1; then echo \"unexpected accepted verb: $verb\"; exit 1; fi; echo \"rejected verb: $verb\"; done; for flag in title body source-branch target-branch provider owner repo host; do if target/debug/atelier review open --\"$flag\" x >/dev/null 2>&1; then echo \"unexpected accepted flag: --$flag\"; exit 1; fi; echo \"rejected open flag: --$flag\"; done; if target/debug/atelier pr --help >/dev/null 2>&1; then echo \"unexpected accepted root: pr\"; exit 1; fi; echo \"rejected root: pr\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent help and rejection validation: only open/show/submit/resolve/merge survive; folded verbs, legacy pr, and manual provider/title/branch fields all reject without aliases."
updated_at: "2026-07-06T18:32:48.972529793+00:00"
---

## Summary

Independent help and rejection validation: only open/show/submit/resolve/merge survive; folded verbs, legacy pr, and manual provider/title/branch fields all reject without aliases.

## Command

```console
bash -lc 'set -euo pipefail; target/debug/atelier review --help; target/debug/atelier review open --help; for verb in status comments link comment approve request-changes; do if target/debug/atelier review "$verb" --help >/dev/null 2>&1; then echo "unexpected accepted verb: $verb"; exit 1; fi; echo "rejected verb: $verb"; done; for flag in title body source-branch target-branch provider owner repo host; do if target/debug/atelier review open --"$flag" x >/dev/null 2>&1; then echo "unexpected accepted flag: --$flag"; exit 1; fi; echo "rejected open flag: --$flag"; done; if target/debug/atelier pr --help >/dev/null 2>&1; then echo "unexpected accepted root: pr"; exit 1; fi; echo "rejected root: pr"'
```

Exit status: 0

## Stdout

Bytes: 1892
Truncated: no

```text
Configured review artifacts

Usage: atelier review [OPTIONS] <COMMAND>

Commands:
  open     Open or confirm the active review artifact for an issue owner
  show     Show linked review state and optionally its comments
  merge    Merge or confirm the linked review artifact without changing Atelier workflow state
  submit   Submit one comment, approval, or change request
  resolve  Resolve a native room finding
  help     Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
Open or confirm the active review artifact for an issue owner

Usage: atelier review open [OPTIONS]

Options:
      --issue <ISSUE>
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --role <ROLE>
      --existing <EXISTING>      Link an existing provider review instead of creating one
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
rejected verb: status
rejected verb: comments
rejected verb: link
rejected verb: comment
rejected verb: approve
rejected verb: request-changes
rejected open flag: --title
rejected open flag: --body
rejected open flag: --source-branch
rejected open flag: --target-branch
rejected open flag: --provider
rejected open flag: --owner
rejected open flag: --repo
rejected open flag: --host
rejected root: pr
```

## Stderr

Bytes: 0
Truncated: no

```text
```
