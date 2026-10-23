---
created_at: "2026-10-23T15:53:48.737828562+00:00"
id: "atelier-4iuq"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:53:45.978432624+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(merge_check_on_a_diverged_base_also_reports_the_review_and_claim_gates) | test(merge_check_names_every_uncommitted_record_and_path_and_still_runs_the_gates) | test(land_reports_uncommitted_records_dirty_paths_and_the_gates_in_one_run)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "merge --check on a diverged base also reports review and claim gates; uncommitted records, dirty code and missing approval reported in one land/merge run, fixes run verbatim (rerun after merging master)"
updated_at: "2026-10-23T15:53:48.738700801+00:00"
---

## Summary

merge --check on a diverged base also reports review and claim gates; uncommitted records, dirty code and missing approval reported in one land/merge run, fixes run verbatim (rerun after merging master)

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
SHA-256: 6767936f7a294811513fe1305c161c7223ef934cc500c75b3a29b88b063846d6
Test summary: Summary [   0.662s] 3 tests run: 3 passed, 719 skipped
