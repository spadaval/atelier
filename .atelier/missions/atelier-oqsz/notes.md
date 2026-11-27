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
| 2 | Sol approved, Grok approved | 0 | 0 |

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

**Round 2**, of `427a0be`, a plan of 840 words. Submitted 07:33 UTC, and
both verdicts were pushed by 07:38: 5 minutes. Both reviewers were fresh
threads on the same two models. Each marked the carried findings fixed and
raised nothing new. The plan merged at 07:40 as `4ba0285`.

**The result.** Two rounds, 17 minutes from the first submit to the last
verdict, four findings, all about outcomes. `atelier-ww7w`: five rounds,
80 minutes, 45 findings.

**What it does and does not show**

- A plan written as outcomes drew no finding about how a mechanism would
  behave, under the old reviewer text. That points at the level a plan is
  written at.
- The two findings that blocked were both in `atelier-qut2`, the one item
  that named a procedure: a plan to review again, and a file not to
  change. Where the plan was specific it drew findings.
- It is one review. This plan is a third the size of `atelier-ww7w`'s, on
  another subject, and Sol stood where Opus did. `atelier-qut2`'s second
  review of `atelier-ww7w`'s plan holds the plan and the models fixed and
  changes the reviewer's text, which this review could not do.
- The marking is the planner's, of a plan the planner wrote.

**Filed from the planning, outside the mission:** `atelier-9f1j`
(`submit` orders its runner to start a reviewer), `atelier-nc7t` (a branch
with two merge bases is refused for reasons that name something else) and
`atelier-jahm` (`merge` says a plan merges with no review).

## 2026-11-27: the planner leads, and the order the items start in

Sumeet, in the planner's thread: "Yeah, close it. Then, lead the mission."
The "it" is `atelier-oiwy`, which he ruled closed on the evidence it has,
with no further runs. That ruling went to the lead of `atelier-kw8u`, whose
item it is.

The lead starts `atelier-f0sn` and `atelier-7ci6` first. This host allows
two heavy builds at once (`~/.cargo/config.toml`), and each item's reviewer
builds too, so `atelier-hota` starts when one of the two merges.

`atelier-qut2` is held until `atelier-oiwy` is merged or dropped. Then it
may change `roles/reviewer.md` and all of `SKILL.md` itself, and the item
its Scope would have filed for them is not needed. If `atelier-oiwy` is
still open when the three other items are merged, `atelier-qut2` starts
with its Scope as written.
