---
created_at: "2026-11-10T16:22:09.995499449+00:00"
id: "atelier-q97t"
evidence_type: "validation"
commit: "e0e65ac147676bc4fbb143c682d0e561e7b6c1a7"
result: "pass"
captured_at: "2026-11-10T16:19:32.357922191+00:00"
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
updated_at: "2026-11-10T16:22:10.135180009+00:00"
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
SHA-256: 49eee613cd0aaec025e47a912e84543add396d90297f72e28465da63584de3b3

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
