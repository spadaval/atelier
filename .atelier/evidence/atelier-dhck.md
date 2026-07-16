---
created_at: "2026-07-07T05:32:22.752150923+00:00"
id: "atelier-dhck"
evidence_type: "test"
captured_at: "2026-07-07T05:32:12.648592294+00:00"
command: "cargo run -q -p atelier-cli -- --quiet work missions"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-tdgs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tdgs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo run -q -p atelier-cli -- --quiet work missions"
updated_at: "2026-07-07T05:32:26.972225464+00:00"
---

## Summary

cargo run -q -p atelier-cli -- --quiet work missions

## Command

```console
cargo run -q -p atelier-cli -- --quiet work missions
```
Exit status: 0

## Stdout

Bytes: 26
Truncated: no

```text
atelier-c0mp
atelier-24xn
```

## Stderr

Bytes: 182
Truncated: no

```text
2026-07-07T05:32:22.459506Z  WARN Local cache application identity changed from 0 to 1096043602; rebuilt SQLite cache from /root/.codex/worktrees/e613/atelier-c0mp-overview/.atelier
```
