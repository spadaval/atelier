---
created_at: "2026-11-10T16:56:00.907293795+00:00"
id: "atelier-8ies"
evidence_type: "validation"
commit: "74f8450a9bc4ace627cb396b377084dbc490abc6"
result: "pass"
captured_at: "2026-11-10T16:53:39.048153426+00:00"
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
updated_at: "2026-11-10T16:56:01.062517644+00:00"
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
SHA-256: d7aa250d877a23b4b6221b9eb345325fc84012cb1c8a9c326a85ab608ebf53a6

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
