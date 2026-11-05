---
created_at: "2026-11-04T23:35:43.816685855+00:00"
id: "atelier-m2ht"
evidence_type: "check"
commit: "d4a6c1bc15bf31ee768d5162bb5093c6db4361a6"
result: "pass"
captured_at: "2026-11-04T23:35:43.404979238+00:00"
command: "sh -c 'dist generate >/dev/null 2>&1 && git diff --exit-code -- .github/workflows/release.yml dist-workspace.toml'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-l7gl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l7gl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "dist generate at the pinned cargo-dist 0.33.1 leaves release.yml unchanged after the switch to pr-run-mode = plan (no hand edits)"
updated_at: "2026-11-04T23:35:43.817038824+00:00"
---

## Summary

dist generate at the pinned cargo-dist 0.33.1 leaves release.yml unchanged after the switch to pr-run-mode = plan (no hand edits)

## Command

```console
sh -c 'dist generate >/dev/null 2>&1 && git diff --exit-code -- .github/workflows/release.yml dist-workspace.toml'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
