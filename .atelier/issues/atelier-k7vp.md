---
created_at: "2026-11-05T04:40:10.732898478+00:00"
id: "atelier-k7vp"
issue_type: "bug"
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
title: "Credit a criterion to work whose Done when names it"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

Results credits S1, S3 and S4 only to the validator atelier-g6a1, though lnwf, 7v9q and 5tl2 each say 'Delivers Sx' in their Done when; the rule looks only at the Goal's first sentence and the criterion text.

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown debrief and the site's debrief page. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
