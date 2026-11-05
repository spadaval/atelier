---
created_at: "2026-11-05T01:23:16.421624364+00:00"
id: "atelier-gyqe"
evidence_type: "test"
commit: "14293fddb0032f0c5d1158908881cb570404572b"
result: "pass"
captured_at: "2026-11-05T01:23:14.055654830+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session report_parts a_branch_cannot_weaken"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-4vda"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4vda"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Session report-part tests: a claim one commit back on a base with no upstream must carry its parts; earlier-claim, scoped-child and weakened-policy cases still hold"
updated_at: "2026-11-05T01:23:16.422288925+00:00"
---

## Summary

Session report-part tests: a claim one commit back on a base with no upstream must carry its parts; earlier-claim, scoped-child and weakened-policy cases still hold

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session report_parts a_branch_cannot_weaken
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 914
SHA-256: aa941d39b25ba4ee647ba20155f5f626048ec643d92921d9e2a3f1bee9b9fad1
Test summary: Summary [   0.359s] 4 tests run: 4 passed, 132 skipped
