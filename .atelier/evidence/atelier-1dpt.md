---
created_at: "2026-11-05T02:27:25.579994593+00:00"
id: "atelier-1dpt"
evidence_type: "validation"
commit: "b646d9d4c0240d54961858b05ed6648100332f07"
result: "pass"
captured_at: "2026-11-05T02:27:22.293828541+00:00"
command: "sh -c 'target/debug/atelier debrief atelier-r6bt | grep -A4 \"└ atelier-w5hx\" | grep -q \"Surprised by: .work decisions -q. also changed order\" && grep -q \"also changed order\" \"$(target/debug/atelier debrief atelier-r6bt --html -q)\"'"
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
title: "Debug binary at b646d9d4: 'debrief atelier-r6bt' prints atelier-w5hx's Surprises ('work decisions -q also changed order') and Left open under its child line beneath atelier-2bjz, and the --html page holds the same Surprises text"
updated_at: "2026-11-05T02:27:25.580556599+00:00"
---

## Summary

Debug binary at b646d9d4: 'debrief atelier-r6bt' prints atelier-w5hx's Surprises ('work decisions -q also changed order') and Left open under its child line beneath atelier-2bjz, and the --html page holds the same Surprises text

## Command

```console
sh -c 'target/debug/atelier debrief atelier-r6bt | grep -A4 "└ atelier-w5hx" | grep -q "Surprised by: .work decisions -q. also changed order" && grep -q "also changed order" "$(target/debug/atelier debrief atelier-r6bt --html -q)"'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
