---
schema: atelier.item
schema_version: 3
id: atelier-7ci6
title: Refuse a merge whose merge check failed unless the reason was given for that failure, and say plainly when one is passed
status: open
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.427676472Z
---

## Goal

`ATELIER_SKIP="<reason>"` passes every refusal a command meets. On the
Hub's `v2`, merges `122403ea` and `acf8a7f4` were run with a reason
written for "you are not this item's implementer", and the same reason
carried them past `npm exec -- vitest run` failing. The trailer says so in
one clause, `passes (it failed: exit 1)`, of a line shaped like the two
expected ones, and both merge commits list that check under "These merge
checks passed on this tree". A failed merge check
should stop a merge unless the caller means to pass that failure, and the
record should say what happened.

## Done when

- [ ] A test replays those merges: a reason that passes another refusal,
  and a merge check that fails. The merge is refused, nothing is merged,
  and the refusal names the check and how it failed.
- [ ] A lead can still merge past a failed check on purpose, as
  `roles/lead.md` allows. When it does, the last line `merge` prints says
  which check failed and that the merge went ahead.
- [ ] That merge's commit lists the check as failed, not among those that
  passed, and `atelier show <mission>` and `atelier log` show it as they
  show every rule passed.
- [ ] `docs/design.md` and `roles/lead.md` say how a failed merge check is
  passed.

## Scope

`atelier merge` and how a skip meets a merge check, in `crates/` and their
tests; `docs/design.md` and `skills/agent-factory/roles/lead.md`. Not the
skip for any other refusal, and not what a refusal offers: the Hub's lead
also found that every refusal ends by offering the skip, which stays as it
is here.
