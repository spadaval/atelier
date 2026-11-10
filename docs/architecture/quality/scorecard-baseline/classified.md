# Classified scorecard measures, before the redesign

Before the redesign, 11 of 100 refusals stopped a real mistake, and 46 of 100
fix-now findings were product defects. The owner stepped into a closed
mission's work between 0 and 17 times; most of what he typed in those threads
was about other work. Each figure is one rater's judgment under
[rubric.md](rubric.md), with a second rater's agreement below.

Window: 2026-10-14 to the cutoff, 2026-11-10T05:14:58Z, read from the frozen
inputs named in [manifest.json](manifest.json). Every sample was drawn by
`scripts/scorecard/draw_samples.py` with seed `20261110`; the unlabelled
samples, the first rater's labels and the second rater's are in
[samples/](samples/). The first rater was Claude Opus 5.5, the implementer of
atelier-3kr7.

## Interventions per mission

21 missions closed in the window: 17 in atelier, 3 in demiurge and 1 in
life-on-rails. Astromech and cosplay-studio closed none. Five of the 21
(atelier-2mky, gg6b, l63c, ll3p and nn9y) were superseded straight from
draft, so they never started and have no window. The other 16 each had at
least one thread tied to them.

Of 1,773 T3 threads in the five repositories, 534 were tied to a closed
mission by branch, title or first message, and 27 more as threads above them
in T3's lineage, where the owner talks to the orchestrator. 78 of those were
tied to more than one mission. The remaining 1,212 belong to no closed
mission. Inside the missions' windows those threads hold 142 messages the
owner typed, all classified here, and 1,439 agent-written ones, left out. For
the 8 messages that could belong to more than one mission, the label file
names the mission each was about.

| Mission | Repository | Closed as | Typed | Status | Unsticking | Direction | Interventions | Approvals | Other |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| atelier-1gb3 | atelier | done | 14 | 0 | 3 | 0 | **3** | 0 | 11 |
| atelier-1w8c | atelier | superseded | 3 | 0 | 0 | 0 | **0** | 0 | 3 |
| atelier-fl12 | atelier | done | 3 | 2 | 0 | 1 | **3** | 0 | 0 |
| atelier-k8h8 | atelier | done | 13 | 2 | 0 | 3 | **5** | 6 | 2 |
| atelier-kgcr | atelier | done | 1 | 0 | 0 | 1 | **1** | 0 | 0 |
| atelier-mmpv | atelier | done | 9 | 1 | 0 | 1 | **2** | 1 | 6 |
| atelier-n4mj | atelier | done | 9 | 0 | 0 | 2 | **2** | 1 | 6 |
| atelier-o3lm | atelier | done | 0 | 0 | 0 | 0 | **0** | 0 | 0 |
| atelier-r6bt | atelier | done | 15 | 1 | 0 | 6 | **7** | 2 | 6 |
| atelier-rc8j | atelier | done | 0 | 0 | 0 | 0 | **0** | 0 | 0 |
| atelier-vt8q | atelier | done | 10 | 3 | 0 | 2 | **5** | 3 | 2 |
| atelier-yfqu | atelier | done | 8 | 2 | 0 | 1 | **3** | 2 | 3 |
| worldcrafter-gwx3 | demiurge | done | 0 | 0 | 0 | 0 | **0** | 0 | 0 |
| worldcrafter-p3k7 | demiurge | superseded | 0 | 0 | 0 | 0 | **0** | 0 | 0 |
| worldcrafter-z9qs | demiurge | superseded | 30 | 2 | 1 | 14 | **17** | 3 | 10 |
| lor-uj0r | life-on-rails | done | 27 | 2 | 2 | 8 | **12** | 1 | 14 |
| **Total** | | | **142** | **15** | **6** | **39** | **60** | **19** | **63** |

Across the 16 missions that started, interventions range from 0 to 17, with a
median of 2.5. Correcting direction is two thirds of them (39 of 60). The two
largest counts are missions the owner ran as a conversation: worldcrafter-z9qs
was replanned with him for most of its week, and lor-uj0r's launcher design
was argued out in its kick-off thread. The atelier missions of November
(n4mj, mmpv, 1gb3, fl12, yfqu) had 2 or 3 each. 63 typed messages, 44%, were
about other work: in the thread above an orchestrator, the owner goes on
planning the next mission while this one runs.

## Gate precision

430 atelier calls in real use printed a refusal. The sample is the first 130
of them in seeded order; entries 1 to 105 were judged, and 5 of those (3, 12,
13, 37 and 63) were excluded as not refusals of real work, which leaves 100.

| Repository | Judged | Stopped a mistake |
| --- | --- | --- |
| atelier | 54 | 4 |
| astromech | 28 | 3 |
| life-on-rails | 12 | 3 |
| demiurge | 6 | 1 |
| cosplay-studio | 0 | 0 |
| **Total** | **100** | **11** |

The 11 (entries 2, 14, 26, 40, 51, 58, 62, 80, 85, 96 and 101) are five real
merge conflicts, two follow-up records that pointed at the wrong item, a
mistyped commit hash, two non-trivial commits on a base without review, and a
conflicted merge that needed its resolution reviewed. The 89 that were
friction are mostly an ID of the wrong record kind or a record on another
branch, a base that moved with documents only, missing flags, reviewers asking
for a brief, and gate bugs that the agent then debugged.

## Review yield

Fix-now findings recorded in the window: atelier 478, astromech 372,
life-on-rails 76, demiurge 72 and cosplay-studio 58. Each repository has more
than 20, so the sample is 20 from each.

