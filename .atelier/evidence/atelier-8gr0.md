---
created_at: "2026-06-20T21:08:57.140324377+00:00"
id: "atelier-8gr0"
evidence_type: "validation"
captured_at: "2026-06-20T21:08:56.745087879+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-show.txt\nrg -q \"atelier-439j|atelier-db6z|atelier-19xa|atelier-v2o6\" /tmp/atelier-4h62-show.txt\nrg -q \"issue create|issue status|issue link|issue transition\" docs/product/command-audit docs/product/cli-surface.md\necho \"mission collapse child graph and replacement docs are present\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ybz1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ybz1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-show.txt\nrg -q \"atelier-439j|atelier-db6z|atelier-19xa|atelier-v2o6\" /tmp/atelier-4h62-show.txt\nrg -q \"issue create|issue status|issue link|issue transition\" docs/product/command-audit docs/product/cli-surface.md\necho \"mission collapse child graph and replacement docs are present\"'"
updated_at: "2026-06-20T21:09:01.937540964+00:00"
---

## Summary

bash -lc 'set -euo pipefail
target/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-show.txt
rg -q "atelier-439j|atelier-db6z|atelier-19xa|atelier-v2o6" /tmp/atelier-4h62-show.txt
rg -q "issue create|issue status|issue link|issue transition" docs/product/command-audit docs/product/cli-surface.md
echo "mission collapse child graph and replacement docs are present"'

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier issue show atelier-4h62 > /tmp/atelier-4h62-show.txt
rg -q "atelier-439j|atelier-db6z|atelier-19xa|atelier-v2o6" /tmp/atelier-4h62-show.txt
rg -q "issue create|issue status|issue link|issue transition" docs/product/command-audit docs/product/cli-surface.md
echo "mission collapse child graph and replacement docs are present"'
```

Exit status: 0

## Stdout

Bytes: 62
Truncated: no

```text
mission collapse child graph and replacement docs are present
```

## Stderr

Bytes: 0
Truncated: no

```text
```

