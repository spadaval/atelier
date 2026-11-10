---
created_at: "2026-11-10T15:37:58.193479074+00:00"
id: "atelier-z5i5"
evidence_type: "validation"
commit: "41ec686905cc73fbd1600370bd65f95960843860"
result: "pass"
captured_at: "2026-11-10T15:36:00.368185928+00:00"
command: "sh -c 'rm -rf \"$TMPDIR/det1\" \"$TMPDIR/det2\"; for i in 1 2; do nice -n 19 ionice -c3 python3 scripts/scorecard/counted.py --snapshot \"$HOME/.local/share/atelier-baseline/2026-11\" --out \"$TMPDIR/det$i\" || exit 1; done; sha256sum \"$TMPDIR\"/det1/counted.json \"$TMPDIR\"/det2/counted.json docs/architecture/quality/scorecard-baseline/counted.json; cmp \"$TMPDIR/det1/counted.json\" \"$TMPDIR/det2/counted.json\" && cmp \"$TMPDIR/det1/counted.json\" docs/architecture/quality/scorecard-baseline/counted.json'"
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
title: "two runs on the baseline snapshot give byte-identical counted.json, equal to the committed file"
updated_at: "2026-11-10T15:37:58.334769425+00:00"
---

## Summary

two runs on the baseline snapshot give byte-identical counted.json, equal to the committed file

## Command

```console
sh -c 'rm -rf "$TMPDIR/det1" "$TMPDIR/det2"; for i in 1 2; do nice -n 19 ionice -c3 python3 scripts/scorecard/counted.py --snapshot "$HOME/.local/share/atelier-baseline/2026-11" --out "$TMPDIR/det$i" || exit 1; done; sha256sum "$TMPDIR"/det1/counted.json "$TMPDIR"/det2/counted.json docs/architecture/quality/scorecard-baseline/counted.json; cmp "$TMPDIR/det1/counted.json" "$TMPDIR/det2/counted.json" && cmp "$TMPDIR/det1/counted.json" docs/architecture/quality/scorecard-baseline/counted.json'
```

Exit status: 0

## Stdout

Bytes: 350
SHA-256: 78a7fad60c785aae1511d79916e2f4f448fdecc7ad8c782a68523fb7795981df

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
