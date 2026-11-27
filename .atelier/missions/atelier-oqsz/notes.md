# Notes on atelier-oqsz

## The plan review, as a measurement

This plan was written as outcomes on about one page and reviewed under the
rules and the skill as they stood on 2026-11-27, with no rule passed. The
planner records each round here once both verdicts are pushed. Each new
finding is marked by the planner, and listed so the marking can be checked:

- **outcome:** about what will be true, whether it is what the human asked
  for, whether a criterion can be checked, the items, their order or what
  is out, or a contradiction in the plan.
- **mechanism:** about how something will be built, or how it will behave
  in a case the plan does not state.

The comparison is `atelier-ww7w`'s plan review of 2026-11-22: a plan of
2,400 words, five rounds in 80 minutes, 45 new findings by the count of
the Hub lead's side chat, most about mechanisms.

| Round | Verdicts | New findings: outcome | New findings: mechanism |
| --- | --- | --- | --- |
| 1 | Sol approved, Grok asked for changes | 4 | 0 |

**Reviewers.** `gpt-6.1-sol` at `xhigh` and `grok-4.6` at `high`, each
given only the task line `submit` prints. `atelier-ww7w`'s were
`claude-opus-5-5` and `grok-4.6`, and Opus raised most of its findings by
the side chat's count, so the reviewer differs as well as the plan.

**Round 1**, of `d1de63b`, a plan of 790 words. Submitted 07:21 UTC, and
both verdicts were pushed by 07:32: 11 minutes.

- `1-8ja5.1` (Sol, defer), outcome: the hosts criterion asks for a new
  version and no item owns the bump.
- `1-8ja5.2` (Sol, defer), outcome: `atelier-qut2` reads one review as
  proof of a cause, where the two plans differ in subject, size and
  reviewers.
- `1-xnyq.1` (Grok, fix now), outcome: skill-measured names no plan to
  review again, so two implementers could tick it having measured
  different things.
- `1-xnyq.2` (Grok, fix now), outcome: `atelier-qut2`'s Done when and
  Scope contradict each other. A plan's reviewer reads `roles/reviewer.md`
  and the brief in `crates/`, and the item could change neither. The
  finding shows this from the code and offers two fixes that name files,
  but what it objects to is an item that cannot meet its own criterion.

No finding asked how `status`, `merge` or `log` should behave in a case
the plan does not state. All four were fixed in `4e34916`, `2c24ed6` and
`1c19859`, the two deferred ones included, because each took a sentence.
