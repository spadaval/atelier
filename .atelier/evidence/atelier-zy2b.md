---
created_at: "2026-11-11T01:22:31.295707700+00:00"
id: "atelier-zy2b"
evidence_type: "test"
commit: "05a508ab42a3033c702fa243c828a5811a13d82d"
result: "pass"
captured_at: "2026-11-11T01:22:18.744801051+00:00"
command: "bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z \"$(git diff --name-only master... -- crates/)\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t2jo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t2jo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z \"$(git diff --name-only master... -- crates/)\"'"
updated_at: "2026-11-11T01:22:31.449153642+00:00"
---

## Summary

bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z "$(git diff --name-only master... -- crates/)"'

## Command

```console
bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --workspace --all-targets -- -D warnings && cargo nextest run -j 4 --locked && cd .. && git diff --check master... && test -z "$(git diff --name-only master... -- crates/)"'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 26719
SHA-256: 1f9aba150df6c03d6529088814f0eb97cf48136a3e18a13674c3205dcb04791a
Test summary: Summary [  11.781s] 223 tests run: 223 passed, 0 skipped
