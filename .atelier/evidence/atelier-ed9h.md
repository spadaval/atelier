---
created_at: "2026-11-10T14:45:59.380891997+00:00"
id: "atelier-ed9h"
evidence_type: "test"
commit: "e1c65365d841f88c19f0544a0a58f133e62ec67e"
result: "pass"
captured_at: "2026-11-10T14:45:44.718470291+00:00"
command: "bash -c 'nice -n 19 ionice -c3 python3 scripts/scorecard/freeze.py verify && ls -a $HOME/.local/share/atelier-baseline/2026-11/t3/'"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-3kr7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3kr7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The snapshot still matches its manifest after every read; no journal file beside the database"
updated_at: "2026-11-10T14:45:59.521320151+00:00"
---

## Summary

The snapshot still matches its manifest after every read; no journal file beside the database

## Command

```console
bash -c 'nice -n 19 ionice -c3 python3 scripts/scorecard/freeze.py verify && ls -a $HOME/.local/share/atelier-baseline/2026-11/t3/'
```

Exit status: 0

## Stdout

Bytes: 76
SHA-256: 7c235276aff3de0716543099159d6b9150d1ceaaca185100a3d72017f2c28e5d

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
