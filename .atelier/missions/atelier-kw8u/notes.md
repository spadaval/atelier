## 2026-11-18: a fifth passage, the rule against nits

The human asked, by a side chat on the lead's thread, for a rule against
nits to be added as a passage the evals try. atelier-oiwy now tries five
passages where the plan's `passages-tried` criterion names four. The plan
is unchanged: the fifth is more than the criterion asks, on the human's
word, and it is kept or left out by the same rule as the others.

The rule also bears on atelier-s8wm, a plan review that keeps splitting,
which is outside this mission.

## 2026-11-18: more replayed reviews

atelier-1z52 reported that one replayed review takes 1 to 3.5 minutes. Under
the Authority to extend the review cases once that was known, the lead
added atelier-z8rg, which replays the other 12 after-merge defects where they
fit, and made atelier-id3v wait on it so the matrix runs them.

Implementer replays and a replayed mission are not taken up: a confined run
cannot start a second thread, grading would need each fix's regression
test ported to the old code, and the three changes sit on bases 19 hours
apart.

## 2026-11-18: two replay fixes before the matrix

atelier-z8rg replayed nine more defects and filed two deferred findings as
draft items. The lead readied both and made atelier-id3v wait on them,
because each changes what the replayed reviews measure: atelier-s63j takes
old process instructions out of two fixtures, and atelier-6ve6 stops the
judge counting a sound finding as unsupported for want of the file it
cites.

## 2026-11-18: what a Codex thread is told

atelier-aq82 found that a no-skill Codex lead still stops to ask leave to
start a subagent, so its lead cells measure that and nothing else. The
harness tells a Claude Code thread how to start another and tells a Codex
thread nothing. The lead added atelier-69oh to tell both the same, and made
atelier-id3v wait on it.

## 2026-11-18: Codex's own rule on subagents stays, and the matrix gets a third arm

atelier-69oh told a Codex thread how to start another, and one of two
no-skill leads still asked leave. Codex 0.169 adds its own rule when it
offers `spawn_agent`: start no subagent unless the user, an `AGENTS.md` or a
skill asks. A config key would replace the rule. The lead left it in
place, because T3 Code's Codex threads keep it, so the rule is what a Sol
thread on the work host meets. The skill is what asks there.

So a no-skill Sol cell partly measures that rule. To tell the sentence that
lifts it from the rest of the skill, atelier-id3v runs a third arm in
`sol`: a skill that holds only the Starting Another Thread section. The
lead also set the matrix's size: five runs a cell, three for a replayed
review, one matrix on the host at a time.

## 2026-11-18: the human lifted the limit of two eval agents

The plan's Authority says the lead may not raise the limit of two eval
agents alive at once. The human lifted it on 2026-11-18, by a side chat on
the lead's thread, after reading that the matrix would take about a day:
"Why not parallelize?" and then "tell the main thread to go nuts". The
limit was theirs, set on 2026-11-10 for mission atelier-qr5s, and the plan
carried it forward. The plan's text is left as it is and this note is the
record of their word against it.

The lead added atelier-u708, which makes the cap a setting by kind of run: three
at once for runs that build Rust and six for the others. Within one run
nothing changes. atelier-id3v's line "Only one matrix runs on this host at
a time", which the lead added the same day, goes: its implementer is told
to change it on the item's branch and to run the rest of the matrix with
the new caps once atelier-u708 merges, keeping the runs already made.

## 2026-11-18: the human's severity scale replaces the lead's wording

By a second side chat the human gave five additions about review, with a
survey of 143 Sol reviews behind them. One enters this mission: their scale
of critical, important, useful and nit replaces the lead's wording of
atelier-oiwy's fifth passage. The two round-2 cases leave atelier-oiwy,
because what a later round is for goes to the binary's brief.

The other four go to a mission that follows this one, since they change
what this mission's Scope leaves out: the brief the binary prints for a
later round, the implementer's exits from a finding, a passage that asks
which requirement each new part serves, and cases of correct but overbuilt
work with their controls.

## 2026-11-19: replayed reviews filled the disk

A kept replay run holds about 2.7 GB of Rust build output, and the harness
keeps every failing one. atelier-ad0z's matrix and atelier-id3v's `house`
replays filled this host's root disk twice; the T3 Code server runs on it.
At 05:31 the lead deleted the build output of 53 finished runs under
`/var/tmp/af-evals`, 71 GB, leaving 146 GB free, and touched no run of the
last 30 minutes and no result.

atelier-scly, which atelier-ad0z's implementer filed, is readied, and
atelier-oiwy waits on it: that item runs the replayed reviews ten times
over and would fill the disk again.

## 2026-11-19: what the matrix showed, and what the lead did with it

atelier-id3v, 469 graded runs:

- In `sol` the skill takes the lead's cases from 4 of 25 to 25 of 25, and
  a skill cut to its Starting Another Thread section does exactly as well,
  35 of 35. All of the skill's effect on passes there is getting past
  Codex's own rule on subagents. In `house` every lead run passes in both
  arms, and the skill cuts the median time and tokens of each lead case.
- No planted defect reached main, no departure went unrecorded and no lead
  stopped with nothing raised, with the skill before atelier-mtzj or after.
  Of 40 recorded departures the judge found two reasons that did not hold.
- On replayed reviews the skill found the known defect in 12 of 72 against
  17 of 72 without it, and on `touched-claims` in 0 of 6 against 5 of 6.

The lead added atelier-7h71 to confirm that last gap and find the text behind it,
and made atelier-oiwy wait on it. The 18 cases mission atelier-8az0 added
while the matrix ran are not run again on their own: atelier-oiwy's review
arms run the review cases among them.
