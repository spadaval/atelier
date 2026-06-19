---
created_at: "2026-06-18T01:15:33.236403681+00:00"
id: "atelier-qzdi"
evidence_type: "validation"
captured_at: "2026-06-18T01:15:30.722136054+00:00"
command: "bash -lc 'set -euo pipefail\nrg -n \"schema_version: 2|pr_merged|linked_pr_merged\" .atelier/workflow.yaml\ntarget/debug/atelier workflow check\ntarget/debug/atelier lint atelier-jhzk\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-jhzk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jhzk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "repo workflow policy requires linked_pr_merged for epic closeout"
updated_at: "2026-06-18T01:15:37.234426786+00:00"
---

## Summary

repo workflow policy requires linked_pr_merged for epic closeout

## Command

```console
bash -lc 'set -euo pipefail
rg -n "schema_version: 2|pr_merged|linked_pr_merged" .atelier/workflow.yaml
target/debug/atelier workflow check
target/debug/atelier lint atelier-jhzk
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 392
Truncated: no

```text
2:schema_version: 2
55:  pr_merged:
56:    builtin: linked_pr_merged
119:          - pr_merged
Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Issue Types:    6
Statuses:       7
Validators:     8
Workflows:      3
Record Health:  pass
Issues Checked: 608
Docs/Help Drift: clear
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 0
Truncated: no

```text
```

