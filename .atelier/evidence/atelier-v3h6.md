---
created_at: "2026-06-21T18:50:13.607473557+00:00"
id: "atelier-v3h6"
evidence_type: "validation"
captured_at: "2026-06-21T18:50:06.831048683+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier status | rg \"atelier issue table --kind mission\"\ntarget/debug/atelier issue table --kind mission | rg \"Issue Table: mission|atelier issue status <id>\"\ntarget/debug/atelier issue list --ready | rg \"\\[epic\\] atelier-vays|ready \\[feature\\] atelier-62po\"\ntarget/debug/atelier issue status atelier-53bu | rg \"Mission Status atelier-53bu|Selectable Work|Terminal Checks\"\ntarget/debug/atelier issue status --help | rg \"Usage: atelier issue status \\[OPTIONS\\] <ID>\"\nif target/debug/atelier issue status >/tmp/e7t1-no-id-status-clean.out 2>&1; then\n  echo \"no-ID issue status unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg \"Usage: atelier issue status <ID>\" /tmp/e7t1-no-id-status-clean.out\n! rg -F \"Default first command: \\`atelier issue status\\`\" docs .agents/skills/agent-factory crates/atelier-cli/src crates/atelier-cli/tests\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-e7t1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e7t1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Clean command-surface status/table/help/docs search"
updated_at: "2026-06-21T18:50:19.552479661+00:00"
---

## Summary

Clean command-surface status/table/help/docs search

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier status | rg "atelier issue table --kind mission"
target/debug/atelier issue table --kind mission | rg "Issue Table: mission|atelier issue status <id>"
target/debug/atelier issue list --ready | rg "\[epic\] atelier-vays|ready \[feature\] atelier-62po"
target/debug/atelier issue status atelier-53bu | rg "Mission Status atelier-53bu|Selectable Work|Terminal Checks"
target/debug/atelier issue status --help | rg "Usage: atelier issue status \[OPTIONS\] <ID>"
if target/debug/atelier issue status >/tmp/e7t1-no-id-status-clean.out 2>&1; then
  echo "no-ID issue status unexpectedly succeeded" >&2
  exit 1
fi
rg "Usage: atelier issue status <ID>" /tmp/e7t1-no-id-status-clean.out
! rg -F "Default first command: \`atelier issue status\`" docs .agents/skills/agent-factory crates/atelier-cli/src crates/atelier-cli/tests
'
```

Exit status: 0

## Stdout

Bytes: 561
Truncated: no

```text
  Inspect mission choices (4 current mission(s), none active): atelier issue table --kind mission
Issue Table: mission
  Inspect one objective: atelier issue status <id>
[epic] atelier-vays high - Epic: Collapse mission command surface into issue commands (context; parent blocked)
    ready [feature] atelier-62po - Make issue show/status own mission objective views
Mission Status atelier-53bu [ready] - Make workflow obligations explicit and minimal
Selectable Work
Terminal Checks
Usage: atelier issue status [OPTIONS] <ID>
Usage: atelier issue status <ID>
```

## Stderr

Bytes: 0
Truncated: no

```text
```

