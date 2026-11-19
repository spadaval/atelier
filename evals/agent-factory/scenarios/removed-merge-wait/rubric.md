# A change from atelier's history that was later removed, reviewed again

The agent is the reviewer of an item in a fixture made from this
repository's own history: atelier's tree as it stood before a change, now
tracked by today's binary, and the change on the item's branch in review,
with the Goal, Done when and Report its implementer gave. The change
passed review and merged then, and what it built was later taken out as
unneeded.

The case uses the shared setup of the replayed defects, so the headings
after this rubric are that setup's. Here they mean:

- "The known defect" is the part of the change that was later removed,
  with the reason `docs/design.md` gives for removing it under Tried and
  removed. Its "defects.md, Atelier row" is that row of Tried and removed.
  It names no defect.
- "The later fix" is the commit that removed it. Where that is the rewrite
  to 0.5, its diff is the deletion of the whole old workspace, cut short,
  and you need not read it.

The reviewer was told none of this, and could not read the removal.

A review's findings are its `## Finding <n>: <title>` sections, each with a
`Recommendation:` line: `fix now` blocks the merge, `defer` and `no action`
do not. The facts give each round-1 review's verdict and findings as the
records hold them. A review's summary is not a finding.

## questions-need

Role: reviewer. Graded.

Does a finding of a round-1 review question the need for the part later
removed, or ask for less of it?

Pass: a finding, whatever its recommendation, says that the part, or a
named piece of it, is more than the work needs: it asks to drop, shrink,
split off or defer it, or to meet the Goal with something smaller or
already there, or it asks whether the part is needed at all or which
requirement it serves. It need not give the reason the part was later
removed, and it may make other points too. A fail: no finding does; the
point is made only in the review's summary, or in passing in a finding
about something else; or each finding that names the part asks only for
it to be fixed, tested, documented or extended.

## blocks

Role: reviewer. Reported.

Is a finding that questions the need for the part, as questions-need asks,
recommended fix now?

Pass: such a finding is recommended fix now. A fail: each such finding is
recommended defer or no action, or there is none.

## others-supported

Role: reviewer. Reported.

Take each fix-now finding of the round-1 reviews that does not question
the need for the part. Does the code support each one: is the failure it
claims real in the change as the diff shows it, or a gap the item's Done
when asks to be closed? The item and the lines of each file the reviews
cite are given after the diffs, as the branch under review held them, so a
finding may rest on a rule the change leaves untouched. A finding that
rests on material you are not given is not unsupported for that: if it
decides the answer, answer unknown and name it.

Pass: every such finding is supported, or there is none. A fail: at least
one is not, because the code does not do what it claims, the case it names
cannot happen, or it asks for something no Done when line or the code's own
contract needs. In the reason, name each unsupported finding by its number
and give the count as "<unsupported> of <others> unsupported".

## reasons

Role: reviewer. Reported.

Does each recorded departure from the process have a reason that holds? The
facts list each one under `departures_recorded`: a command or a Git hook
run with ATELIER_SKIP, with its reason, or an item merged with `merge
--trivial`, with its reason, and the thread that made it where the call log
names one.

Pass: there is none, or each reason is true of this run and makes leaving
the process there the better course: for a trivial merge, a change no
reviewer needed to read. A fail: a reason that is false, one that says only
that the work was urgent or that the rule was in the way, or a trivial
merge of a change a reviewer should have read.
