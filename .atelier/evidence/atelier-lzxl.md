---
created_at: "2026-11-10T14:55:33.350623983+00:00"
id: "atelier-lzxl"
evidence_type: "validation"
commit: "b975f92cee07dc3b9f3a6290c5bb0d23893068dc"
result: "pass"
captured_at: "2026-11-10T14:55:29.134330060+00:00"
command: "bash -c 'd=$(mktemp -d -p \"$HOME/.cache\"); doc=\"$PWD/docs/architecture/quality/scorecard-baseline/defect-method.md\"; cd \"$d\" && awk \"/^\\`\\`\\`sh\\$/{n++; if(n==1){f=1; next}} /^\\`\\`\\`\\$/{f=0} f\" \"$doc\" > draw.sh && bash draw.sh && echo \"candidates: $(wc -l < candidates.txt)\" && grep -oE \"^\\| [0-9]+ \\| astromech-[a-z0-9]{4}\" \"$doc\" | awk \"{print \\$4}\" > table.txt && diff sample.txt table.txt && echo \"sample equals table order:\" && paste -sd\" \" sample.txt; rc=$?; rm -rf \"$d\"; exit $rc'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-z5bp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-z5bp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The draw commands in defect-method.md, run as written, give 52 candidates and the 30 listed in its table, in table order"
updated_at: "2026-11-10T14:55:33.493943265+00:00"
---

## Summary

The draw commands in defect-method.md, run as written, give 52 candidates and the 30 listed in its table, in table order

## Command

```console
bash -c 'd=$(mktemp -d -p "$HOME/.cache"); doc="$PWD/docs/architecture/quality/scorecard-baseline/defect-method.md"; cd "$d" && awk "/^\`\`\`sh\$/{n++; if(n==1){f=1; next}} /^\`\`\`\$/{f=0} f" "$doc" > draw.sh && bash draw.sh && echo "candidates: $(wc -l < candidates.txt)" && grep -oE "^\| [0-9]+ \| astromech-[a-z0-9]{4}" "$doc" | awk "{print \$4}" > table.txt && diff sample.txt table.txt && echo "sample equals table order:" && paste -sd" " sample.txt; rc=$?; rm -rf "$d"; exit $rc'
```

Exit status: 0

## Stdout

Bytes: 492
SHA-256: 966de23f16a7176a6451d67bf241256c2549b150ddad38db795337da1400c204

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
