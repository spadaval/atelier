---
created_at: "2026-10-23T15:47:34.149245582+00:00"
id: "atelier-d0yh"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:31.141578596+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(merge_check_on_a_diverged_base_also_reports_the_review_and_claim_gates) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates) | test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-kiry"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kiry"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "merge --check on a diverged base also reports review and claim gates; uncommitted records, dirty code and missing approval reported in one land/merge run, fixes run verbatim"
updated_at: "2026-10-23T15:47:34.149547595+00:00"
---

## Summary

merge --check on a diverged base also reports review and claim gates; uncommitted records, dirty code and missing approval reported in one land/merge run, fixes run verbatim

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(merge_check_on_a_diverged_base_also_reports_the_review_and_claim_gates) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates) | test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 832
SHA-256: b07acce2dc599a66f9bc6c27aa420a814fdf542632c909bf6b2934edb0c7c590
Test summary: Summary [   0.685s] 3 tests run: 3 passed, 713 skipped
