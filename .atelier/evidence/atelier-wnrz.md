---
created_at: "2026-11-10T14:49:00.487430084+00:00"
id: "atelier-wnrz"
evidence_type: "validation"
commit: "715d9637a990921a44b3fd7eba8f07d3d8a7ac78"
result: "pass"
captured_at: "2026-11-10T14:49:00.196004890+00:00"
command: "sh -c '! python3 scripts/scorecard/counted.py --snapshot \"$1\" --out \"$1/out\" 2>&1 | tee /dev/stderr | grep -q \"orchestration_v2 projections, as copied on 2026-11-10\" && exit 1; python3 scripts/scorecard/counted.py --snapshot \"$1\" --out \"$1/out\"; test $? -eq 1' sh /home/supadava/.cache/sc-tmp/badschema"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-5fsj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5fsj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "counted.py given a database without the expected tables exits 1 naming the schema it expected"
updated_at: "2026-11-10T14:49:00.632344379+00:00"
---

## Summary

counted.py given a database without the expected tables exits 1 naming the schema it expected

## Command

```console
sh -c '! python3 scripts/scorecard/counted.py --snapshot "$1" --out "$1/out" 2>&1 | tee /dev/stderr | grep -q "orchestration_v2 projections, as copied on 2026-11-10" && exit 1; python3 scripts/scorecard/counted.py --snapshot "$1" --out "$1/out"; test $? -eq 1' sh /home/supadava/.cache/sc-tmp/badschema
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 2468
SHA-256: 77c68b56374e2d38079bdbbd72cf5efde940549a247fbd25bb738e0a2811debf
