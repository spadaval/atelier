---
schema: atelier.item
schema_version: 3
id: atelier-k7vp
title: Credit a criterion to work whose Done when names it
status: dropped
mission: null
labels:
- review
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T04:40:10.732898478Z
---

## Goal

Results credits S1, S3 and S4 only to the validator atelier-g6a1, though lnwf, 7v9q and 5tl2 each say 'Delivers Sx' in their Done when; the rule looks only at the Goal's first sentence and the criterion text.

## Done when

- [ ] A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown debrief and the site's debrief page. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)

## Report

Dropped: The `debrief` command was deleted in 0.5.
