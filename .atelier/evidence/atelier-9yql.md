---
created_at: "2026-11-04T23:08:43.267146233+00:00"
id: "atelier-9yql"
evidence_type: "test"
commit: "c64cb4bd8e1ed97262592aa57c9961d66ea6d263"
result: "pass"
captured_at: "2026-11-04T23:08:39.876119116+00:00"
command: "cargo nextest run -p atelier-app path_log"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-cej5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cej5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "path_log tests pass locally with the date steps split; each split test ~3.1s, like matches_git_over_merges_octopus_and_criss_cross (was 7.6s combined). Linux only; macOS proof is the PR CI run."
updated_at: "2026-11-04T23:08:43.267604258+00:00"
---

## Summary

path_log tests pass locally with the date steps split; each split test ~3.1s, like matches_git_over_merges_octopus_and_criss_cross (was 7.6s combined). Linux only; macOS proof is the PR CI run.

## Command

```console
cargo nextest run -p atelier-app path_log
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1245
SHA-256: 604ede72597b49bde84a6cfc48d654028f52883297a14aebd67c8920df214870
Test summary: Summary [   3.053s] 9 tests run: 9 passed, 98 skipped
