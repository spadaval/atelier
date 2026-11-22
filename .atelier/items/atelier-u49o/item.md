---
schema: atelier.item
schema_version: 3
id: atelier-u49o
title: Say in the reviewer's task line which checkout to run atelier review in
status: dropped
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T01:53:11.985194613Z
---

## Goal

`atelier submit` prints the reviewer's task as "Run atelier review <id> and
follow what it prints." A reviewer started from the implementer's thread
starts in that thread's directory, which is often the main checkout, and
`review` there is refused: it runs in the checkout on the item's branch. In
atelier-rt3o's smoke run of 2026-11-17 (`gpt-6.2-sol`, Codex), the reviewer
subagent hit this in three of the seven thread scenarios (next-loop,
next-settle-within-authority, next-refused-push), and each spent the run's
one allowed failed call on it before following the refusal. Runs are in
`~/.cache/af-evals/runs/rt3o-smoke/`.

## Done when

- [ ] The task line `submit` prints for the reviewer names the checkout to
  run `atelier review` in, or `review` run elsewhere in the clone reviews in
  the item's checkout; a test covers a reviewer started in the main
  checkout.

## Scope

## Report

Dropped: Covered by atelier-zc9b in mission atelier-ww7w (agreed with Sumeet on 2026-11-22)
