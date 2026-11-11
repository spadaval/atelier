---
schema: atelier.item
schema_version: 3
id: atelier-rdsa
title: Fix debrief validation links and Results header labels
status: draft
mission: null
labels:
- review
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T05:17:15.686356292Z
---

## Goal

Two nits from the S7 agent re-review of atelier-vt8q's debrief at 7518f516: each timeline 'How' link goes to the top of Validation, not its run, and the Results header leaves the criterion and mark columns unlabelled.

## Done when

- [ ] A CLI fixture shows each timeline 'How' link pointing at its own validation run, and the Results header labelling every column, on the site's debrief page.

## Context

Dropped in decision atelier-mzpn: the receipt-chip nits (proof chips that show only the receipt ID, chip borders at 1.13:1), since receipts are optional notes that gate nothing (atelier-md0v), and a records-only trivial waiver listed under Changed course, since only code makes an approval stale (atelier-3jdm). vt8q's records become prunable after 2026-11-11, so a fixture is the check. Stands alone until loose end atelier-jdpf reviews a debrief generated under the new lifecycle.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)
