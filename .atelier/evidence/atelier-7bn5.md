---
created_at: "2026-10-23T16:08:58.278567920+00:00"
id: "atelier-7bn5"
evidence_type: "test"
commit: "4c448459e684ba27e9133b4912bc96a360da8ce2"
result: "pass"
captured_at: "2026-10-23T16:08:56.088251464+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(evidence_attach_takes_the_target_forms_record_takes) | test(evidence_record_confirms_in_one_line_and_ends_in_its_verdict) | test(every_invalid_value_takes_the_shared_form_and_exits_2) | test(evidence_target_parser_takes_kind_id_or_a_bare_issue_id) | test(fixed_values_are_listed_and_others_are_usage_errors)'"
exit_status: "0"
agent_identity: "claude-implementer-l7nz"
target:
  kind: "issue"
  id: "atelier-b05y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-b05y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "evidence attach takes issue/<id>, bare and two-argument targets, --target takes a bare issue ID, unknown --kind warns and records, invalid targets use the shared form"
updated_at: "2026-10-23T16:08:58.279283747+00:00"
---

## Summary

evidence attach takes issue/<id>, bare and two-argument targets, --target takes a bare issue ID, unknown --kind warns and records, invalid targets use the shared form

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(evidence_attach_takes_the_target_forms_record_takes) | test(evidence_record_confirms_in_one_line_and_ends_in_its_verdict) | test(every_invalid_value_takes_the_shared_form_and_exits_2) | test(evidence_target_parser_takes_kind_id_or_a_bare_issue_id) | test(fixed_values_are_listed_and_others_are_usage_errors)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1084
SHA-256: 0184346fd0caecbf2ac40461c10134101822be72ed4f4995eec68cbf3b632fe7
Test summary: Summary [   0.215s] 5 tests run: 5 passed, 907 skipped
