---
created_at: "2026-06-13T15:52:51.330135312+00:00"
id: "atelier-f2yn"
evidence_type: "validation"
captured_at: "2026-06-13T15:52:51.062647227+00:00"
command: "sh -c 'target/debug/atelier issue show atelier-19mc 2>&1 | grep -F '\"'\"'atelier-19mc is a mission record, not an issue record'\"'\"' | grep -F '\"'\"'atelier mission show atelier-19mc'\"'\"' && target/debug/atelier issue show atelier-nope 2>&1 | grep -F '\"'\"'Issue atelier-nope was not found'\"'\"''"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ktcm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "wrong-kind issue command transcript and focused tests pass"
updated_at: "2026-06-13T15:52:53.149173623+00:00"
---

wrong-kind issue command transcript and focused tests pass

Command: sh -c 'target/debug/atelier issue show atelier-19mc 2>&1 | grep -F '"'"'atelier-19mc is a mission record, not an issue record'"'"' | grep -F '"'"'atelier mission show atelier-19mc'"'"' && target/debug/atelier issue show atelier-nope 2>&1 | grep -F '"'"'Issue atelier-nope was not found'"'"''
Exit status: 0

Stdout summary:
Error: atelier-19mc is a mission record, not an issue record. Use `atelier mission show atelier-19mc`.
Error: Issue atelier-nope was not found

Stderr summary:
(none)

