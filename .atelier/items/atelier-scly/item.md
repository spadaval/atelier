---
schema: atelier.item
schema_version: 3
id: atelier-scly
title: Delete a replayed review's build output once it is graded, so a matrix cannot fill the disk
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A90b6e89e-5f0b-4a65-9258-88eb6abe0d8b%3Adelegate-task%3A653e24dd-a76a-40f9-ab1c-4f07a3b8c3f9
ready:
  by: 'the lead of atelier-kw8u: replayed reviews filled the host''s disk twice on 2026-11-19'
  at: 2026-11-19T10:32:10.603919033Z
trivial: null
caused_by: null
created_at: 2026-11-19T09:12:32.354952666Z
---

## Goal

A replayed review builds the old workspace with cargo in its own `target/`,
about 2.5G, and the harness keeps a run's work directory whenever the run
did not pass. Most replays do not pass, so a matrix keeps most of its
builds. On 2026-11-19 atelier-id3v's runs held 132G in `/var/tmp/af-evals`
and filled this host's disk, which the T3 Code server shares. Six house
replays ran out of space mid-review and had to be set aside and made again,
and the harness crashed writing `results.jsonl`, leaving a truncated line.

## Done when

- [ ] Once a replayed review is graded, its `target/` is deleted, whether it
  passed or not; the rest of the work directory is kept as now. So is
  build output the reviewer made elsewhere under the work directory, such
  as a `target/` in a checkout under its `tmp/`.
- [ ] A test with a stand-in agent shows a failing replay's work directory
  kept without its `target/`.

## Scope

`evals/agent-factory/harness/` and the README's line on what a run keeps.
