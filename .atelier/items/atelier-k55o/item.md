---
schema: atelier.item
schema_version: 3
id: atelier-k55o
title: Refuse to create work items without a real Goal and Done when
status: draft
mission: atelier-uf97
labels:
- cli-output
- issue-commands
- bug
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.961551559Z
---

## Goal

`atelier issue create "x"` and `issue create "x" --body "Plain goal text"` both succeed and write placeholder sections (`## Done when` holding `Done when was not specified.`, and `Goal was not specified.` when there is no body); checked 2026-11-07. Under atelier-jnrh, `review request` asks only that boxed Done when lines be ticked, so a placeholder Done when has nothing to tick and passes. Refuse the placeholder when the record is written, where it is cheapest to fix.

## Done when

- [ ] `issue create` refuses a work item without a real Goal and Done when (a mission without an Outcome, a decision without a Question; Success criteria stay optional under ADR 0024) and writes nothing; the error names the missing section and shows an example `--body`. If `backlog` items are exempt, the error or a warning says so.
- [ ] CLI tests cover create with no body, with plain text only, and with full sections.

## Context

The refusal can come from atelier-ilba's write guard once atelier-w4ef makes the whole-repository `check` apply the placeholder rule, or from `create` itself, as `issue update --body` checks its sections today (crates/atelier-cli/src/commands/issue.rs). Dropped in decision atelier-mzpn: refusing `atelier start` on a placeholder record. Also dropped: counting a Done when with no boxed lines as unticked at `review request`, which atelier-jnrh considered and rejected (decision note: a prose Done when has nothing to tick, and requiring boxes everywhere would be a new rule).

## Unknowns

Whether backlog items may be filed with placeholders (capture now, refine later); atelier-w4ef follows the same answer for `check`.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-w4ef (Make check and check <id> apply one rule set)
