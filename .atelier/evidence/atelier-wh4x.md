---
created_at: "2026-10-23T16:08:55.077707206+00:00"
id: "atelier-wh4x"
evidence_type: "test"
commit: "4c448459e684ba27e9133b4912bc96a360da8ce2"
result: "pass"
captured_at: "2026-10-23T16:08:52.909023763+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_show_is_brief_and_full_prints_the_stored_record) | test(evidence_show_keeps_hostile_and_blank_failing_output_in_its_stream) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass)'"
exit_status: "0"
agent_identity: "claude-implementer-l7nz"
target:
  kind: "issue"
  id: "atelier-o8ni"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-o8ni"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "evidence show is brief (facts, standing, one tail excerpt, no empty fields) with --full printing every field and the stored record once; short IDs resolve"
updated_at: "2026-10-23T16:08:55.078383669+00:00"
---

## Summary

evidence show is brief (facts, standing, one tail excerpt, no empty fields) with --full printing every field and the stored record once; short IDs resolve

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(evidence_show_is_brief_and_full_prints_the_stored_record) | test(evidence_show_keeps_hostile_and_blank_failing_output_in_its_stream) | test(evidence_capture_keeps_the_failure_tail_and_summarizes_a_pass)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 870
SHA-256: 5397f1af74721f9f3c39e66f3d38761cd5f7a14967c874a014233506f7bbc16b
Test summary: Summary [   0.171s] 3 tests run: 3 passed, 705 skipped
