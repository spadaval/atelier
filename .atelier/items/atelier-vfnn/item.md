---
schema: atelier.item
schema_version: 3
id: atelier-vfnn
title: scripts/ci-jobs.sh skips every CI job when its git diff fails
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T07:51:21.402155599Z
---

## Goal

`scripts/ci-jobs.sh` fails open. When its `git diff` fails, for example on a
head that does not resolve, it prints every job as `false` and exits 0, so
CI would skip every job and conclude success. CI always passes `HEAD`, so
this has not happened, and it predates `atelier-1n49`, whose implementer
found it by passing a bad head.

## Done when

- [ ] When the diff between base and head cannot be read, `scripts/ci-jobs.sh` either runs every job or exits non-zero; it never prints every job as `false`.
- [ ] `scripts/ci-jobs.test.sh` covers a head that does not resolve.

## Scope

In: `scripts/ci-jobs.sh`, `scripts/ci-jobs.test.sh`.
