---
created_at: "2026-10-23T15:34:11.337876020+00:00"
id: "atelier-zx5z"
evidence_type: "test"
commit: "9f4d7ee0478cb378cac9b7072ff2de41ba86b223"
result: "pass"
captured_at: "2026-10-23T15:34:09.323801271+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli --test merge_gates sync_rerun_receipt_summary_is_the_old_summary_with_the_rerun_suffix"
exit_status: "0"
agent_identity: "claude-implementer-imlh"
target:
  kind: "issue"
  id: "atelier-3xq4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3xq4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "a merge --sync rerun receipt's Summary is the old Summary text plus the rerun suffix, with one Command and one Stdout section"
updated_at: "2026-10-23T15:34:11.338137418+00:00"
---

## Summary

a merge --sync rerun receipt's Summary is the old Summary text plus the rerun suffix, with one Command and one Stdout section

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli --test merge_gates sync_rerun_receipt_summary_is_the_old_summary_with_the_rerun_suffix
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 568
SHA-256: 6d6cf6897f2e02535e469b7ea3e316c2a0914fca175ce8692daff3ee986023a1
Test summary: Summary [   0.322s] 1 test run: 1 passed, 80 skipped
