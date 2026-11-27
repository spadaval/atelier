---
schema: atelier.item
schema_version: 3
id: atelier-vc63
title: Tighten implementer, reviewer, and lead role files
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- cf983ef7-3317-404a-9c82-373f732b9e07
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T18:02:07.468574571Z
---

## Goal

Tighten three small rough edges left in the Agent Factory role files after
`atelier-oqsz`:

1. `roles/implementer.md` does not tell an implementer to ask the lead to
   shrink or rewrite a `Done when` line before building it when it turns out
   much harder than the Goal justifies or no longer fits what landed earlier.
2. `roles/reviewer.md` still ends Size and Verdict with "Report every other
   finding you can support, whatever your verdict", which fights the Severity
   section's rule against nits, and omits mentioning that `Shrink:` also
   applies to an overbuilt `Done when` line (as the review brief already says).
3. `roles/lead.md` has the dispute-scoping sentence ("Where the dispute is
   whether the work belongs in the item...") stranded at the end of the
   failed-merge-check paragraph.

## Done when

- [x] `skills/agent-factory/roles/implementer.md` tells the implementer to ask
  the lead (or starting thread) to shrink or rewrite an item before building a
  `Done when` line that turns out much harder/bigger than the Goal justifies or
  no longer fits what landed earlier.
- [x] `skills/agent-factory/roles/reviewer.md` drops "Report every other
  finding you can support, whatever your verdict", notes that `Shrink:` also
  covers a `Done when` line that exceeds the Outcome, and stays strictly
  shorter in word count than before.
- [x] `skills/agent-factory/roles/lead.md` moves the stranded dispute-scoping
  sentence back into the dispute paragraph.
- [x] `git diff --check` and `atelier check` pass.

## Scope

`skills/agent-factory/roles/implementer.md`,
`skills/agent-factory/roles/reviewer.md`, and
`skills/agent-factory/roles/lead.md`. No changes to `SKILL.md`, `procedures/`,
or Rust code.

## Report

Made the three targeted role-file edits requested by the human:

- `skills/agent-factory/roles/implementer.md`: added one sentence closing the
  loop with `roles/lead.md`'s `Shaping Items as Work Lands` so implementers ask
  the lead to shrink or rewrite oversized/stale `Done when` lines before
  building them.
- `skills/agent-factory/roles/reviewer.md`: removed the contradictory "Report
  every other finding you can support, whatever your verdict" sentence and
  aligned `Shrink:` with the CLI review brief's rule for `Done when` lines that
  exceed the Outcome (word count reduced).
- `skills/agent-factory/roles/lead.md`: restored the stranded dispute-scoping
  sentence to the dispute paragraph above the failed-merge-check paragraph.

### Validation

Verified with `git diff --check`, `atelier check`, and word-count comparison on
`roles/reviewer.md`.
