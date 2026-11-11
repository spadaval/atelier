---
created_at: "2026-11-11T08:38:35.339669416+00:00"
id: "atelier-11b9"
evidence_type: "test"
commit: "f47a181fee4b844ad3afb079ad6d3ff25d83c8c5"
result: "pass"
captured_at: "2026-11-11T08:38:18.541242801+00:00"
command: "bash -c 'cd desk && pnpm -s typecheck && pnpm -s test && pnpm -s build && pnpm -s e2e'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-va99"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-va99"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The app: typecheck, Vitest, build and Playwright at both widths"
updated_at: "2026-11-11T08:38:35.508848839+00:00"
---

## Summary

The app: typecheck, Vitest, build and Playwright at both widths

## Command

```console
bash -c 'cd desk && pnpm -s typecheck && pnpm -s test && pnpm -s build && pnpm -s e2e'
```

Exit status: 0

## Stdout

Bytes: 8307
SHA-256: a6dbf780f3d98abac7f0b979970b5f6424c6dec3f787c7031b264b300b5507d3

## Stderr

Bytes: 62
SHA-256: 219d2c79a7e7ac7cabb23ee44a765d40da86f4adb1c09d479527c44394318992
