---
created_at: "2026-11-09T17:04:06.857831364+00:00"
id: "atelier-epci"
evidence_type: "validation"
commit: "594dc0f8fc5fd3a5955733f4aa4834cb3a47ce98"
result: "pass"
captured_at: "2026-11-09T17:04:04.125824863+00:00"
command: "/home/supadava/.cache/atelier-tmp-zpmb/load.sh /home/supadava/.cache/atelier-tmp-zpmb/atelier-release atelier atelier-k8h8"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After the round 1 fixes, release build: 20 concurrent cold-cache debrief requests for atelier-k8h8 all answer 200, peak RSS under 300 MB, T3 Code under 1 s throughout, warm under 100 ms"
updated_at: "2026-11-09T17:04:06.987672924+00:00"
---

## Summary

After the round 1 fixes, release build: 20 concurrent cold-cache debrief requests for atelier-k8h8 all answer 200, peak RSS under 300 MB, T3 Code under 1 s throughout, warm under 100 ms

## Command

```console
/home/supadava/.cache/atelier-tmp-zpmb/load.sh /home/supadava/.cache/atelier-tmp-zpmb/atelier-release atelier atelier-k8h8
```

Exit status: 0

## Stdout

Bytes: 434
SHA-256: 35f4bdbb0dcc11cc69c6ee696a4d698a96018167626f0558d58aa94ff84dfe52

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
