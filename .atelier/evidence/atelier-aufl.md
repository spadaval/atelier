---
created_at: "2026-06-20T21:12:22.315594482+00:00"
id: "atelier-aufl"
evidence_type: "validation"
captured_at: "2026-06-20T21:12:17.427554287+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-parent.txt\nrg -q \"12 total \\| status: done=12\" /tmp/atelier-4h62-parent.txt\ntarget/debug/atelier issue status atelier-e146 > /tmp/atelier-e146-status.txt\nrg -q \"Docs/Help Drift: clear\" /tmp/atelier-e146-status.txt\nif target/debug/atelier mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\necho \"mission-collapse epic children done; docs drift clear; mission namespace removed\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-4h62"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4h62"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-parent.txt\nrg -q \"12 total \\| status: done=12\" /tmp/atelier-4h62-parent.txt\ntarget/debug/atelier issue status atelier-e146 > /tmp/atelier-e146-status.txt\nrg -q \"Docs/Help Drift: clear\" /tmp/atelier-e146-status.txt\nif target/debug/atelier mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\necho \"mission-collapse epic children done; docs drift clear; mission namespace removed\"'"
updated_at: "2026-06-20T21:12:27.036659968+00:00"
---

## Summary

bash -lc 'set -euo pipefail
target/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-parent.txt
rg -q "12 total \| status: done=12" /tmp/atelier-4h62-parent.txt
target/debug/atelier issue status atelier-e146 > /tmp/atelier-e146-status.txt
rg -q "Docs/Help Drift: clear" /tmp/atelier-e146-status.txt
if target/debug/atelier mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then
  echo "mission namespace unexpectedly succeeded"
  exit 1
fi
echo "mission-collapse epic children done; docs drift clear; mission namespace removed"'

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-parent.txt
rg -q "12 total \| status: done=12" /tmp/atelier-4h62-parent.txt
target/debug/atelier issue status atelier-e146 > /tmp/atelier-e146-status.txt
rg -q "Docs/Help Drift: clear" /tmp/atelier-e146-status.txt
if target/debug/atelier mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then
  echo "mission namespace unexpectedly succeeded"
  exit 1
fi
echo "mission-collapse epic children done; docs drift clear; mission namespace removed"'
```

Exit status: 0

## Stdout

Bytes: 81
Truncated: no

```text
mission-collapse epic children done; docs drift clear; mission namespace removed
```

## Stderr

Bytes: 0
Truncated: no

```text
```

