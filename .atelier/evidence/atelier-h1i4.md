---
created_at: "2026-11-09T21:12:24.994792147+00:00"
id: "atelier-h1i4"
evidence_type: "test"
commit: "56472e68d53c1f38d4ca05ad8baaed1aa3257ad3"
result: "pass"
captured_at: "2026-11-09T21:12:19.967151391+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test serve"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "atelier-uma8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uma8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Serve tests: inbox matches status and work human in alpha and beta, with hand-written mission links; fixture JSON current"
updated_at: "2026-11-09T21:12:25.135444295+00:00"
---

## Summary

Serve tests: inbox matches status and work human in alpha and beta, with hand-written mission links; fixture JSON current

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test serve
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 3720
SHA-256: 74815937e004471ee1230329dc676449ab4a25af8140868a76087246639b10c4
Test summary: Summary [   2.370s] 26 tests run: 26 passed, 1 skipped
