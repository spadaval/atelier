---
schema: atelier.item
schema_version: 3
id: atelier-5erp
title: evals/agent-factory/test.sh leaves about 60 temp directories on each run
status: draft
mission: null
labels:
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T10:17:47.654443152Z
---

## Goal

`evals/agent-factory/test.sh` leaves 54 to 60 directories in `TMPDIR` each
time it runs. Their contents match the `tempfile.mkdtemp()` calls with no
cleanup in `scenarios/costly-hardening/files/*/test_flags.py`. `/tmp` on
Sumeet's host has a small quota, and three implementers of `atelier-ww7w`
each had to remove them by hand. `atelier-eqzk` covers the extension's
similar leak only.

## Done when

- [ ] A run of `evals/agent-factory/test.sh` leaves `TMPDIR` holding what it held before.
- [ ] The harness's own tests say so, or `test.sh` fails when a run leaves something.

## Scope

In: `evals/agent-factory/`.
