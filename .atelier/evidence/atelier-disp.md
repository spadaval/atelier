---
created_at: "2026-11-10T19:14:07.230998975+00:00"
id: "atelier-disp"
evidence_type: "test"
commit: "88863e85edce1cd566986063fe2bd56692b2e050"
result: "pass"
captured_at: "2026-11-10T19:14:04.828705827+00:00"
command: "bash -c 'cd evals/agent-factory/harness && python3 -m unittest 2>&1 | tail -2 && cd ../scenarios/next-loop && NEXT_ATELIER=/home/supadava/.cache/atelier-fcb7/67nn/bin-r3/atelier python3 -m unittest -v 2>&1 | tail -10'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-67nn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-67nn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "harness tests and the next-loop grader's fixtures, with NEXT_ATELIER set to the round-3 binary"
updated_at: "2026-11-10T19:14:07.373754987+00:00"
---

## Summary

harness tests and the next-loop grader's fixtures, with NEXT_ATELIER set to the round-3 binary

## Command

```console
bash -c 'cd evals/agent-factory/harness && python3 -m unittest 2>&1 | tail -2 && cd ../scenarios/next-loop && NEXT_ATELIER=/home/supadava/.cache/atelier-fcb7/67nn/bin-r3/atelier python3 -m unittest -v 2>&1 | tail -10'
```

Exit status: 0

## Stdout

Bytes: 640
SHA-256: a507e65b9454ba391110811e8c5266455737caac00646123d11d43df1ef33885

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
