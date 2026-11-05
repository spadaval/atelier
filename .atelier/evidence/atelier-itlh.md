---
created_at: "2026-11-05T01:23:12.398922168+00:00"
id: "atelier-itlh"
evidence_type: "test"
commit: "14293fddb0032f0c5d1158908881cb570404572b"
result: "pass"
captured_at: "2026-11-05T01:23:09.950742011+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test cli_integration reports::"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ypks"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ypks"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Report tests: a no-flag reclose after a hand-edited reopen prints the kept Outcome; supersede --reason drops a report written for a close; existing report tests still pass"
updated_at: "2026-11-05T01:23:12.399648033+00:00"
---

## Summary

Report tests: a no-flag reclose after a hand-edited reopen prints the kept Outcome; supersede --reason drops a report written for a close; existing report tests still pass

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test cli_integration reports::
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1198
SHA-256: 6687c237568efedc49c040dea457f23dca94dc7dba78539a309e8fe11f7b97ed
Test summary: Summary [   0.133s] 6 tests run: 6 passed, 392 skipped
