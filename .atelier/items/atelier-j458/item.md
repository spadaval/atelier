---
schema: atelier.item
schema_version: 3
id: atelier-j458
title: Load review history when a record path is not UTF-8
status: draft
mission: null
labels:
- land
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-22T23:23:56.029116881Z
---

## Goal

Since atelier-7b6e, land and doctor load the history of .atelier/issues and .atelier/reviews once (crates/atelier-app/src/path_log.rs:199-200). If any commit holds a non-UTF-8 file name under those directories, the load fails and land and doctor error outright, where the old git log walks kept working. Found in review of atelier-7b6e (finding atelier-i0w4).

## Done when

- [ ] A history containing a non-UTF-8 path under .atelier/issues or .atelier/reviews loads, either keyed as bytes or skipping paths no record ID can match, and land's answers are unchanged. A test with such a path covers it.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-i0w4
