---
created_at: "2026-11-11T05:15:13.043489886+00:00"
id: "atelier-7t03"
evidence_type: "validation"
commit: "b2e0f822dcdaf3c3546b57f5b9e6bd640a9ba17e"
result: "pass"
captured_at: "2026-11-11T05:15:12.430332324+00:00"
command: "sh -c 'cd ~/.cache/atelier-desk/clones/atelier && TMPDIR=$HOME/.cache/atelier-desk/tmp python3 /home/supadava/.t3/worktrees/atelier/atelier-1ie1/scripts/cutover/convert_records.py --verify 009cdab9fe6afbcec42e46137266282490a276d1 >/dev/null'"
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
title: "Converter --verify in the scratch clone matches a fresh conversion of 009cdab9 (exit status not masked by a pipe, unlike atelier-ffcb)"
updated_at: "2026-11-11T05:15:13.210364531+00:00"
---

## Summary

Converter --verify in the scratch clone matches a fresh conversion of 009cdab9 (exit status not masked by a pipe, unlike atelier-ffcb)

## Command

```console
sh -c 'cd ~/.cache/atelier-desk/clones/atelier && TMPDIR=$HOME/.cache/atelier-desk/tmp python3 /home/supadava/.t3/worktrees/atelier/atelier-1ie1/scripts/cutover/convert_records.py --verify 009cdab9fe6afbcec42e46137266282490a276d1 >/dev/null'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
