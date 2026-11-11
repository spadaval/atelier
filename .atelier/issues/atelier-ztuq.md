---
created_at: "2026-11-05T04:40:11.289744306+00:00"
id: "atelier-ztuq"
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
status: "backlog"
title: "Tidy small debrief inaccuracies from the S7 review"
updated_at: "2026-11-07T21:03:33.214129009+00:00"
---

## Goal

Nits from the S7 review of atelier-vt8q's debrief at c73d55e8: the mission chip card's Outcome shows a raw '### Outcome'; the verdict omits zero counts; no year appears anywhere; each 'Delivered after this validation' list includes the validation item itself; gddg (split out by a `fix_now` finding) shows as plainly Added.

## Done when

- [ ] A CLI fixture in a scratch repository shows each nit fixed in both the Markdown debrief and the site's debrief page.

## Context

Decision atelier-mzpn held the 'Delivered after this validation' nit until validation settled; atelier-8bh4 has since made a validation an on-base item with no branch or review round of its own. vt8q's records become prunable after 2026-11-11, so a fixture is the check. Stands alone until loose end atelier-jdpf reviews a debrief generated under the new lifecycle.
