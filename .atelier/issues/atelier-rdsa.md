---
created_at: "2026-11-05T05:17:15.686356292+00:00"
id: "atelier-rdsa"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:56.381372287+00:00"
status: "superseded"
title: "Fix debrief validation links and Results header labels"
updated_at: "2026-11-11T03:55:56.381372287+00:00"
---

## Goal

Two nits from the S7 agent re-review of atelier-vt8q's debrief at 7518f516: each timeline 'How' link goes to the top of Validation, not its run, and the Results header leaves the criterion and mark columns unlabelled.

## Done when

- [ ] A CLI fixture shows each timeline 'How' link pointing at its own validation run, and the Results header labelling every column, on the site's debrief page.

## Context

Dropped in decision atelier-mzpn: the receipt-chip nits (proof chips that show only the receipt ID, chip borders at 1.13:1), since receipts are optional notes that gate nothing (atelier-md0v), and a records-only trivial waiver listed under Changed course, since only code makes an approval stale (atelier-3jdm). vt8q's records become prunable after 2026-11-11, so a fixture is the check. Stands alone until loose end atelier-jdpf reviews a debrief generated under the new lifecycle.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes the site's debrief page, which the What is removed table removes with `serve`; the Desk (mission atelier-z4cr) shows the debrief.
