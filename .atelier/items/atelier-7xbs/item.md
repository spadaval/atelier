---
schema: atelier.item
schema_version: 3
id: atelier-7xbs
title: Stop a trailing blank line in an item's record from failing CI
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T04:10:35.902977411Z
---

## Goal

A blank line at the end of an item's record fails CI's `git diff --check`,
and each time it has cost a review round: on atelier-ogpg, atelier-knx8,
atelier-9f7b, atelier-s63j and atelier-wlyn, by a survey of reviews on
2026-11-18, and on atelier-b9o6 the same day. The agent wrote a Report, the
record ended in a blank line, nothing local said so, and CI refused the
branch after its review. It is a fault of the tool, not of five authors.

## Done when

- [ ] A record that ends in a blank line never reaches CI as a failure: the
  command that writes or validates it removes the line, or the pre-commit
  hook refuses the commit and names the file and the fix.
- [ ] A test covers a Report written with a trailing blank line.

## Scope

Where records are written and validated in `crates/`, and the pre-commit
hook.
