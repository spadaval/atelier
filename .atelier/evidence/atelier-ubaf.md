---
created_at: "2026-11-11T06:44:37.956151079+00:00"
id: "atelier-ubaf"
evidence_type: "test"
commit: "713765ab5d1041687f48f4fb92e75f44b49bbbf9"
result: "pass"
captured_at: "2026-11-11T06:44:28.882718807+00:00"
command: "bash -c 'cd desk && pnpm typecheck && pnpm test && pnpm build && pnpm e2e'"
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
title: "desk/: pnpm typecheck, pnpm test (22), pnpm build and pnpm e2e (20, desktop and phone) pass"
updated_at: "2026-11-11T06:44:38.121133286+00:00"
---

## Summary

desk/: pnpm typecheck, pnpm test (22), pnpm build and pnpm e2e (20, desktop and phone) pass

## Command

```console
bash -c 'cd desk && pnpm typecheck && pnpm test && pnpm build && pnpm e2e'
```

Exit status: 0

## Stdout

Bytes: 4118
SHA-256: e6ca052e8d6b652ce17400635c795b851c3d41713e09d69cee82b0285426904b

## Stderr

Bytes: 75
SHA-256: 29ccfb761c3b074fd162e3093209bb734be67ac6728cb0a61a8da41de1856754
