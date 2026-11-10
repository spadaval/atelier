---
created_at: "2026-11-10T17:16:40.297123876+00:00"
id: "atelier-p0c9"
evidence_type: "validation"
commit: "e883e09633522360fbd347d70d19e92e538a8ade"
result: "pass"
captured_at: "2026-11-10T17:16:40.134891837+00:00"
command: "bash -c 'set -e; awk \"/^## Scorecard/,/^The Before column/\" docs/strategies/redesign-product.md | grep \"^|\" | awk -F\"|\" \"{print NF-2}\" | sort | uniq -c; diff <(git show master:docs/strategies/redesign-product.md | awk \"/^## Scorecard/,/^The codebase health/\" | grep \"^|\" | cut -d\"|\" -f2,3) <(awk \"/^## Scorecard/,/^The Before column/\" docs/strategies/redesign-product.md | grep \"^|\" | cut -d\"|\" -f2,3) && echo group-and-measure-unchanged; git diff --check master...HEAD && echo diff-check-clean'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-6sti"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6sti"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Scorecard table in redesign-product.md: 14 table lines (header, separator, 12 rows), each with 3 columns; its Group and Measure columns are identical to master's, so only Before cells changed; git diff --check is clean."
updated_at: "2026-11-10T17:16:40.439848420+00:00"
---

## Summary

Scorecard table in redesign-product.md: 14 table lines (header, separator, 12 rows), each with 3 columns; its Group and Measure columns are identical to master's, so only Before cells changed; git diff --check is clean.

## Command

```console
bash -c 'set -e; awk "/^## Scorecard/,/^The Before column/" docs/strategies/redesign-product.md | grep "^|" | awk -F"|" "{print NF-2}" | sort | uniq -c; diff <(git show master:docs/strategies/redesign-product.md | awk "/^## Scorecard/,/^The codebase health/" | grep "^|" | cut -d"|" -f2,3) <(awk "/^## Scorecard/,/^The Before column/" docs/strategies/redesign-product.md | grep "^|" | cut -d"|" -f2,3) && echo group-and-measure-unchanged; git diff --check master...HEAD && echo diff-check-clean'
```

Exit status: 0

## Stdout

Bytes: 55
SHA-256: d9b7ca45c6eb3603eb939b234e376fcc009a7d66cc3b12ae12d4359e4333dae8

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
