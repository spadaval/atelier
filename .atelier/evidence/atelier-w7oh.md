---
created_at: "2026-11-04T23:27:11.566717412+00:00"
id: "atelier-w7oh"
evidence_type: "test"
commit: "60b995d1ec3dec7057308fff5ac613580da137d7"
result: "pass"
captured_at: "2026-11-04T23:27:10.361965745+00:00"
command: "sh -c '! target/debug/atelier issue note --help | grep -q -- \"--one-way\" && { target/debug/atelier issue note atelier-w5hx x --kind decision --reason r --one-way; test $? -eq 2; } && test -z \"$(git grep -n -i \"one-way\\|one_way\\|oneway\" crates docs/product)\" && target/debug/atelier check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-w5hx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-w5hx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "issue note --help has no --one-way, passing it is a clap usage error (exit 2), the grep over crates and docs/product prints nothing, and atelier check passes with the migrated notes"
updated_at: "2026-11-04T23:27:11.567046206+00:00"
---

## Summary

issue note --help has no --one-way, passing it is a clap usage error (exit 2), the grep over crates and docs/product prints nothing, and atelier check passes with the migrated notes

## Command

```console
sh -c '! target/debug/atelier issue note --help | grep -q -- "--one-way" && { target/debug/atelier issue note atelier-w5hx x --kind decision --reason r --one-way; test $? -eq 2; } && test -z "$(git grep -n -i "one-way\|one_way\|oneway" crates docs/product)" && target/debug/atelier check'
```

Exit status: 0

## Stdout

Bytes: 14
SHA-256: 5ea3019fc3285e78b922a880508210d7feacc2c23bd53142d5f40b996d9fc2e4

## Stderr

Bytes: 212
SHA-256: 0b03619a340eda599f15492d3b3b3cbbd6252e97df0cbd3ffb2f29fb775f3c27
