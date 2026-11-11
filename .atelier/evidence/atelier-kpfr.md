---
created_at: "2026-11-11T07:15:30.448904959+00:00"
id: "atelier-kpfr"
evidence_type: "test"
commit: "2591288505e645142ea1be5952048170627b2705"
result: "pass"
captured_at: "2026-11-11T07:15:20.356910875+00:00"
command: "bash -c 'cd desk && pnpm install --frozen-lockfile >/dev/null && pnpm typecheck 2>&1 | tail -1 && pnpm test 2>&1 | grep -E \"Tests \" && pnpm build >/dev/null 2>&1 && DESK_E2E_PORT=4871 CI=1 pnpm e2e 2>&1 | tail -1'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3hl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3hl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "desk/: pnpm typecheck, pnpm test, pnpm build and pnpm e2e on the branch synced with master"
updated_at: "2026-11-11T07:15:30.622172200+00:00"
---

## Summary

desk/: pnpm typecheck, pnpm test, pnpm build and pnpm e2e on the branch synced with master

## Command

```console
bash -c 'cd desk && pnpm install --frozen-lockfile >/dev/null && pnpm typecheck 2>&1 | tail -1 && pnpm test 2>&1 | grep -E "Tests " && pnpm build >/dev/null 2>&1 && DESK_E2E_PORT=4871 CI=1 pnpm e2e 2>&1 | tail -1'
```

Exit status: 0

## Stdout

Bytes: 62
SHA-256: 99c203466b788cf8022f5baec7f59b4421cc5b10291cec6c2310e196a30061f4

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
