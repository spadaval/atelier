---
schema: atelier.review
schema_version: 2
id: atelier-a35g
title: finding on atelier-4nsl @23dae74d
target: atelier-4nsl
identity: independent reviewer subagent (claude-opus-5-5), atelier-4nsl
commit: 23dae74d3e56985d9c442e71ce68c29a2dc542b4
created_at: 2026-11-07T11:17:02.394956392Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/architecture/markdown-first-record-store.md:24 still says review events use atelier.review version 1. This branch adds version 3 for records about a branch while issue records stay at version 2, and an older binary refuses the whole repository's records once a version 3 record is present. Fix: state both versions and that consequence in one or two sentences, linking ADR 0028 decision 6.
