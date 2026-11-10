---
created_at: "2026-11-10T15:16:32.851278399+00:00"
id: "atelier-7xj9"
evidence_type: "validation"
commit: "7c0a86bccc0e429f7298b197d01d7d0ea060fa37"
result: "pass"
captured_at: "2026-11-10T15:16:32.632573532+00:00"
command: "sh -c 'python3 scripts/scorecard/counted.py --snapshot \"$1\" --out \"$1/out\" 2>\"$1/err\"; code=$?; cat \"$1/err\"; test $code -eq 1 && grep -q \"orchestration_v2 projections, as copied on 2026-11-10\" \"$1/err\"' sh /home/supadava/.cache/sc-tmp/badschema"
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
updated_at: "2026-11-10T15:16:32.989341154+00:00"
---

## Summary

counted.py given a database without the expected tables exits 1 naming the schema it expected

## Command

```console
sh -c 'python3 scripts/scorecard/counted.py --snapshot "$1" --out "$1/out" 2>"$1/err"; code=$?; cat "$1/err"; test $code -eq 1 && grep -q "orchestration_v2 projections, as copied on 2026-11-10" "$1/err"' sh /home/supadava/.cache/sc-tmp/badschema
```

Exit status: 0

## Stdout

Bytes: 1234
SHA-256: 48018cabf696160098df29effd6e225b44c1c54e90ec2b9f4929042b9b061153

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
