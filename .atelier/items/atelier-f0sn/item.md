---
schema: atelier.item
schema_version: 3
id: atelier-f0sn
title: Put what waits on the reader first in status, one line per item, with a line of review pace for a mission's lead
status: open
mission: atelier-oqsz
labels: []
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.292565379Z
---

## Goal

A mission's lead on 0.7.0 could not read its own `status`: it printed the
backlog first and the missions led last, reached 17,870 bytes, and the
harness showed the first 2 KB. Of the largest output 51% was unticked Done
when lines of items other threads were building and 12% thread ids, and
much of the rest blocked-by chains. The 41 saved outputs are under
`~/.claude/projects/-home-supadava--t3-worktrees-industrial-experience-hub-v2/7041651b-a459-48d3-8e59-3d0651f2296a/tool-results/hook-*-stdout.txt`.

Nothing told that lead what its reviews cost either. Give `status` an
order and a size a reader can use, and give a lead one line of review pace
for each mission it leads.

## Done when

- [ ] Whatever the reader is, what waits on that thread prints before
  anything that does not, and the backlog prints after both.
- [ ] A test builds a twelve-item mission with items building, in review
  and blocked, and its lead's `status` is under 2 KB.
- [ ] A test builds the state behind the Hub's largest saved output (three
  missions led, eight items held by other threads) and its lead's `status`
  is under 4 KB.
- [ ] A lead's `status` holds one line of review pace for each mission it
  leads: rounds per item, the time a round took, and the new findings in
  rounds after the first. The Report gives the line for `hub-v2-bg14`
  beside a count made by hand from its review records.
- [ ] What `status` no longer prints is still one command away, and
  `status` names that command.
- [ ] `docs/product.md` and `docs/design.md` say what `status` shows and
  in what order. `evals/agent-factory/test.sh` passes.

## Scope

What `status` prints, in `crates/` and their tests, and the two documents.
Not `atelier show`, not the Desk, no new command and no change to a
record's form. `atelier-5asm` fixes three lines `status` prints that are
refused; leave them to it.
