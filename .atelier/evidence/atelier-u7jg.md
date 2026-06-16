---
created_at: "2026-06-16T20:49:24.032481407+00:00"
id: "atelier-u7jg"
evidence_type: "validation"
captured_at: "2026-06-16T20:49:21.073014984+00:00"
command: "bash -lc '\nset -euo pipefail\ntarget/debug/atelier lint\ntarget/debug/atelier export --check\ntarget/debug/atelier workflow check\ngit status --short\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-9p3t"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9p3t"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Post-evidence tracker validation for closeout removal"
updated_at: "2026-06-16T20:49:27.805359537+00:00"
---

## Summary

Post-evidence tracker validation for closeout removal

## Command

```console
bash -lc '
set -euo pipefail
target/debug/atelier lint
target/debug/atelier export --check
target/debug/atelier workflow check
git status --short
'
```

Exit status: 0

## Stdout

Bytes: 559
Truncated: no

```text
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Issue Types:    6
Statuses:       7
Validators:     7
Workflows:      3
Record Health:  pass
Issues Checked: 556
Docs/Help Drift: clear
 M .atelier/issues/atelier-9p3t.md
?? .atelier/evidence/atelier-62bl.md
?? .atelier/evidence/atelier-awru.md
?? .atelier/evidence/atelier-imra.md
?? .atelier/evidence/atelier-ps58.md
?? .atelier/evidence/atelier-zewy.md
?? .atelier/issues/atelier-9p3t.activity/
```

## Stderr

Bytes: 0
Truncated: no

```text
```

