---
schema: atelier.item
schema_version: 3
id: atelier-9f1j
title: submit orders whoever runs it to start a reviewer, against a lead that runs the review queue
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:40:12.087427950Z
---

## Goal

`atelier submit` ends by telling whoever ran it to start the reviewers:
"starting it is your job ... nobody else will start it. So start one for
each task yourself now, without waiting to be asked." Where a mission's
lead runs the review queue and has told its implementers not to start
reviewers, the tool and the lead give opposite orders. The lead of
`hub-v2-bg14` reported on 2026-11-27 that nearly every implementer there
stopped to explain why it was not doing what `submit` said. That count is
the lead's report, not checked. Left out of mission `atelier-oqsz` to keep
it to the four changes the human agreed.

## Done when

- [ ] What `submit` prints about starting a reviewer does not contradict a
  lead that starts the reviewers itself, and an implementer with no lead
  still learns that the review will not start unless it starts one.
- [ ] A test covers both cases.

## Scope

What `submit` and `status` print about starting a reviewer, in `crates/`
and their tests, and the skill where it says the same. `atelier-5zmb` is
the neighbouring case of a Codex thread with no skill; read it first.
