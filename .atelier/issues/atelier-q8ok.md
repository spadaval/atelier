---
created_at: "2026-10-15T22:08:58.975010341+00:00"
id: "atelier-q8ok"
issue_type: "feature"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ox63"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Mark work that only a human can do"
updated_at: "2026-10-15T22:08:58.975010341+00:00"
---

## Goal

Records have no owner or actor. Work that needs the human (at the PC, in VR, an account approval, a judgement call) looks like any other ready item, so an agent can pick it up and stall.

## Done when

A record can say it needs the human (a field or a reserved label with tool support), `work ready` lists those items separately or hides them with a flag, and `prime` mentions how many are waiting. Documented in the agent guide.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). astromech uses a `needs-cmdr` label as a stand-in (voice A/B test, eval clips, the galaxy-map paste spike, the signing decision).