| Category | atelier | astromech | demiurge | life-on-rails | cosplay-studio | Total |
| --- | --- | --- | --- | --- | --- | --- |
| User bug | 6 | 4 | 4 | 6 | 12 | 32 |
| Tooling bug | 2 | 2 | 4 | 0 | 6 | 14 |
| Test gap | 1 | 4 | 0 | 0 | 0 | 5 |
| Wrong statement | 3 | 5 | 3 | 0 | 1 | 12 |
| Wording nit | 0 | 3 | 0 | 0 | 0 | 3 |
| Tracker record or process | 8 | 2 | 9 | 14 | 1 | 34 |
| Overbuilt | 0 | 0 | 0 | 0 | 0 | 0 |
| **Product defects** | **8 of 20** | **6 of 20** | **8 of 20** | **6 of 20** | **18 of 20** | **46 of 100** |

Product defects are user bugs plus tooling bugs, as in the assessment's 39%.
Planning findings dominate where review mostly reads plans: 14 of
life-on-rails' 20 and 9 of demiurge's.

## Agreement

The second rater was Grok 4.6 (high reasoning), started by the first rater
with only `rubric.md` and the unlabelled sample files, after the first
rater's labels were committed. It labelled entries 1 to 30 of each sample
(`<measure>.second.json`). Neither rater's labels were changed afterwards.

| Measure | Same label | Agreement |
| --- | --- | --- |
| Interventions | 22 of 30 | 73% |
| Gate precision | 25 of 30 | 83% |
| Review yield | 29 of 30 | 97% |

Every disagreement, as entry number, its identifier, then the first rater's
label and the second's:

- Interventions (message IDs in `interventions.json`): 2 (atelier-k8h8,
  `d9254461`) other / correcting_direction; 9 (worldcrafter-z9qs, `9c55ef1c`)
  unsticking / correcting_direction; 11 (worldcrafter-z9qs, `1bb38162`)
  other / correcting_direction; 13 (lor-uj0r, `2d3d1be1`) other / approval;
  14 (atelier-mmpv, `70f37e37`) other / correcting_direction; 15
  (atelier-n4mj, `275df3bf`) other / status_question; 22 (worldcrafter-z9qs,
  `e2e8a97e`) other / correcting_direction; 30 (atelier-k8h8, `8a5f14cb`)
  other / status_question. The one multi-mission entry both labelled as an
  intervention (5) was attributed to the same mission by both.
- Gate precision (turn items in `refusals.json`): 4 (astromech) friction /
  stopped_mistake; 11 (atelier) friction / stopped_mistake; 18
  (life-on-rails) friction / stopped_mistake; 26 (astromech) stopped_mistake
  / friction; 28 (atelier) friction / stopped_mistake.
- Review yield: 3 (cosplay-studio-wo4m) wrong_statement / user_bug.

In seven of the eight intervention disagreements the first rater said
`other`, and in six of those the second saw an intervention: the raters split
on whether a message in the thread above an orchestrator was about the
mission at all. If the second rater's labels held across the sample, the
intervention counts above would be higher. The rubric's first question is
the unclear one; a later run should tie messages to missions more narrowly
or give that question sharper tests. Four of the five refusal disagreements
are the second rater finding a real problem where the first saw a state the
agent expected or a change the policy exempts: the rubric's line between "a
refusal on a real problem" and "a state the agent already knew" needs a
sharper rule.

## Limits and where the rubric was hardest

- **One rater.** Every figure above is the first rater's labels; the second
  rater checks 30 entries per measure, not the whole sample.
- **Tying threads to missions** uses IDs and lineage. A thread that worked on
  a mission without naming it is missed, and a long-lived thread above an
  orchestrator is attributed to every mission open below it; the `other`
  label absorbs what that attribution gets wrong, but cannot recover what the
  first misses. A mission's start is when its record left draft, so planning
  done before then is outside the count, and replanning after it is inside.
- **Approval or unsticking.** An agent that stops to ask "shall I start the
  next item?" has paused work, yet the owner's "yes" is labelled `approval`.
  Counting those as unsticking would add 19 interventions.
- **Policy at the time.** Whether a document change needed review differed
  between repositories and over the window, which decides several
  `land_base_unreviewed` refusals.
- **Planning findings.** A finding on a plan that describes code that would
  be wrong if built is `process_nit` by rule, so review yield undercounts what
  planning review prevents.
- **Claims are not verified.** Findings are labelled by what they claim, not
  by whether the claim held.
- **Two known label faults, left as they are.** Labels and the rubric are
  frozen once the second rater has run, because changing either would spoil
  the agreement figures. Review found two faults:
  - Entry 38 (atelier-l8sl) is labelled `tooling_bug`, but its CI trigger
    did not exist yet at the reviewed commit. By the rubric's planning rule it
    is `process_nit`. With it relabelled, review yield is 45 of 100, and
    atelier's is 7 of 20. A reread of the other user-bug and tooling-bug
    labels found no other finding about code that did not yet exist.
  - The planning rule says a fault in a "decision record" is `process_nit`,
    yet the rubric's `wrong_statement` example atelier-yzhp, and entries 8
    and 67, are faults in ADRs. The rubric means tracker decision items, not
    ADRs, but does not say so. Either reading leaves the product-defect count
    unchanged. A later run should say that ADRs and other documents take
    `wrong_statement`, and only tracker records and mission strategies take
    `process_nit`.

## Recomputing

`python3 scripts/scorecard/draw_samples.py --out <directory>` redraws the
three samples; `cmp` against `samples/` shows they match. The tables above
come from joining each `<measure>.labels.json` with its sample file by `n`.
The agreement, from `samples/`:

```sh
python3 -c 'import json; [print(m, sum(a["label"] == b["label"] for a, b in zip(json.load(open(m + ".labels.json"))["labels"], json.load(open(m + ".second.json"))["labels"])), "of 30") for m in ("interventions", "refusals", "findings")]'
```
