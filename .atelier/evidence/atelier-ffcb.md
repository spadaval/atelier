---
created_at: "2026-11-11T05:15:06.540624604+00:00"
id: "atelier-ffcb"
evidence_type: "validation"
commit: "e2b073fb78ffc59dc5dfacfbc776027102f6095a"
result: "pass"
captured_at: "2026-11-11T05:15:05.922239130+00:00"
command: "sh -c 'cd ~/.cache/atelier-desk/clones/atelier && TMPDIR=$HOME/.cache/atelier-desk/tmp python3 /home/supadava/.t3/worktrees/atelier/atelier-1ie1/scripts/cutover/convert_records.py --verify 009cdab9fe6afbcec42e46137266282490a276d1 | tail -1'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-1ie1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1ie1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Converter --verify in the scratch clone matches a fresh conversion of 009cdab9"
updated_at: "2026-11-11T05:15:06.704905831+00:00"
---

## Summary

Converter --verify in the scratch clone matches a fresh conversion of 009cdab9

## Command

```console
sh -c 'cd ~/.cache/atelier-desk/clones/atelier && TMPDIR=$HOME/.cache/atelier-desk/tmp python3 /home/supadava/.t3/worktrees/atelier/atelier-1ie1/scripts/cutover/convert_records.py --verify 009cdab9fe6afbcec42e46137266282490a276d1 | tail -1'
```

Exit status: 0

## Stdout

Bytes: 119
SHA-256: 25aca1047d74aa45f917ada3f2c57e073d090ac4d4581d020cb31f1feb69fafb

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
