---
schema: atelier.item
schema_version: 3
id: atelier-2yef
title: skills/agent-factory/references/atelier.md:15-16 vs :66-67, and procedures/orchestrate.md:89 (eb14a31). Declining a sta…
status: dropped
mission: null
labels:
- agent-factory
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T21:50:02.001056830Z
---

## Goal

Deferred from finding atelier-41xn on atelier-l58v @9ea0a19b, by `atelier review defer`.

medium: skills/agent-factory/references/atelier.md:15-16 vs :66-67, and procedures/orchestrate.md:89 (eb14a31). Declining a started item is now a normal ending (a stop signal's discard, a standing reject), but the skill does not say where its record goes or what happens to the branch: atelier.md says decline needs 'no branch, review or merge' while also saying 'a started item's own record changes only on its work branch'. A reader following the second sentence declines on the work branch, archives it, and the base shows the item in_progress forever. All declining eval runs guessed the same way (decline committed on main, branch tagged archive/<id> and deleted, its follow-ups and findings left only on the tag), so no failure was observed. Fix: one sentence in atelier.md saying a declined started item's record is committed on the base and its branch archive-tagged and deleted, and that follow-ups filed on that branch lapse with it.

## Done when

- [ ] Done when was not specified.

## Links

Relationships of the old record that the new format has no field for.

- parent: atelier-rqzu (Deferred review findings on atelier-l58v)
- discovered_from review: atelier-41xn

## Report

Dropped: `references/atelier.md` and `procedures/orchestrate.md` were deleted when the skill was rewritten (atelier-orb2); ending a started item is now `atelier drop` on the base.
