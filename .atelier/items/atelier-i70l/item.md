---
schema: atelier.item
schema_version: 3
id: atelier-i70l
title: Drop the old record paths from scripts/ci-jobs.sh
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T16:48:44.538227539Z
---

## Goal

`scripts/ci-jobs.sh` decides which CI jobs a change needs. Its rule that lets a records-only change skip the Rust job still lists the old tracker's paths (`.atelier/issues/`, `.atelier/reviews/`, `.atelier/evidence/`), which no repository holds since the cutover of 2026-11-11. Remove them, so the rule names only what exists.

## Done when

- [ ] `scripts/ci-jobs.sh` and `scripts/ci-jobs.test.sh` name no old record path, and the test passes.
- [ ] A change that touches only `.atelier/items/` or `.atelier/missions/` still skips the Rust job, shown by the test.

## Context

Found by atelier-2mlh, the first item under the new tool, whose search for leftovers of the old fallback matched these lines. Small; start it whenever CI's job rule is next touched.
