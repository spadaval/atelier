---
created_at: "2026-11-09T16:43:53.109409962+00:00"
id: "atelier-tt27"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:43:50.675274171+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve divergent_head"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Divergent-HEAD test: with the common directory's HEAD on a work branch with its own strategy, criteria, code and receipt, the served brief and debrief equal the CLI's views in a checkout of the base, generated_from is the base, and the receipt age counts only base commits; it failed with code_commits_since 2 instead of 1 when the HEAD lookup was put back"
updated_at: "2026-11-09T16:43:53.238781889+00:00"
---

## Summary

Divergent-HEAD test: with the common directory's HEAD on a work branch with its own strategy, criteria, code and receipt, the served brief and debrief equal the CLI's views in a checkout of the base, generated_from is the base, and the receipt age counts only base commits; it failed with code_commits_since 2 instead of 1 when the HEAD lookup was put back

## Command

```console
cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve divergent_head
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 582
SHA-256: 524a1ab706ff9f19d7dcde34f05ad259f643157d1b1a4ab95ff1cba09df552d3
Test summary: Summary [   0.291s] 1 test run: 1 passed, 14 skipped
