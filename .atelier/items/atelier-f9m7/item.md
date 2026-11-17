---
schema: atelier.item
schema_version: 3
id: atelier-f9m7
title: Say in the extension's rules that code must be worth writing, not that it is expensive
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- b465e6cf-360b-421a-8aff-f30b0629e257
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T19:12:28.072682271Z
---

## Goal

Rule 3 of `t3-extension/AGENTS.md` should make an agent ask what a thing is
for before writing it. Today it reads "Code is expensive. Less is more.",
which names no cost and reads as "write fewer lines". Read that way it
argues against tests, error messages and structure, which rules 1 and 2 and
Sumeet's own instructions want. What he meant, in conversation on
2026-11-17: code is not free and has to give enough value to justify its
existence, as a counterweight to speculative edge cases, tools with no
purpose and scope creep.

## Done when

- [x] Rule 3 is headed "Code is not free." and tells the agent to say what a thing is for before writing it, and not to write it if it cannot.
- [x] It asks a concrete question of a guard, a test and a tool. The test's question is which bug would make it fail.
- [x] It keeps "Prefer the simplest implementation that satisfies the real requirement."
- [x] It says size is not the measure, so the rule does not read as "write less".
- [x] Rules 1 and 2 and every other file are unchanged.

## Scope

In: rule 3 of `t3-extension/AGENTS.md`.

Out: the reviewer and implementer guides in `skills/agent-factory`, and
Sumeet's global instructions. He said to change this rule for now and
revisit the rest later.

## Report

Rule 3 of the extension's rules now asks what a thing is for before it is
written, where it used to say "Code is expensive. Less is more." The wording
is the draft Sumeet agreed in conversation, with one addition: I kept the
old rule's line about the simplest implementation, which the draft had
dropped and which nothing else in the file says.

The change is prose in one file, 11 lines for 5. No code, test or other
document names the old wording (`git grep -i "code is expensive"` finds only
this rule).

### Validation

- `atelier check`: passes.
- `git diff --check`: no output.
