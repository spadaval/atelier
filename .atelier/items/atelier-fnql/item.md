---
schema: atelier.item
schema_version: 3
id: atelier-fnql
title: Let a review disposition record a finding as fixed, so a deferred finding fixed in passing needs no follow-up
status: dropped
mission: null
labels:
- review
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T22:25:20.461573649Z
---

## Goal

Sumeet directed on 2026-11-08 that a deferred finding fixed in passing "stays deferred, or is recorded as fixed", and that `fix_now` never marks a fix. Today `review disposition --decision` offers only `fix_now` (an older form), `defer`, `no_action` and `wont_fix`, so the skill records such a finding as deferred and moves its follow-up under the item so the merge marks it done. A `fixed` decision naming the fixing commit would say what happened in one record.

## Done when

- [ ] `review disposition --decision fixed --resolution-commit <sha>` settles a finding recommended defer, and `merge --check` accepts it.
- [ ] `review disposition --help` describes it, and the skill's orchestrate.md uses it in place of moving the follow-up.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-3qze

## Report

Dropped: Done by the 0.5 record format: a reviewer marks a carried finding `Now: fixed`, and `review disposition` is gone.
