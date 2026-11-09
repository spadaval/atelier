---
created_at: "2026-11-09T16:43:55.579646786+00:00"
id: "atelier-w8hs"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:43:53.832014537+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve discovery"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Discovery tests: a repository with two worktrees, one detached and behind, shows once with the base's records; a base named by origin/HEAD; a checkout on a work branch with another policy does not change the base; each error case shows on its own page"
updated_at: "2026-11-09T16:43:55.709500642+00:00"
---

## Summary

Discovery tests: a repository with two worktrees, one detached and behind, shows once with the base's records; a base named by origin/HEAD; a checkout on a work branch with another policy does not change the base; each error case shows on its own page

## Command

```console
cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve discovery
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1322
SHA-256: 240fabbfeb6cb1c7cf3125677665d22154fb46f5016da29f09c9ab065df6814e
Test summary: Summary [   0.092s] 7 tests run: 7 passed, 8 skipped
