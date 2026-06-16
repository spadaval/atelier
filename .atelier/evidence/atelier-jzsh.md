---
created_at: "2026-06-15T18:51:55.468730434+00:00"
id: "atelier-jzsh"
evidence_type: "validation"
captured_at: "2026-06-15T18:51:24.194796039+00:00"
command: "bash -lc 'target/debug/atelier --help >/tmp/zwna-help.txt; target/debug/atelier status >/tmp/zwna-status.txt; target/debug/atelier issue list --ready >/tmp/zwna-ready.txt; target/debug/atelier mission status atelier-v5nb >/tmp/zwna-mission.txt; target/debug/atelier doctor >/tmp/zwna-doctor.txt; target/debug/atelier export --check >/tmp/zwna-export.txt; wc -l /tmp/zwna-help.txt /tmp/zwna-status.txt /tmp/zwna-ready.txt /tmp/zwna-mission.txt /tmp/zwna-doctor.txt /tmp/zwna-export.txt'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zwna"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zwna"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Representative CLI smoke commands for help status issue mission doctor and export remain stable"
updated_at: "2026-06-15T18:51:59.011073430+00:00"
---

## Summary

Representative CLI smoke commands for help status issue mission doctor and export remain stable

## Command

```console
bash -lc 'target/debug/atelier --help >/tmp/zwna-help.txt; target/debug/atelier status >/tmp/zwna-status.txt; target/debug/atelier issue list --ready >/tmp/zwna-ready.txt; target/debug/atelier mission status atelier-v5nb >/tmp/zwna-mission.txt; target/debug/atelier doctor >/tmp/zwna-doctor.txt; target/debug/atelier export --check >/tmp/zwna-export.txt; wc -l /tmp/zwna-help.txt /tmp/zwna-status.txt /tmp/zwna-ready.txt /tmp/zwna-mission.txt /tmp/zwna-doctor.txt /tmp/zwna-export.txt'
```

Exit status: 0

## Stdout

Bytes: 165
Truncated: no

```text
  69 /tmp/zwna-help.txt
  29 /tmp/zwna-status.txt
  10 /tmp/zwna-ready.txt
  93 /tmp/zwna-mission.txt
  28 /tmp/zwna-doctor.txt
   2 /tmp/zwna-export.txt
 231 total
```

## Stderr

Bytes: 122
Truncated: no

```text
2026-06-15T18:51:30.629850Z  WARN Projection index was stale; rebuilt local SQLite projection from /root/atelier/.atelier
```
