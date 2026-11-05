---
created_at: "2026-11-05T02:26:59.778945165+00:00"
id: "atelier-3qiw"
evidence_type: "test"
commit: "b646d9d4c0240d54961858b05ed6648100332f07"
result: "pass"
captured_at: "2026-11-05T02:26:49.685364541+00:00"
command: "cargo nextest run -j 4 -p atelier-cli -E 'test(/debrief|brief/)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ov97"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ov97"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Debrief fixtures pass at b646d9d4, including a_done_childs_report_sits_with_its_child_line: a child closed with Deviations, Surprises and Open items under a done parent shows its Outcome and each part under its child line in Markdown and in the HTML entry, and the parent changes course"
updated_at: "2026-11-05T02:26:59.779467306+00:00"
---

## Summary

Debrief fixtures pass at b646d9d4, including a_done_childs_report_sits_with_its_child_line: a child closed with Deviations, Surprises and Open items under a done parent shows its Outcome and each part under its child line in Markdown and in the HTML entry, and the parent changes course

## Command

```console
cargo nextest run -j 4 -p atelier-cli -E 'test(/debrief|brief/)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 5638
SHA-256: f397344d94f5975a9ce0d5d964e2baeb1607cc88a7c343c37b8af577bff8678b
Test summary: Summary [   5.539s] 40 tests run: 40 passed, 712 skipped
