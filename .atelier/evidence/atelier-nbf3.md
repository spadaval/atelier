---
created_at: "2026-11-11T01:10:14.851103938+00:00"
id: "atelier-nbf3"
evidence_type: "test"
commit: "38cae6b79be7c5194494f05dab8256fd06cb013c"
result: "pass"
captured_at: "2026-11-11T01:10:11.468371030+00:00"
command: "bash -c 'set -o pipefail; cd evals/agent-factory/scenarios/next-loop && NEXT_ATELIER=/home/supadava/.t3/worktrees/atelier/atelier-efsl/next/target/debug/atelier python3 -m unittest 2>&1 | tail -3'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-efsl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-efsl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next-loop grader tests with this branch's binary"
updated_at: "2026-11-11T01:10:15.008967743+00:00"
---

## Summary

next-loop grader tests with this branch's binary

## Command

```console
bash -c 'set -o pipefail; cd evals/agent-factory/scenarios/next-loop && NEXT_ATELIER=/home/supadava/.t3/worktrees/atelier/atelier-efsl/next/target/debug/atelier python3 -m unittest 2>&1 | tail -3'
```

Exit status: 0

## Stdout

Bytes: 27
SHA-256: 1de2fbda2b81c25e4df5e197ab84b62767c6cf49e0966fb92f60872889e5ef96

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
