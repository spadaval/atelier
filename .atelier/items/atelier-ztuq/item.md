---
schema: atelier.item
schema_version: 3
id: atelier-ztuq
title: Tidy small debrief inaccuracies from the S7 review
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
created_at: 2026-11-05T04:40:11.289744306Z
---

## Goal

Nits from the S7 review of atelier-vt8q's debrief at c73d55e8: the mission chip card's Outcome shows a raw '### Outcome'; the verdict omits zero counts; no year appears anywhere; each 'Delivered after this validation' list includes the validation item itself; gddg (split out by a `fix_now` finding) shows as plainly Added.

## Done when

- [ ] A CLI fixture in a scratch repository shows each nit fixed in both the Markdown debrief and the site's debrief page.

## Context

Decision atelier-mzpn held the 'Delivered after this validation' nit until validation settled; atelier-8bh4 has since made a validation an on-base item with no branch or review round of its own. vt8q's records become prunable after 2026-11-11, so a fixture is the check. Stands alone until loose end atelier-jdpf reviews a debrief generated under the new lifecycle.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)

## Report

Dropped: The `debrief` command was deleted in 0.5.
