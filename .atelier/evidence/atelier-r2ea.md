---
created_at: "2026-10-28T02:16:48.211684769+00:00"
id: "atelier-r2ea"
evidence_type: "test"
commit: "9dfcaccbf99308df440da029d47a86b6013cb749"
result: "pass"
captured_at: "2026-10-28T02:16:45.779321485+00:00"
command: "env TMPDIR=/home/supadava/.cache/atelier-tmp RUST_BACKTRACE=0 cargo nextest run -j 4 -E 'test(/schema_version|schema_newer|newer_atelier|newer/)'"
exit_status: "0"
agent_identity: "Claude Opus 5.5 (orchestrator)"
target:
  kind: "issue"
  id: "atelier-5tl2"
  role: "validates"
criteria:
- slug: "S4"
  result: "pass"
criteria_mission: "atelier-vt8q"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5tl2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "env TMPDIR=/home/supadava/.cache/atelier-tmp RUST_BACKTRACE=0 cargo nextest run -j 4 -E 'test(/schema_version|schema_newer|newer_atelier|newer/)'"
updated_at: "2026-10-28T02:16:48.211968038+00:00"
---

## Summary

env TMPDIR=/home/supadava/.cache/atelier-tmp RUST_BACKTRACE=0 cargo nextest run -j 4 -E 'test(/schema_version|schema_newer|newer_atelier|newer/)'

## Command

```console
env TMPDIR=/home/supadava/.cache/atelier-tmp RUST_BACKTRACE=0 cargo nextest run -j 4 -E 'test(/schema_version|schema_newer|newer_atelier|newer/)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1099
SHA-256: 36b355b8948f8ba7ed827dfcedee18aabf7c93b72123d8ac722c74fa897be6f6
Test summary: Summary [   0.265s] 6 tests run: 6 passed, 941 skipped
