---
schema: atelier.item
schema_version: 3
id: atelier-sxcj
title: Accept --body-file in issue create and issue update
status: dropped
mission: atelier-uf97
labels:
- issue-commands
- retro-2026-10-17
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T19:00:36.340695076Z
---

## Goal

Agents write bodies to temp files and pass `--body "$(cat ...)"`, or edit records and strategy docs with Python string replacement (e592938d did it three times).

## Done when

- [ ] `issue create` and `issue update` accept `--body-file <path>` (and `-` for stdin), validated like `--body`. Tests cover file and stdin.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-cj54 (replace one body section), atelier-rz2l (import a drafted record).

## Report

Dropped: `issue create` and `update --body` were removed in 0.5; bodies are edited in their files.
