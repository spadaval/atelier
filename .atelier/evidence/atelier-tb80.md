---
created_at: "2026-07-09T15:29:18.301799509+00:00"
id: "atelier-tb80"
evidence_type: "test"
captured_at: "2026-07-09T15:29:15.403870504+00:00"
command: "bash -lc 'cargo fmt -- --check && target/debug/atelier check && git diff --check && git diff --check origin/master...HEAD && for c in 6467818b04aab8456a87252e0fce30b6da28b4f2 7c09cc0ddf9cde481df4e8deb169df5d1795ca90 4a7ddf67942a782406fd32ee360e246be68d0409 ccd4ea4feb74eb23c3f0498e1f87b9da945fe69b 4ba130d8941a13d24ee318444019b2dbf7777bef 312d792a265a18bf6916501e4526a5f10ab5f871; do git merge-base --is-ancestor \"$c\" HEAD; done && git merge-base --is-ancestor codex/atelier-p4z2-plan HEAD'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-gxq5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-gxq5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Formatting, tracker lint, whitespace, and preserved ancestry checks pass"
updated_at: "2026-07-09T15:29:18.305852614+00:00"
---

## Summary

Formatting, tracker lint, whitespace, and preserved ancestry checks pass

## Command

```console
bash -lc 'cargo fmt -- --check && target/debug/atelier check && git diff --check && git diff --check origin/master...HEAD && for c in 6467818b04aab8456a87252e0fce30b6da28b4f2 7c09cc0ddf9cde481df4e8deb169df5d1795ca90 4a7ddf67942a782406fd32ee360e246be68d0409 ccd4ea4feb74eb23c3f0498e1f87b9da945fe69b 4ba130d8941a13d24ee318444019b2dbf7777bef 312d792a265a18bf6916501e4526a5f10ab5f871; do git merge-base --is-ancestor "$c" HEAD; done && git merge-base --is-ancestor codex/atelier-p4z2-plan HEAD'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

