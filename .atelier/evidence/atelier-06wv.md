---
created_at: "2026-11-09T20:25:33.146852506+00:00"
id: "atelier-06wv"
evidence_type: "test"
commit: "132006b728303cb90ff8815b499dd4388966856b"
result: "pass"
captured_at: "2026-11-09T20:25:22.443281067+00:00"
command: "bash -c 'cd crates/atelier-site/web && pnpm -s typecheck && pnpm -s test && pnpm -s build && cd ../../.. && cargo build -j 4 -q -p atelier-cli && cd crates/atelier-site/web && pnpm -s e2e'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ahpf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ahpf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "web typecheck, Vitest and Playwright at 390 px"
updated_at: "2026-11-09T20:25:33.282361095+00:00"
---

## Summary

web typecheck, Vitest and Playwright at 390 px

## Command

```console
bash -c 'cd crates/atelier-site/web && pnpm -s typecheck && pnpm -s test && pnpm -s build && cd ../../.. && cargo build -j 4 -q -p atelier-cli && cd crates/atelier-site/web && pnpm -s e2e'
```

Exit status: 0

## Stdout

Bytes: 3376
SHA-256: 55afceb6cdd7bf4a9d243189f711765232623d3ff9942feaf1223f92d5a8ab13

## Stderr

Bytes: 236
SHA-256: 23096fb28e2c8c2b41cec5b29a77093474b53b2a2ff07f5a37f83260b7016181
