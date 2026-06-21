---
created_at: "2026-06-21T18:43:05.159357516+00:00"
id: "atelier-6bj0"
evidence_type: "validation"
captured_at: "2026-06-21T18:43:03.468860020+00:00"
command: "bash -lc 'cargo fmt -- --check && git diff --check && atelier lint && cargo nextest run -p atelier-workflow rejects_missing_issue_type_coverage rejects_duplicate_issue_type_coverage parses_custom_issue_type_registry'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-s43l"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo fmt -- --check && git diff --check && atelier lint && cargo nextest run -p atelier-workflow rejects_missing_issue_type_coverage rejects_duplicate_issue_type_coverage parses_custom_issue_type_registry'"
updated_at: "2026-06-21T18:43:05.159357516+00:00"
---

## Summary

bash -lc 'cargo fmt -- --check && git diff --check && atelier lint && cargo nextest run -p atelier-workflow rejects_missing_issue_type_coverage rejects_duplicate_issue_type_coverage parses_custom_issue_type_registry'

## Command

```console
bash -lc 'cargo fmt -- --check && git diff --check && atelier lint && cargo nextest run -p atelier-workflow rejects_missing_issue_type_coverage rejects_duplicate_issue_type_coverage parses_custom_issue_type_registry'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 618
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
────────────
 Nextest run ID eecedd59-29e3-4005-961b-2c5b021c7449 with nextest profile: default
    Starting 3 tests across 1 binary (28 tests skipped)
        PASS [   0.010s] (1/3) atelier-workflow tests::rejects_duplicate_issue_type_coverage
        PASS [   0.010s] (2/3) atelier-workflow tests::parses_custom_issue_type_registry
        PASS [   0.011s] (3/3) atelier-workflow tests::rejects_missing_issue_type_coverage
────────────
     Summary [   0.011s] 3 tests run: 3 passed, 28 skipped
```

