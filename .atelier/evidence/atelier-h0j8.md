---
created_at: "2026-11-09T18:40:09.444844117+00:00"
id: "atelier-h0j8"
evidence_type: "test"
commit: "fbd16d2958cd358a0ef7eebfe3755050b3ee835b"
result: "pass"
captured_at: "2026-11-09T18:39:59.716802805+00:00"
command: "bash -c 'cd crates/atelier-site/web && pnpm build && cd ../../.. && cargo build -j 4 -p atelier-cli && cd crates/atelier-site/web && pnpm e2e'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yn69"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yn69"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Playwright: every route of the fixture repositories at 390 px, no console error or horizontal scroll, not-found pages, asset content types and a missing-asset 404"
updated_at: "2026-11-09T18:40:09.581378473+00:00"
---

## Summary

Playwright: every route of the fixture repositories at 390 px, no console error or horizontal scroll, not-found pages, asset content types and a missing-asset 404

## Command

```console
bash -c 'cd crates/atelier-site/web && pnpm build && cd ../../.. && cargo build -j 4 -p atelier-cli && cd crates/atelier-site/web && pnpm e2e'
```

Exit status: 0

## Stdout

Bytes: 2451
SHA-256: 194114e33c5c557af0e9d65adb2b3d9417e5f5246d9edc766b108b1fe7b1c8f8

## Stderr

Bytes: 499
SHA-256: 5394355b9159bca885d551c19d64d366016c85522867887cb5c5230a85713410
