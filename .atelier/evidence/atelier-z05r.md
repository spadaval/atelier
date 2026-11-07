---
created_at: "2026-11-07T02:52:33.298215506+00:00"
id: "atelier-z05r"
evidence_type: "test"
commit: "ff7ffadf3096be3532ca30bccb1d515c1a8ae47c"
result: "pass"
captured_at: "2026-11-07T02:52:18.963015365+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test lifecycle_cost"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-n4mj"
  role: "validates"
criteria:
- slug: "lifecycle-cost"
  result: "pass"
criteria_mission: "atelier-n4mj"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n4mj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "lifecycle-cost: the scenario test asserts 1 review file per finding (and reports 13 record-only commits as a measurement)"
updated_at: "2026-11-07T02:52:33.298498073+00:00"
---

## Summary

lifecycle-cost: the scenario test asserts 1 review file per finding (and reports 13 record-only commits as a measurement)

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test lifecycle_cost
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 2778
SHA-256: 80e50c8b0c106404e73a025a1c30a2e16b578c68f269210c02f1075bd30f34b9
Test summary: Summary [   0.817s] 1 test run: 1 passed, 0 skipped
