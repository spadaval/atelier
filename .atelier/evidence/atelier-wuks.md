---
created_at: "2026-11-10T15:43:51.313485451+00:00"
id: "atelier-wuks"
evidence_type: "validation"
commit: "f086907408be403eaf4542feaac87d67a1183666"
result: "pass"
captured_at: "2026-11-10T15:43:51.152686382+00:00"
command: "python3 -I /home/supadava/.cache/c71h/verify-totals.py docs/architecture/quality/scorecard-baseline"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-c71h"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c71h"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Outcome counts recounted from the trace tables (defects.md plus defect-method.md's 30 rows with row 25 moved) equal the summary: 130 rows, 33 after merge, 25 before merge, 20 before adoption, 12 no item, 40 not traceable"
updated_at: "2026-11-10T15:43:51.452182158+00:00"
---

## Summary

Outcome counts recounted from the trace tables (defects.md plus defect-method.md's 30 rows with row 25 moved) equal the summary: 130 rows, 33 after merge, 25 before merge, 20 before adoption, 12 no item, 40 not traceable

## Command

```console
python3 -I /home/supadava/.cache/c71h/verify-totals.py docs/architecture/quality/scorecard-baseline
```

Exit status: 0

## Stdout

Bytes: 453
SHA-256: 776cb90c3bad8340af70c3c5dd095bc22e82701bcb73d6825e9ca32d30337fb3

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
