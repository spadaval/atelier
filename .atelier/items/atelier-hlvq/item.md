---
schema: atelier.item
schema_version: 3
id: atelier-hlvq
title: merge judges CI on the commit before the one just pushed, and refuses a fix as failed
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T09:13:19.157692314Z
---

## Goal

On 2026-11-27 `atelier-cpfd`'s implementer pushed a fix to its branch and
ran `atelier merge` at once. It refused with "CI failed ... on 8ac4286",
the head before the fix, whose run had failed. The run for the new head
started on GitHub about 20 seconds later, and a second `merge` waited for
it and passed. `AGENTS.md` says `merge` waits for CI on the branch's
latest code. Here it judged an older commit, and told an implementer its
fix had failed.

## Done when

- [ ] `merge` judges only a CI run of the branch's head commit, and where
  the forge has none yet it waits for one, as it waits for a run to
  finish.
- [ ] A test covers a head pushed after a failed run of the commit before
  it.

## Scope

How `atelier merge` picks the CI run it judges, in `crates/` and its
tests. Filed during the freeze on Atelier's work on itself that follows
`atelier-oqsz`; it waits until a real mission meets it again or the human
asks.
