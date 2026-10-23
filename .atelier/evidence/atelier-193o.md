---
created_at: "2026-10-23T16:08:48.335921305+00:00"
id: "atelier-193o"
evidence_type: "test"
commit: "4c448459e684ba27e9133b4912bc96a360da8ce2"
result: "pass"
captured_at: "2026-10-23T16:08:46.136715044+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_record_confirms_in_one_line_and_ends_in_its_verdict) | test(test_evidence_capture_records_nonzero_exit_as_command_metadata) | test(test_evidence_quiet_output_is_a_stable_id_composition_path)'"
exit_status: "0"
agent_identity: "claude-implementer-l7nz"
target:
  kind: "issue"
  id: "atelier-3ekh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3ekh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "evidence record pass, fail (exit 0, last 5 lines, verdict naming the item), manual, --commit-records and quiet confirmations are pinned"
updated_at: "2026-10-23T16:08:48.336588320+00:00"
---

## Summary

evidence record pass, fail (exit 0, last 5 lines, verdict naming the item), manual, --commit-records and quiet confirmations are pinned

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_record_confirms_in_one_line_and_ends_in_its_verdict) | test(test_evidence_capture_records_nonzero_exit_as_command_metadata) | test(test_evidence_quiet_output_is_a_stable_id_composition_path)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 867
SHA-256: 6f8a46f05b9a5dfeb42146d43a28ad169c6221404e6adfb9cd731a4279fb3651
Test summary: Summary [   0.216s] 3 tests run: 3 passed, 705 skipped
