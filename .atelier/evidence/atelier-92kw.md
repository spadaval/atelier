---
created_at: "2026-11-11T06:51:21.604764579+00:00"
id: "atelier-92kw"
evidence_type: "test"
commit: "c3de0519003dc73ba92bf501d3ee6436ed9e0b73"
result: "pass"
captured_at: "2026-11-11T06:51:12.722682345+00:00"
command: "bash -c 'cd desk && pnpm install --frozen-lockfile && pnpm typecheck && pnpm test && pnpm build && pnpm e2e'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-94pv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-94pv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After syncing master (666a3545): desk/ pnpm typecheck, pnpm test (22), pnpm build and pnpm e2e (20, desktop and phone) pass"
updated_at: "2026-11-11T06:51:21.769824240+00:00"
---

## Summary

After syncing master (666a3545): desk/ pnpm typecheck, pnpm test (22), pnpm build and pnpm e2e (20, desktop and phone) pass

## Command

```console
bash -c 'cd desk && pnpm install --frozen-lockfile && pnpm typecheck && pnpm test && pnpm build && pnpm e2e'
```

Exit status: 0

## Stdout

Bytes: 4263
SHA-256: 0931476d6842c2d087c05fd55dac329bfc75925fbc0ac5540eb84f231f0f8100

## Stderr

Bytes: 75
SHA-256: 29ccfb761c3b074fd162e3093209bb734be67ac6728cb0a61a8da41de1856754
