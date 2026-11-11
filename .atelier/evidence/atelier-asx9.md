---
created_at: "2026-11-11T06:34:01.276096773+00:00"
id: "atelier-asx9"
evidence_type: "test"
commit: "29405f1f8a60da358da3d2038737298367ef4856"
result: "pass"
captured_at: "2026-11-11T06:33:51.931395717+00:00"
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
title: "desk/: pnpm typecheck, pnpm test (16), pnpm build and pnpm e2e (18, desktop and phone) pass"
updated_at: "2026-11-11T06:34:01.449774119+00:00"
---

## Summary

desk/: pnpm typecheck, pnpm test (16), pnpm build and pnpm e2e (18, desktop and phone) pass

## Command

```console
bash -c 'cd desk && pnpm typecheck && pnpm test && pnpm build && pnpm e2e'
```

Exit status: 0

## Stdout

Bytes: 3864
SHA-256: f44aa2d5ff137a7443649699767e2dfc0336ab6ac77d7c44c28d3ea29a798878

## Stderr

Bytes: 75
SHA-256: 29ccfb761c3b074fd162e3093209bb734be67ac6728cb0a61a8da41de1856754
