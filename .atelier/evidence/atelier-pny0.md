---
created_at: "2026-11-10T05:20:53.657416638+00:00"
id: "atelier-pny0"
evidence_type: "validation"
commit: "2dfeef82d317b0bb97a63633bc74feb58157d392"
result: "pass"
captured_at: "2026-11-10T05:20:53.493479420+00:00"
command: "sh -c 'git ls-files | grep -E '\"'\"'atelier-baseline|\\.sqlite$|\\.jsonl$'\"'\"' | wc -l; stat -c '\"'\"'%a %n'\"'\"' /home/supadava/.local/share/atelier-baseline /home/supadava/.local/share/atelier-baseline/2026-11 /home/supadava/.local/share/atelier-baseline/2026-11/* /home/supadava/.local/share/atelier-baseline/2026-11/t3/statev2.sqlite; echo group-or-world-accessible: $(find /home/supadava/.local/share/atelier-baseline -perm /077 | wc -l)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-scx2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-scx2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Nothing from the snapshot is tracked; snapshot directories are 700 and no path is group or world accessible"
updated_at: "2026-11-10T05:20:53.797595106+00:00"
---

## Summary

Nothing from the snapshot is tracked; snapshot directories are 700 and no path is group or world accessible

## Command

```console
sh -c 'git ls-files | grep -E '"'"'atelier-baseline|\.sqlite$|\.jsonl$'"'"' | wc -l; stat -c '"'"'%a %n'"'"' /home/supadava/.local/share/atelier-baseline /home/supadava/.local/share/atelier-baseline/2026-11 /home/supadava/.local/share/atelier-baseline/2026-11/* /home/supadava/.local/share/atelier-baseline/2026-11/t3/statev2.sqlite; echo group-or-world-accessible: $(find /home/supadava/.local/share/atelier-baseline -perm /077 | wc -l)'
```

Exit status: 0

## Stdout

Bytes: 417
SHA-256: 9acf17ff2f53c726fbb268fccbad533d01b0febde51bf9d44159d5e122a6365a

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
