---
created_at: "2026-10-23T15:12:37.129380473+00:00"
id: "atelier-u4wr"
evidence_type: "test"
commit: "052b990459d71d425ebcdf3db6d1a65f9c870666"
result: "pass"
captured_at: "2026-10-23T15:12:32.809430206+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_)'"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-h1nc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h1nc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-3ibn/atelier-kq8v: every Next: fix printed by the land and merge --check refusal tests (review request, blocker start or unlink --role blocked_by, dispositions, supersede of an open child under a closed owner) runs verbatim in a fixture copy, exits 0 and changes state unless read-only"
updated_at: "2026-10-23T15:12:37.129874184+00:00"
---

## Summary

atelier-3ibn/atelier-kq8v: every Next: fix printed by the land and merge --check refusal tests (review request, blocker start or unlink --role blocked_by, dispositions, supersede of an open child under a closed owner) runs verbatim in a fixture copy, exits 0 and changes state unless read-only

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 850
SHA-256: f0b9b4cb1280071917d966dd2d38e61dfb4f3271e17285ed647c97d72002891b
Test summary: Summary [   2.307s] 3 tests run: 3 passed, 909 skipped
