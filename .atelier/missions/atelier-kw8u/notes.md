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
